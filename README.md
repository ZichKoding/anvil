# Anvil

A lightweight terminal IDE built in Rust. Where iron meets purpose.

## Features

- **File tree sidebar** with lazy directory loading and expand/collapse
- **Syntax highlighting** via tree-sitter (Rust, Python, JavaScript, JSON, TOML, Markdown)
- **Vim-style keybindings** (Normal/Insert modes) with VS Code mode option
- **RetroTerm theme** - warm amber/green-on-dark terminal aesthetic
- **Rope-based text buffer** (ropey) for efficient editing of large files
- **Configurable** via `~/.config/anvil/anvil.toml`
- **Cross-platform** - Linux (x86_64, aarch64), macOS, Windows, Termux

## Install

```bash
# One-liner (Linux & macOS)
curl -fsSL https://raw.githubusercontent.com/ZichKoding/anvil/main/install.sh | sh
```

```powershell
# One-liner (Windows PowerShell)
irm https://raw.githubusercontent.com/ZichKoding/anvil/main/install.ps1 | iex
```

```bash
# From source
cargo install --path .

# For Pi / Termux (optimized release build)
CARGO_TARGET_DIR=/tmp/anvil-target cargo build --release
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

| Key | Mode | Action |
|-----|------|--------|
| `i` | Normal | Enter insert mode |
| `a` / `A` | Normal | Append after cursor / end of line |
| `o` / `O` | Normal | Open line below / above |
| `Esc` | Insert | Return to normal mode |
| `x` | Normal | Delete character |
| `h/j/k/l` | Normal | Navigate left/down/up/right |
| `g` / `G` | Normal | Go to top / bottom of file |
| `Tab` | Any | Toggle focus (tree / editor) |
| `Ctrl+S` | Any | Save file |
| `Ctrl+B` | Any | Toggle sidebar |
| `Ctrl+N/P` | Any | Next / previous tab |
| `Ctrl+W` | Any | Close current tab |
| `Ctrl+C` | Any | Quit |
| `q` | Normal | Quit |

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

[editor]
show_line_numbers = true
tab_size = 4
```

## Architecture

Built with the same stack as the Helix editor:
- **ratatui** + **crossterm** - TUI framework
- **tree-sitter** - Incremental parsing for syntax highlighting
- **ropey** - Rope data structure for O(log n) text operations

Optimized for Raspberry Pi 5: <15MB idle memory, atomic saves for SD card safety, `opt-level = "z"` for minimal binary size.

## License

Apache-2.0
