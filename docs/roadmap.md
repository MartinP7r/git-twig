# Product Roadmap

## Q1 - The "Context" Release (v0.2.0)
Focus: Providing immediate context to the user before they commit.

- [ ] **Context Header**
    - Display current branch name.
    - Show ahead/behind counts (`⬆️ 2 ⬇️ 0`).
    - Show upstream branch status.
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

## Future Ideas / Backlog
- [ ] **Git Worktrees Support**: Visualize multiple worktrees.
- [ ] **Configurable Themes**: Allow user-defined colors/icons via `git config`.
- [ ] **Jason/YAML Output**: For machine parsing/integration with other tools.
