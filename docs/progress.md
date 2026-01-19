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

### Current Status
- Branch: `feat/rich-icons`
- CI/CD: Tests passing locally.
- Version: Ready for further feature development or release preparation.
