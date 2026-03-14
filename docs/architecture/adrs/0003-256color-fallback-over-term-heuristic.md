# ADR-0003: Use 256-Color Indexed Fallback Instead of TERM-Based Truecolor Heuristic

## Status
Accepted

## Context

On Raspberry Pi OS, `COLORTERM` is not set and `TERM=xterm-256color`. The original
`supports_truecolor()` function in `src/theme/palette.rs` checked, in order:

1. `COLORTERM` == "truecolor" or "24bit"
2. `WT_SESSION` (Windows Terminal)
3. `ConEmuANSI` == "ON"
4. `TERM_PROGRAM` in {vscode, iTerm.app, WezTerm}
5. Compile-time `#[cfg(target_os = "windows")]` default true

On a Pi running lxterm or any terminal that does not set one of those vars, every branch
fell through to `false`. The consequence was that all 35 RGB theme colors were passed to
the 16-color `approximate_ansi()` fallback. That function mapped colors by dominant
channel, producing a very lossy result: both the cursor foreground (`#e0c097`) and
background (`#1a1a2e`) resolved to `Color::Blue`, making the cursor invisible against
its own background.

Two candidate fixes were considered:

- **Option A:** Add `TERM.contains("256color")` to `supports_truecolor()`, treating
  `xterm-256color` as evidence of 24-bit RGB support.
- **Option B:** Keep `supports_truecolor()` semantically strict (24-bit RGB only) and
  replace the 16-color fallback with a 256-color indexed mapping.

## Decision

Option B was adopted, with one additive change to `supports_truecolor()`.

**Change 1 — New truecolor signals in `supports_truecolor()`:**
Three known Linux terminal emulators that unconditionally support truecolor were added
as detection signals: `KITTY_WINDOW_ID`, `ALACRITTY_WINDOW_ID`, and
`FOOT_TERMINAL_VERSION`. These are set by the emulator process itself and are
unambiguous; they do not appear in tmux or SSH passthrough contexts.

`TERM` was deliberately not added. A terminal advertising `xterm-256color` guarantees
indexed-256 support (ESC[38;5;Nm), not 24-bit RGB (ESC[38;2;r;g;bm). Edge cases where
emitting RGB sequences to a non-RGB terminal produces garbled output include: tmux with
`default-terminal xterm-256color`, SSH into a host whose remote `TERM` is overridden,
and older libvte-based terminals. Treating `xterm-256color` as a truecolor signal would
silently corrupt rendering in those environments.

**Change 2 — 256-color indexed fallback replaces 16-color fallback:**
`approximate_ansi()` was rewritten to map `Color::Rgb(r, g, b)` into the xterm 256-color
space:

- Near-gray values (all channels within 10 of each other) are mapped into the 24-entry
  grayscale ramp at indices 232-255 (8, 18, 28 ... 238 luminance steps).
- All other values are mapped into the 6x6x6 color cube at indices 16-231 using
  `index = 16 + 36*r_idx + 6*g_idx + b_idx` where each channel is quantized to 0-5.

The public entry point `to_256_fallback()` wraps `approximate_ansi()` and passes
non-RGB colors through unchanged.

## Consequences

**Positive:**
- The cursor visibility bug is eliminated. `#1a1a2e` (cursor bg) maps to
  `Indexed(59)` (dark blue-black in the cube) and `#e0c097` (cursor fg) maps to
  `Indexed(187)` (warm yellow), making the cursor visible on Pi without truecolor support.
- All 35 theme colors produce visually distinguishable results in the fallback path,
  preserving the intent of the Night Owl theme rather than collapsing it to 8 hues.
- `supports_truecolor()` remains semantically correct: it returns `true` only when the
  terminal is known to handle 24-bit RGB escape sequences. Future callers can trust the
  return value without qualification.
- Terminals that set `KITTY_WINDOW_ID`, `ALACRITTY_WINDOW_ID`, or
  `FOOT_TERMINAL_VERSION` now receive full RGB rendering on Linux.
- Windows behaviour is unchanged: the compile-time `cfg` block still returns `true`
  unconditionally (ConPTY supports truecolor on all currently supported Windows versions).

**Negative / Trade-offs:**
- Terminals that support truecolor but set none of the known env vars (e.g., a
  custom build of st, or an obscure libvte fork) will receive 256-color output rather
  than RGB. Users in those environments can work around this by setting
  `COLORTERM=truecolor` in their shell profile, which is the standard mechanism for
  exactly this case.
- The grayscale ramp mapping uses a linear approximation; it does not account for
  gamma. For the current Night Owl palette this is acceptable, but high-contrast
  accessibility themes may require a gamma-corrected version in future.

## Platform-Specific Considerations

This decision was motivated by Raspberry Pi OS constraints. The Pi 5 target runs
headless or with lxterm/xfce4-terminal, neither of which sets `COLORTERM`. The 256-color
space is universally supported by any terminal shipping a `xterm-256color` terminfo
entry, which covers all terminals in the Raspberry Pi OS package repositories. The fix
requires no runtime dependencies and adds no memory overhead beyond the existing palette
module.
