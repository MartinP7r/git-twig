---
title: "Roadmap"
description: "The path ahead for Git-Twig development."
---

# Product Roadmap

## Shipped Releases 🚢

### [v1.2.0] - The "Efficiency" Milestone (Current)
Focus: Power-user speed and premium documentation.

- [x] **Visual Line Mode (`Shift+V`)**: Select multiple lines to stage/unstage or fold/unfold in bulk.
- [x] **Advanced Vim Navigation**: `gg`/`G`, `Ctrl+u`/`Ctrl+d`.
- [x] **Compact Path Rendering**: View toggle (`v`) for collapsed directory chains.
- [x] **MkDocs Documentation**: Beautiful, searchable docs hosted on GitHub Pages.
- [x] **Yank Path (`y`)**: Copy absolute/relative paths to system clipboard.
- [x] **Machine Readable Output**: `--json` and `--yaml` support for scripting.
- [x] **Git Worktrees Support**: Modal (`w`) to switch between worktrees.
- [x] **Diff Search**: Search (`/`) and navigation (`n`/`N`) within the diff view.
- [x] **Action History**: Undo (`u`) and Redo (`Ctrl+r`) for staging.

### [v1.1.0] - Distribution & Presence 🚀
- [x] **Homebrew Formula**: Official tap for macOS users.
- [x] **TUI Help Overlay**: Integrated `?` pop-up.
- [x] **Status Bar**: Global diff summary stats.
- [x] **Customization**: User-defined colors and icons via `git config`.

### [v1.0.0] - The "Workflow" Milestone
- [x] **Interactive Mode (TUI)**: Full keyboard navigation (Vim bindings).
- [x] **Real-time Staging**: Toggle `git add`/`reset` directly from the tree.
- [x] **Inline Diffs**: View changes with syntax highlighting.
- [x] **Split View**: Vertical panes for Staged vs Unstaged changes.

---

## Upcoming: v1.3.0 - The "Commit" Milestone ✍️
Focus: Completing the local development loop.

- [ ] **TUI Commit**: Press `c` to commit staged changes.
    - Simplified commit message input field.
    - Option to launch `$EDITOR` for long messages.
- [ ] **Interactive Patch Staging**: Select and stage specific hunks or lines within the Diff View.
- [ ] **Git Revert**: Toggle revert for specific files/hunks.
- [ ] **Command Runner**: Press `!` to run arbitrary shelled commands against the selected file/path.

---

## Upcoming: v1.4.0 - Remote Context 🌐
- [ ] **Remote View**: Sidebar or toggle to show remote branches and tracking status.
- [ ] **Fetch/Pull Toggle**: Visual indicators for needed updates.
- [ ] **Stash Management**: List and apply/drop stashes from a dedicated view.

---

## Future Ideas / Backlog 📒
- [ ] **Log Tree**: A visual representation of the git log with branch forks.
- [ ] **Diff Config**: Support for external diff tools (difftastic, delta).
- [ ] **Performance++**: Parallel git status calls for massive repositories.
