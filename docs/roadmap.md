---
title: "Roadmap"
description: "The path ahead for Git-Twig development."
---

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
- [x] **Configurable Keybindings**: Allow customizing TUI keys via `git config`.
- [x] **TUI Help Overlay**: Dedicated pop-up overlay (triggered by `?`) with simplified bottom bar hints.
- [x] **Diff Summary Status Bar**: Dynamic status bar showing global diff summary: `X files changed, Y insertions(+), Z deletions(-)`.
- [x] **Searchbar UX**: Add a blinking or static cursor/caret to the search input to clearly visualize spaces and position.


---

## Upcoming: v1.2.0 - The "Efficiency" Milestone ⚡️
Focus: Speeding up interactions for power users.

- [x] **Visual Line Mode (`Shift+V`)**: Select multiple lines to stage/unstage or fold/unfold in bulk.
- [x] **Advanced Vim Navigation**:
    - `gg` / `G`: Jump to start/end of the list.
    - `zz`: Center the viewport on the current selection.
    - `ctrl+u` / `ctrl+d`: Page up/down.
- [x] **Compact Path Rendering**: A new view toggle option (cycle via `v`) that collapses empty directory chains into a single line (e.g., `src/ui/components` → `src・ui・components`).
- [x] **Mintlify Documentation**: Migration of documentation to Mintlify for a premium, modern developer experience.
- [x] **Yank Path (`y`)**: Copy the relative or absolute path of the selected file to the system clipboard.

---

## Future Ideas / Backlog 📒
- [x] **Custom Themes**: Allow user-defined colors/icons via `git config`. Default folders to orange.
- [ ] **Machine Readable Output**: JSON/YAML export for scripting integration.
- [x] **Easter Egg**: `Alt-V` switches to a view where folders alternate sides, making it look like a physical tree.
- [x] **Action History**: Undo/Redo support for staging operations.
- [x] **Scroll Resistance**: Add a small resistance/haptic-like pause when scrolling to the top edge before looping or stopping.
- [x] **Help Modal Scrolling**: Allow navigation keys (`j`/`k`, labels, etc.) to scroll help text when overlay is visible.
- [ ] **Git Worktrees Support**: Visualize and navigate between multiple worktrees.
- [ ] **Diff Search**: Search for text *within* the diff content in Diff View.
