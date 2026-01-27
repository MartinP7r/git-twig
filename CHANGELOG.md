# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v1.2.4] - 2026-01-27

### Fixed
- **Release CI**: Updated cargo-dist to v0.30.3 and fixed Homebrew tap configuration.

## [v1.2.3] - 2026-01-27

### Fixed
- **Release CI**: Fixed cargo-dist configuration by adding `hosting` option.

## [v1.2.2] - 2026-01-27

### Added
- **Colored Icons**: File and directory icons now have distinct colors based on type in Nerd Font mode.
- **Collapsible Branch Indicators**: Tree view now shows `[+]` and `[-]` indicators for directory expansion state.

### Fixed
- **Help Popup**: Removed unused `zz` binding from help menu.

## [v1.2.0] - 2026-01-24

### Added
- **Visual Line Mode (`Shift+V`)**: Multiselect files for bulk actions.
- **Advanced Navigation**: `gg` / `G` (jump), `zz` (center), `Ctrl+u` / `Ctrl+d` (paging).
- **Compact Path Rendering**: Cycle view (v) to collapse empty directory chains.
- **Yank Path (`y`)**: Copy paths to system clipboard.
- **Action History**: Undo (`u`) and Redo (`Ctrl+r`) for staging actions.
- **Help Modal Scrolling**: Scroll through the `?` overlay with navigation keys.
- **Easter Egg**: `Alt-V` for an alternating tree view.
- **Customization**: User-defined colors and icons via `git config`.

### Changed
- **Documentation Migration**: Moved from Mintlify to MkDocs Material for GitHub Pages.
- **Branding**: Default folder color changed to Orange.

### Fixed
- **Scroll Resistance**: Improved wrap-around behavior at list edges.

## [v1.1.0] - 2026-01-22

### Added
- **Configurable Keybindings**: Remap TUI keys via `git config twig.key.*`.
- **TUI Help Overlay**: Integrated `?` pop-up for keyboard hints.
- **Status Bar**: Added dynamic summary showing global staged/unstaged counts.
- **Searchbar UX**: Visual cursor added to the fuzzy search input.
- **Branch Symbols**: Added support for **Rounded** and **ASCII** tree connectors.

### Fixed
- **Icons**: Directory nodes now correctly display folder icons in Nerd Font mode.
- **Padding**: Vertical alignment refined for clearer tree visualization.

## [v1.0.0] - 2026-01-20

### Added
- **Interactive Mode (TUI)**: Complete terminal UI for managing git status.
    - Vim-style navigation (`h`, `j`, `k`, `l`).
    - Staging/Unstaging files and folders with `<Space>`.
    - Inline diff view with `<Enter>`.
    - Fuzzy search for files with `/`.
    - Folder expansion/collapse logic.
- **Split View**: Option to see Staged and Unstaged changes in separate panes (Toggle with `v`).
- **Context Header**: Improved branch info and ahead/behind status display.
- **Visual Themes**: Support for `ascii`, `unicode`, and `nerd` font themes.
- **Rich Icons**: Semantic file and folder icons for Nerd Font users.
- **Alignment**: Vertically aligned diff bars and status chars across the tree.
- **Editor Integration**: `-o` / `--open` flag to launch `$EDITOR` with all modified files.

### Changed
- **Architectural Refactor**: Modularized codebase for better performance and maintainability.
- **Git Logic**: Centralized all git command execution.

## [v0.1.0] - 2026-01-14

### Added
* Default collapse setting configurable via `git config --global status-tree.collapse <true|false>`
* GitHub Actions rake status badge in README

---

[Unreleased]: https://github.com/martinp7r/git-twig/compare/v1.0.0...HEAD
[v1.0.0]: https://github.com/martinp7r/git-twig/compare/v0.1.0...v1.0.0
[v0.1.0]: https://github.com/martinp7r/git-twig/releases/tag/v0.1.0
