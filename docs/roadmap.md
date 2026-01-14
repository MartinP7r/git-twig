# Product Roadmap

## Q1 - The "Context" Release (v0.2.0)
Focus: Providing immediate context to the user before they commit.

- [ ] **Context Header**
    - Display current branch name.
    - Show ahead/behind counts (`⬆️ 2 ⬇️ 0`).
    - Show upstream branch status.
    - *Note: Make fancy icons (⎇) optional for font compatibility.*
- [ ] **Smart Filtering**
    - `--staged-only`: View only files ready to commit.
    - `--modified-only`: Hide untracked files to reduce noise.

## Q2 - The "Workflow" Release (v1.0.0)
Focus: Transforming the tool from a viewer into an actionable workflow step.

- [ ] **Interactive Mode (TUI)**
    - Navigate the tree with arrow keys.
    - Toggle staging (`git add`) with `<Space>`.
    - View diffs inline with `<Enter>`.
- [ ] **Editor Integration**
    - Flag `--open` (or `-o`) to open all modified files in `$EDITOR`.

## Visual Polish & Theming
Focus: Making the tool look modern and customizable ("git-twig").

- [ ] **Configurable Themes**: Support `--theme` flag.
    - `ascii`: Default/Safe.
    - `unicode`: Rounded corners (`╰──`), Block diff bars (`◼◼◼◻`).
    - `nerd`: Full icons for files/folders and status glyphs.
- [ ] **Semantic Icons**: Support file-type icons or nerd-font ligatures (e.g. 🦀 for Rust, 💎 for Ruby) and folder icons (⚙️ for `config`).
- [ ] **Vertical Alignment**: Align diff stats separator (`|`) vertically across all rows.
- [ ] **High-Res Diff Bars**: Implement Unicode block element rendering.
- [x] **Rename Config Keys**: Migrate from `status-tree.*` to `twig.*` (e.g. `twig.collapse`).
- [x] **Brand Identity**: Design logo and assets.

## Future Ideas / Backlog
- [ ] **Git Worktrees Support**: Visualize multiple worktrees.
- [ ] **Configurable Themes**: Allow user-defined colors/icons via `git config`.
- [ ] **Jason/YAML Output**: For machine parsing/integration with other tools.
