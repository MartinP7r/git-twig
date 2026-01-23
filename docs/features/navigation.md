---
title: "Vim Navigation"
description: "Master the keyboard shortcuts for high-speed repository management."
---

Git-twig is designed to keep your hands on the home row. It supports advanced Vim-style navigation to help you fly through even the largest repositories.

## Basic Movement

- **`j` / `k` or `Down` / `Up`**: Move the cursor line by line.
- **`h` / `l` or `Left` / `Right`**: Collapse or Expand the selected directory.
- **`Shift+H` / `Shift+L`**: Collapse or Expand all directories globally.

## Power Navigation

- **`gg`**: Jump to the very top of the list.
- **`G`**: Jump to the very bottom of the list.
- **`zz`**: Center the current selection in the viewport.
- **`Ctrl+u`**: Page up (scrolls half the screen).
- **`Ctrl+d`**: Page down (scrolls half the screen).

## File Hopping

If you want to skip folders and only move between modified files:

- **`d`**: Jump to the next modified file.
- **`u`**: Jump to the previous modified file.

## Pane Switching

In **Split View** (toggled via `v`), you can jump between the Staged and Unstaged panes:

- **`Tab`**: Switch focus between panes.
