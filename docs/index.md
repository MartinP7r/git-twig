---
title: "Introduction"
description: "A fast, interactive, and actionable alternative to `git status`."
---

# Introduction

**Git-Twig** is a command-line tool designed to help developers manage their Git repositories with speed and clarity. Instead of reading a flat list of modified files, git-twig provides an interactive tree view that allows you to stage, unstage, and inspect changes directly from your terminal.

## Key Features

- **Interactive TUI**: Navigate your repository with familiar Vim bindings (`j`/`k`, `gg`, `G`).
- **Actionable Tree**: Stage (`Space`) or View Diff (`Enter`) items directly.
- **Smart Folding**: Keep your view clean by folding directories you're not working on.
- **Power User Efficiency**: High-speed navigation with `gg`, `G`, and `Ctrl+u/d`.
- **Visual Themes**: Choose from ASCII, Unicode, or Nerd Font icon themes.
- **Colored Icons**: File icons are colored by type in Nerd Font mode.

## Why Git-Twig?

Standard `git status` output is excellent for a quick glance, but it's passive. Git-twig turns that output into a workspace. It's built in Rust for extreme performance and provides a premium terminal experience.

## Quick Start

```bash
# Install via Homebrew
brew tap MartinP7r/tap
brew install git-twig

# Or via cargo
cargo install git-twig

# Start using it
git twig
```

## Next Steps

- [Quickstart Guide](quickstart.md) - Get started in under 5 minutes
- [Navigation](features/navigation.md) - Learn the keyboard shortcuts
- [Visual Mode](features/visual-mode.md) - Multi-select operations
