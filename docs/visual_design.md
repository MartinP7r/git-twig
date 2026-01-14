# Visual Design Proposal

To improve the visual appeal of **git-twig** while maintaining compatibility, we propose a tiered theming system.

## 1. Status Indicators
Replacing text codes `(M)`, `(A)` with icons.

| Status | Minimal | Emoji | Nerd Font |
| :--- | :--- | :--- | :--- |
| **Modified** | `*` | 📝 | `` or `` |
| **Added** | `+` | ✨ | `` |
| **Deleted** | `x` | 💀 | `` |
| **Renamed** | `>` | 🚚 | `` |
| **Untracked** | `?` | 🆕 | `` |

## 2. High-Res Diff Bars
Using Unicode Block Elements to create smoother "progress bar" style diff stats.

*   **Solid Blocks**: `████░░░`
*   **Lower Blocks**: `▄▄▄▄▅▅▅`
*   **Circles**: `●●●○○○`

**Mockup:**
```text
src/main.rs  | 12 ◼◼◼◼◼◼◼◻◻
src/node.rs  |  5 ◼◼◼◻
```

## 3. Tree Connectors
Modernizing the tree structure lines.

*   **Standard**:
    ```text
    ├─ src
    └─ main.rs
    ```
*   **Rounded** (Unicode):
    ```text
    ├── src
    ╰── main.rs
    ```

## 4. File & Folder Icons
For users with "Nerd Fonts" installed.

*   **Rust**: `` or `🦀`
*   **Ruby**: `` or `💎`
*   **Folder**: `` or `📂`
*   **Git**: ``

## Proposed Themes
Implemented via a `--theme` flag or config:

1.  **`ascii`** (Default safety): Standard characters, safest for all terminals.
2.  **`unicode`** (Modern Standard): Rounded corners, block diff bars, unicode bullets.
3.  **`nerd`** (Power User): Full file icons, folder icons, status glyphs.
