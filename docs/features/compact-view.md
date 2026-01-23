---
title: "Compact View"
description: "Collapse empty directory chains to maximize vertical space."
---

Modern projects often have deep nesting (e.g., `src/main/java/com/company/app`). In a standard tree view, these empty "bridge" folders take up valuable vertical space.

**Compact Path Rendering** solves this by collapsing these chains into a single line.

## Toggling Compact Mode

You can cycle through view modes by pressing **`v`**:

1.  **Unified**: A single tree of all changes.
2.  **Split**: Side-by-side panes for Staged and Unstaged.
3.  **Compact**: A condensed tree that collapses empty folders.

## Visual Example

**Standard View:**
```text
└─ src
   └─ tui
      └─ app.rs
```

**Compact View:**
```text
└─ src・tui
   └─ app.rs
```

The `・` separator is used to indicate a collapsed directory chain. This behavior is automatic for any directory that contains only one subdirectory with active changes.
