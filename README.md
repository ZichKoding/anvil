# Anvil

A lightweight terminal IDE built in Rust. Where iron meets purpose.

## Features

- **File tree sidebar** with lazy directory loading and expand/collapse
- **Syntax highlighting** via tree-sitter (Rust, Python, JavaScript, JSON, TOML, Markdown)
- **Vim-style keybindings** (Normal/Insert/Command modes) with VS Code mode option
- **Command mode** — `:w` save, `:q` quit, `:wq` save+quit, `:q!` force quit with unsaved changes guard
- **Terminal color fallback** — Automatic 256-color indexed approximation for non-truecolor terminals
- **Context-aware highlighting** — Functions and constants classified via tree-sitter parent node analysis
- **RetroTerm theme** - warm amber/green-on-dark terminal aesthetic
- **Rope-based text buffer** (ropey) for efficient editing of large files
- **Configurable** via `~/.config/anvil/anvil.toml`
- **Cross-platform** - Linux (x86_64, aarch64), macOS, Windows, Termux

## Install

### From source (current)

```bash
cargo install --path .

# For Pi / Termux (optimized release build)
CARGO_TARGET_DIR=/tmp/anvil-target cargo build --release
```

### Pre-built binaries

```bash
# Linux & macOS
curl -fsSL https://raw.githubusercontent.com/ZichKoding/anvil/main/install.sh | sh
```

```powershell
# Windows PowerShell
irm https://raw.githubusercontent.com/ZichKoding/anvil/main/install.ps1 | iex
```

## Usage

```bash
# Open current directory
anvil

# Open specific directory
anvil /path/to/project
```

## Keybindings

### Vim Mode (default)

#### Global (any mode)

| Key | Mode | Action |
|-----|------|--------|
| `Ctrl+S` | Any | Save file |
| `Ctrl+B` | Any | Toggle sidebar |
| `Ctrl+N` | Any | Next tab |
| `Ctrl+P` | Any | Previous tab |
| `Ctrl+W` | Any | Close current tab |
| `Ctrl+C` | Any | Quit |
| `Ctrl+Q` | Any | Quit |

#### Normal mode

| Key | Mode | Action |
|-----|------|--------|
| `i` | Normal | Enter insert mode |
| `a` / `A` | Normal | Append after cursor / end of line |
| `o` / `O` | Normal | Open line below / above |
| `x` | Normal | Delete character |
| `h/j/k/l` | Normal | Navigate left/down/up/right |
| `0` / `$` | Normal | Go to beginning / end of line |
| `g` / `G` | Normal | Go to top / bottom of file |
| `Home` / `End` | Normal | Go to beginning / end of line |
| `PageUp` / `PageDown` | Normal | Page up / down |
| `Tab` | Normal | Toggle focus (tree / editor) |
| `q` | Normal | Quit |
| `:` | Normal | Enter command mode |

#### Insert mode

| Key | Mode | Action |
|-----|------|--------|
| `Esc` / `Ctrl+[` | Insert | Return to normal mode |
| `Enter` | Insert | Insert newline |
| `Backspace` | Insert | Delete character before cursor |
| `Delete` | Insert | Delete character at cursor |
| `Tab` | Insert | Insert spaces (tab_size width) |
| `Arrow keys` | Insert | Navigate |
| `Home` / `End` | Insert | Go to beginning / end of line |

#### Command mode

| Key | Mode | Action |
|-----|------|--------|
| `:w` | Command | Save file |
| `:q` | Command | Quit (fails if unsaved changes) |
| `:wq` | Command | Save active buffer and quit (fails if other buffers have unsaved changes) |
| `:q!` | Command | Force quit (discard changes) |
| `Backspace` | Command | Delete last character (exits command mode if empty) |
| `Esc` | Command | Cancel command |

#### File tree (when tree is focused)

| Key | Mode | Action |
|-----|------|--------|
| `j` / `k` | Tree | Navigate down / up |
| `l` / `Enter` | Tree | Open file or expand directory |
| `h` / `Left` | Tree | Collapse directory |
| `Right` | Tree | Expand directory |
| `Tab` | Tree | Switch focus to editor |

### VS Code Mode

Set `keybinding_mode = "vscode"` in config. Always in insert mode, no Normal/Insert distinction.

## Configuration

Create `~/.config/anvil/anvil.toml`:

```toml
[general]
theme = "retroterm"
keybinding_mode = "vim"    # or "vscode"
mouse_enabled = true

[sidebar]
width = 25
show_icons = true

[editor]
show_line_numbers = true
tab_size = 4
```

## Supported Languages

| Language | Extensions |
|----------|------------|
| Rust | `.rs` |
| Python | `.py`, `.pyi` |
| JavaScript | `.js`, `.jsx`, `.mjs`, `.cjs` |
| JSON | `.json`, `.jsonc` |
| TOML | `.toml` |
| Markdown | `.md`, `.markdown` |

## Architecture

Built with the same stack as the Helix editor:
- **ratatui** + **crossterm** - TUI framework
- **tree-sitter** - Incremental parsing for syntax highlighting
- **ropey** - Rope data structure for O(log n) text operations

Optimized for Raspberry Pi 5: <15MB idle memory, atomic saves for SD card safety, `opt-level = "z"` for minimal binary size.

## License

Apache-2.0
