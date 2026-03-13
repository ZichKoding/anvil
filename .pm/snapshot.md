---
project: Anvil
github_repo: ZichKoding/anvil
last_synced: 2026-03-13T18:00:00Z
---
# Anvil - Project Snapshot

## Open Issues (5)
- #6 chore: Remove dead code flagged by compiler warnings [tech-debt, good first issue] — **milestone: v0.2.0**
- #7 feat: Integrate terminal color fallback for non-truecolor terminals [enhancement]
- #8 feat: Implement Command mode (: prompt) for Vim keybindings [enhancement]
- #9 test: Add unit test suite [testing, tech-debt] — **milestone: v0.2.0**
- #10 feat: Map Function and Constant syntax highlight groups [enhancement, good first issue]

## Open PRs
- #11 Release v0.2.0: Dead code cleanup, unit tests, and bug fixes (`release/v0.2.0 → main`) — closes #6, #9; CI green

## Recently Closed
- PR #5 Add supported languages section to README (merged 2026-03-11) — closed #4
- PR #3 Implement horizontal viewport scrolling (merged 2026-03-11) — closed #2

## Milestones
- **v0.2.0** (open) — Dead code cleanup, unit tests, clippy compliance
  - Issues: #6, #9
  - PR: #11 (release PR, CI passing)

## Cross-Reference Notes
Issue #6 references issues #7, #8, #9 (via #[allow(dead_code)] TODO comments).
- Mode::Command tracked by #8
- HighlightGroup::Function / Constant tracked by #10
- supports_truecolor() / to_256_fallback() / approximate_ansi() tracked by #7
