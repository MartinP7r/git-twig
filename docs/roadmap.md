# Product Roadmap

## Shipped Releases 🚢

### [v1.0.0] - The "Workflow" Milestone (Current)
Focus: Transforming the tool from a viewer into an actionable interactive terminal application.

- [x] **Interactive Mode (TUI)**: Full keyboard navigation (Vim bindings).
- [x] **Real-time Staging**: Toggle `git add`/`reset` directly from the tree.
- [x] **Inline Diffs**: View changes with syntax highlighting.
- [x] **Split View**: Vertical panes for Staged vs Unstaged changes.
- [x] **Fuzzy Search**: Quickly filter files with `/`.
- [x] **Architectural Refactor**: Modularized codebase for better maintenance.
- [x] **Global Folding**: Expand/Collapse all nodes with `Shift+H/L`.
- [x] **Theme Engine**: Support for ASCII, Unicode, and Nerd Font themes.
- [x] **Editor Integration**: Open all modified files in `$EDITOR`.

### [v0.1.0 - v0.2.0] - Foundation (Shipped)
- [x] **Context Header**: Branch info and ahead/behind counts.
- [x] **Smart Filtering**: `--staged-only`, `--modified-only`, etc.
- [x] **Visual Polish**: Vertical alignment, semantic icons, and diff bars.

---

## Upcoming: v1.1.0 - Distribution & Presence 🚀
Focus: Making git-twig easier to install and more visible.

- [ ] **Homebrew Formula**: Official tap for macOS users via `cargo-binstall`.
- [ ] **GitHub Pages**: Beautiful landing page and documentation.
- [x] **Deep Example Structure**: Richer `sample-git` for better demos.
- [ ] **Horizontal Folder Dividers**: 
    - Render empty directory chains as a single horizontal line with custom dividers.
    - Path should appear on one line, with files starting on the immediate next line.
    - Toggle/Configuration for this "Ultra-Compact" view.
- [ ] **Configurable Keybindings**: Allow customizing TUI keys via `git config`.
- [ ] **TUI Help Overlay**: 
    - Move detailed cheat sheet from the bottom bar into a dedicated pop-up overlay (triggered by `?`).
    - Keep only the most vital hints (e.g. `[?] Help`) visible in the main view.
- [ ] **Diff Summary Status Bar**: 
    - Repurpose the bottom bar into a dynamic status bar.
    - Show global diff summary: `X files changed, Y insertions(+), Z deletions(-)`.

---

## Future Ideas / Backlog 📒
- [ ] **Git Worktrees Support**: Visualize and navigate between multiple worktrees.
- [ ] **Custom Themes**: Allow user-defined colors/icons via `git config`.
- [ ] **Machine Readable Output**: JSON/YAML export for scripting integration.
- [ ] **Multi-Selection**: Selection ranges or marking multiple files for batch actions.
- [ ] **Action History**: Undo/Redo support for staging operations.
