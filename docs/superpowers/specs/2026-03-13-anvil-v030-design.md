# Anvil v0.3.0 Design Specification

## Overview

Three enhancement features for Anvil v0.3.0, implemented sequentially into `release/v0.3.0`.

## Issue #10: Map Function and Constant Highlight Groups

**Goal:** Context-aware identifier classification using parent node kind from tree-sitter AST.

**Changes:**
- `src/syntax/highlighter.rs`: Modify `node_kind_to_group` to accept `parent_kind: Option<&str>` parameter
- `collect_leaf_spans`: Pass parent node kind when calling `node_kind_to_group`
- Remove `#[allow(dead_code)]` from `Function` and `Constant` variants

**Mappings:**
- `"identifier"` + parent `call_expression|function_item|function_definition|function_declaration` -> Function
- `"identifier"` + parent `const_item` -> Constant
- `"identifier"` + other -> Variable (existing)

## Issue #7: Terminal Color Fallback

**Goal:** Resolve fallback colors at theme construction time for non-truecolor terminals.

**Changes:**
- `src/theme/mod.rs`: Add `with_fallback_colors()` method, update `default_theme()` to conditionally apply
- `src/theme/palette.rs`: Remove `#[allow(dead_code)]` from `supports_truecolor()`, `to_256_fallback()`, `approximate_ansi()`

**Design:** One-time startup cost, not per-frame. `default_theme()` checks `supports_truecolor()` and applies `with_fallback_colors()` if needed.

## Issue #8: Command Mode

**Goal:** Implement vim-style command mode (`:w`, `:q`, `:wq`, `:q!`).

**Changes:**
- `src/input/command.rs`: New module with `CommandResult` enum and `execute_command()` function
- `src/input/mod.rs`: Add `pub mod command;`
- `src/app.rs`: Add `command_buffer: String`, remove dead_code on `Mode::Command`
- `src/input/handler.rs`: Add command mode entry (`:` key) and handler
- `src/ui/layout.rs`: Conditional command-line rendering in status bar

**Flow:** `:` in Normal mode -> Command mode. Esc -> Normal. Enter -> execute. Backspace on empty -> Normal.

**Unsaved guard:** `:q` checks `editors.iter().any(|e| e.buffer.modified)`. If unsaved, shows warning. `:q!` force quits.
