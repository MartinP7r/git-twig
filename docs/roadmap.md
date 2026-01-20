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

- [x] **Homebrew Formula**: Official tap for macOS users via `cargo-dist` (Metadata added to Cargo.toml).
- [x] **GitHub Pages**: Beautiful landing page and documentation.
- [x] **Deep Example Structure**: Richer `sample-git` for better demos.
- [x] **Branch Symbol Options**: Added support for **Rounded** (Unicode) and **Standard** (ASCII) tree connectors.
- [x] **Horizontal Folder Dividers**: Render empty directory chains as a single horizontal line with custom dividers.
- [ ] **Configurable Keybindings**: Allow customizing TUI keys via `git config`.
- [x] **TUI Help Overlay**: Dedicated pop-up overlay (triggered by `?`) with simplified bottom bar hints.
- [x] **Diff Summary Status Bar**: Dynamic status bar showing global diff summary: `X files changed, Y insertions(+), Z deletions(-)`.
- [ ] **Searchbar UX**: Add a blinking or static cursor/caret to the search input to clearly visualize spaces and position.

---

## Future Ideas / Backlog 📒
- [ ] **Git Worktrees Support**: Visualize and navigate between multiple worktrees.
- [ ] **Custom Themes**: Allow user-defined colors/icons via `git config`.
- [ ] **Machine Readable Output**: JSON/YAML export for scripting integration.
- [ ] **Multi-Selection**: Selection ranges or marking multiple files for batch actions.
- [ ] **Action History**: Undo/Redo support for staging operations.
