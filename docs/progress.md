# Project Progress

## 2026-01-15 - 2026-01-16: Core Interactive Mode & Cleanup

### Features Implemented

#### 1. Interactive TUI Mode
- **Goal**: Enable user interaction with the git status tree for staging/unstaging.
- **Changes**:
  - Switched to `ratatui` and `crossterm` for TUI implementation.
  - Added `-I` / `--interactive` flag to launch the TUI.
  - Implemented Vim-style navigation (`j`/`k`) and basic shortcut keys.
  - Added staging (`s`/`space`) and unstaging (`u`) functionality for files and directories.
  - Implemented recursive staging/unstaging logic for directory nodes.

#### 2. Tree Data Structure Enhancements
- **Goal**: Provide necessary metadata for TUI navigation and git operations.
- **Changes**:
  - Refactored `Node` to track `full_path` and `raw_status`.
  - Implemented a flattening algorithm to convert the tree into a renderable list with visual connectors (├, └, │).

### Improvements & Cleanup
- **Bug Fix**: Resolved "pathspec did not match" error when staging files already marked for deletion.
- **Legacy Cleanup**: Removed leftover Ruby configuration files (`.rubocop.yml`, GitHub workflows).
- **Environment**: Updated `.gitignore` to focus on Rust development artifacts.


## 2026-01-19: Interactive Features & Visual Enhancements

### Features Implemented

#### 4. Global Folding
- **Goal**: Quickly collapse or expand the entire tree.
- **Changes**:
  - Implemented `Shift+H` (collapse all) and `Shift+L` (expand all) in interactive mode.
  - Added recursive directory path collection to `Node::get_all_dir_paths`.
  - Updated help bar UI to display the new shortcuts.

#### 1. Rich Icons (Nerd Fonts)
- **Goal**: Enhance visual distinction of files and directories using Nerd Fonts.
- **Changes**:
  - Implemented `Theme::nerd()` with file-specific icon mappings (e.g., Rust, TOML, Markdown).
  - Added support for special filenames (`Dockerfile`, `LICENSE`) and directories (`src`, `target`).
  - Added `--simple-icons` flag to opt-out of rich icons while keeping the Nerd theme structure.
  - **Fix**: Ensured correct spacing between icons and filenames.
  - **Fix**: Adjusted priority to match special files (like `Cargo.toml`) before extensions.

#### 2. File Jump Navigation
- **Goal**: Faster navigation through large file trees.
- **Changes**:
  - Implemented `d` (down) key: Jumps to the next file, skipping all directories.
  - Implemented `u` (up) key: Jumps to the previous file, skipping all directories.
  - Updated `App::next_file` and `App::previous_file` logic in `interactive.rs`.

#### 3. Tree Folding
- **Goal**: Better management of deep directory hierarchies.
- **Changes**:
  - Implemented `h` (collapse) and `l` (expand) keys in interactive mode.
  - Updated `Node::flatten` to respect a `collapsed_paths` set.
  - Directories added to `collapsed_paths` are rendered without their children.

### Improvements & Fixes

- **Formatting**: Applied `cargo fmt` to `src/interactive.rs` and `src/node.rs`.
- **Testing**:
  - Added unit tests for `icons.rs` to verify icon mappings.
  - Added regression tests for icon spacing (`" test.rs"`).
  - Added tests for tree folding logic (`test_flatten_collapsed`).
- **Docs**:
  - Updated `README.md` with git config documentation (`twig.indent`, `twig.collapse`, `twig.theme`).
  - Updated `walkthrough.md` with new feature indicators.
  - Updated `docs/roadmap.md` to mark features as complete.


## 2026-01-23: Roadmap Completion (v1.1.0 Polish)

### Features Implemented

#### 1. Searchbar UX (Cursor)
- **Goal**: Clearly visualize the insertion point in the search bar.
- **Changes**:
  - Added `f.set_cursor` in `src/tui/ui.rs` to show the terminal cursor while typing.
  - Calculated cursor position using `unicode-width` for accurate placement.

#### 2. Configurable Keybindings
- **Goal**: Allow power users to customize their workflow.
- **Changes**:
  - Implemented `KeyConfig` and `Action` system in `src/config.rs`.
  - Added `git::get_config_regexp` to load custom mappings from `git config`.
  - Refactored `src/tui/event.rs` to use action-based dispatching instead of hardcoded keys.
  - Supports remapping keys via `git config twig.key.<action> <key>`.

### Current Status
- Branch: `feat/v1.1-polish`
- CI/CD: All tests passing.
- Version: v1.1.0 milestone features fully implemented.


## 2026-01-24: v1.2.0 "Efficiency" Milestone (Completed)

### Features Implemented

#### 1. Power User Navigation
- **Goal**: Enable rapid traversal and manipulation of the file tree.
- **Changes**:
  - **Visual Line Mode (`Shift+V`)**: Implemented multi-selection for bulk staging/unstaging and folding.
  - **Vim Motions**: Added `gg` (top), `G` (bottom), `zz` (center view), and `Ctrl+u`/`Ctrl+d` (paging).
  - **Compact View**: Added a toggle (`v`) to collapse empty directory chains (e.g., `src/ui/components`) into single lines.
  - **Yank Path (`y`)**: Added ability to copy the selected node's path to the system clipboard.

#### 2. Advanced Search & Worktrees
- **Goal**: Deepen the integration with git's core features and large codebases.
- **Changes**:
  - **Diff Search**: Implemented search (`/`) within the Diff View, with next/prev (`n`/`N`) navigation and match highlighting.
  - **Git Worktrees**: Added a modal (`w`) to list and instantly switch between git worktrees.
  - **Scroll Clamping**: Polished UI to prevent scrolling past content boundaries in help and diff views.

#### 3. Reliability & Output
- **Goal**: Make the tool robust and scriptable.
- **Changes**:
  - **Action History**: Implemented Undo (`u`) and Redo (`Ctrl+r`) for staging operations to prevent data loss accidents.
  - **Machine Readable Output**: Added `--json` and `--yaml` flags to export the internal tree state for external tooling (e.g., status bars or scripts).
  - **Documentation**: Migrated docs to **MkDocs Material** for a premium, searchable web experience hosted on GitHub Pages.

### Improvements & Fixes
- **UI Polish**: Removed redundant bottom-bar hints, improved help modal scrolling, and refined status bar layout.
- **Code Quality**: Added `dead_code` cleanup and ANSI stripping for robust diff searching.


## 2026-01-26: v1.3.0 "Commit" Milestone (Planning)

### Upcoming Features
- **TUI Commit**: Commit staged changes directly from the interface (`c`), with editor fallback.
- **Interactive Patch Staging**: Stage specific hunks or lines from the Diff View (partial staging).
- **Git Revert**: Quickly revert changes for specific files.
- **Command Runner**: Execute arbitrary shell commands on selected files (`!`).
