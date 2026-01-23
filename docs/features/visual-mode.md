---
title: "Visual Selection"
description: "Perform bulk actions on multiple files at once."
---

Efficiency is at the heart of git-twig. **Visual Selection Mode** allows you to select a range of files or directories and perform actions on them in one go.

## Entering Visual Mode

Press **`Shift+V`** (uppercase V) while in the interactive TUI. 

You'll notice the current line is highlighted. As you move the cursor using `j`, `k`, `gg`, or `G`, the selection range will expand or contract, following your movement.

## Bulk Actions

Once you have a range selected, you can trigger the following actions:

- **`Space`**: Stage or Unstage all items in the selection.
- **`h` / `l`**: Fold or Unfold all directories within the range.
- **`y`**: Yank (copy) the relative paths of all selected items to your clipboard, separated by newlines.

## Exiting

You can exit Visual Mode at any time by pressing **`Esc`** or **`Shift+V`** again. Performing a bulk action will also automatically return you to normal mode.
