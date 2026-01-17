# Product Roadmap

## Q1 - The "Context" Release (v0.2.0)
Focus: Providing immediate context to the user before they commit.

- [x] **Context Header**
    - Display current branch name.
    - Show ahead/behind counts (`⬆️ 2 ⬇️ 0`).
    - Show upstream branch status.
    - *Note: Make fancy icons (⎇) optional for font compatibility.*
- [x] **Smart Filtering**
    - `--staged-only`: View only files ready to commit.
    - `--modified-only`: Hide untracked files to reduce noise.
    - `--untracked-only`: View only untracked files.

## Q2 - The "Workflow" Release (v1.0.0)
Focus: Transforming the tool from a viewer into an actionable workflow step.

- [x] **CI Automation**: GitHub Action to run `cargo test` on every PR (separate from release).
- [x] **Interactive Mode (TUI)**
    - [x] Navigate the tree with arrow keys or **Vim bindings** (`h`, `j`, `k`, `l`).
    - [x] Toggle staging (`git add`/`reset`) for **files and whole folders** with `<Space>` (unified toggle).
    - [x] View diffs inline with `<Enter>` (with syntax highlighting).
- [x] **Editor Integration**
    - [x] Flag `--open` (or `-o`) to open all modified files in `$EDITOR`.

## Visual Polish & Theming
Focus: Making the tool look modern and customizable ("git-twig").

- [x] **Configurable Themes**: Support `--theme` flag.
    - `ascii`: Default/Safe.
    - `unicode`: Rounded corners (`╰──`), Block diff bars (`◼◼◼◻`).
    - `nerd`: Full icons for files/folders and status glyphs.
- [x] **Semantic Icons**: Support file-type icons or nerd-font ligatures (e.g. 🦀 for Rust, 💎 for Ruby) and folder icons (⚙️ for `config`).
- [x] **Compact Paths**: Flatten empty directory chains (e.g., `src/main/java...` on one line) to reduce nesting noise.
- [x] **Vertical Alignment**: Align diff stats separator (`|`) vertically across all rows.
- [x] **High-Res Diff Bars**: Implement Unicode block element rendering.
- [x] **Rename Config Keys**: Migrate from `status-tree.*` to `twig.*` (e.g. `twig.collapse`).
- [x] **Brand Identity**: Design logo and assets.

## Future Ideas / Backlog
- [ ] **Git Worktrees Support**: Visualize multiple worktrees.
- [ ] **Configurable Themes**: Allow user-defined colors/icons via `git config`.
- [ ] **Jason/YAML Output**: For machine parsing/integration with other tools.
- [x] **Split View**: Option to show staged and unstaged files in vertically separate sections (Toggle with `v`).
- [ ] **Fuzzy Search**: Filter file list in interactive mode with `/`.
- [ ] **File Jump**: Jump between files (skipping directories) with `d`/`u`.
