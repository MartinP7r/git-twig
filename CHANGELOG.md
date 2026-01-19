# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-01-20

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

## [0.1.0] - 2026-01-14

### Added
* Default collapse setting configurable via `git config --global status-tree.collapse <true|false>`
* GitHub Actions rake status badge in README

---

[Unreleased]: https://github.com/martinp7r/git-twig/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/martinp7r/git-twig/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/martinp7r/git-twig/releases/tag/v0.1.0
