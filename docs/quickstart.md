---
title: "Quickstart"
description: "Get up and running with Git-Twig in minutes."
---

## Installation

### Homebrew (macOS & Linux)

The easiest way to install git-twig is via Homebrew:

```bash
brew install MartinP7r/tap/git-twig
```

### Cargo

If you have Rust installed, you can build from source:

```bash
cargo install git-twig
```

## First Steps

1.  Navigate to a Git repository.
2.  Run `git-twig -I`.
3.  Use `j` and `k` to move through the project tree.
4.  Press `Space` to stage a file.
5.  Press `q` or `Esc` to exit when you're done.

## Command Line Options

| Flag | Description |
| :--- | :--- |
| `-I, --interactive` | Enable the interactive TUI mode. |
| `--theme <value>` | Choose a theme: `ascii`, `unicode`, `rounded`, `nerd`. |
| `--indent-size <int>` | Set the directory indentation level (default: 2). |
| `--collapse` | Start with all directories collapsed. |

Next, explore our **Core Features** to see how to level up your workflow.
