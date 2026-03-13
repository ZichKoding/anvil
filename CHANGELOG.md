# Changelog

All notable changes to Anvil will be documented in this file.

## [0.3.0] - 2026-03-13

### Added
- **Command mode** — Vim-style `:w`, `:q`, `:wq`, `:q!` with unsaved changes guard (#8)
- **Function/constant highlighting** — Context-aware identifier classification via tree-sitter parent node (#10)
- **Terminal color fallback** — Automatic ANSI color approximation for non-truecolor terminals (#7)
- 50 new unit tests (152 total)

### Fixed
- Truecolor detection — recognize Windows Terminal, ConEmu, VS Code; default to truecolor on Windows
- Cursor visibility — software-rendered cursor cell with inverted colors, independent of terminal emulator
- RetroTerm theme contrast — improved 9 color values for better readability (gutter, comments, borders, punctuation, operators)
- Integer overflow in `approximate_ansi()` luminance calculation (u16 → u32)
- `:wq` data-loss bug — now checks all editors for unsaved changes after saving active buffer

## [0.2.0] - 2026-03-13

### Removed
- `HighlightGroup::default_color()` method (superseded by `Theme::color_for_group()`)
- `Viewport::visible_range()` method (unused)
- `Theme.name` and `Theme.selection_bg` fields (set but never read)
- Unused `_source` parameter threaded through syntax highlighting functions

### Fixed
- **Performance:** `rope.to_string()` was called inside the render loop once per visible line, causing O(n) allocations per frame. The full-document conversion is now eliminated from the hot path entirely since `highlight_line` no longer requires the source text.
- **Correctness:** `render_editor_area` used direct array indexing (`editors[idx]`) which could panic when all editors are closed. Now uses bounds-checked `get_mut()`.
- **Correctness:** Tautological branch `if is_cursor_line { theme.fg } else { theme.fg }` simplified.
- **Clippy:** Resolved all 15 clippy warnings including derivable impls, collapsible ifs, needless returns, manual clamp, parameter-only-used-in-recursion, and too-many-arguments.

### Improved
- `FileTree::render_themed` now accepts `&Theme` instead of 10 individual color parameters, eliminating a class of silent argument-order bugs.
- `collect_leaf_spans` extracted from `SyntaxHighlighter` impl to a standalone function (clippy: only-used-in-recursion).

### Added
- 102 unit tests across 6 modules: Buffer, Cursor, Viewport, Config, palette, and languages.
- `#[allow(dead_code)]` annotations with issue references on intentionally kept scaffolding:
  - `Mode::Command` (#8)
  - `HighlightGroup::Function` / `Constant` (#10)
  - `supports_truecolor()` / `to_256_fallback()` / `approximate_ansi()` (#7)

## [0.1.0] - 2025-01-01

### Added
- Initial release
- Vim and VS Code keybinding modes
- Tree-sitter syntax highlighting (Rust, Python, JavaScript, JSON, TOML, Markdown)
- Rope-backed buffer with atomic save
- File tree with lazy directory loading
- Horizontal viewport scrolling
- Configurable via `~/.config/anvil/anvil.toml`
- RetroTerm color theme
- Cross-platform builds (Linux x86_64, Linux ARM64, macOS x86_64, macOS ARM64, Windows x86_64)
