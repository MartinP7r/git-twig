# git-twig 🌿

![git-twig logo](docs/gfx/logo.png)

**git-twig** is a high-performance command line tool written in **Rust** that helps you keep track of changes in your git repository. It visualizes your `git status` as a tree, similar to the `tree` command, but with richer context.

Run `git twig` (or `git tree`) to list all files:
- **Untracked** `(?)`
- **Added** `(A)`
- **Modified** `(M)`
- **Deleted** `(D)`
- **Renamed** `(R)`

It includes a visual diff bar (e.g., `++++----`) to show the scale of changes.

## Installation

### From Source (Rust)
```bash
cargo install --path .
```
*(Homebrew Tap coming soon!)*

## Usage

Start using it in your terminal:
```bash
git-twig
# or if you aliased it:
git twig
```

### Options
```text
-I, --interactive      Start interactive TUI mode
-o, --open             Open modified files in $EDITOR
-s, --staged-only      Show only staged files
-m, --modified-only    Hide untracked files
    --untracked-only   Show only untracked files
-i, --indent <N>       Set indentation (2-10 spaces)
-c, --collapse         Collapse single-child directories
    --theme <T>        Set visual theme (ascii, unicode, nerd)
    --simple-icons     Use generic icons instead of rich Nerd Font icons
-h, --help             Show help message
```

## Interactive Mode 🕹️
Run `git-twig -I` to enter the interactive TUI.
- **Navigate**: `j`/`k` (Vim-style) or Arrow keys.
- **Stage/Unstage**: `<Space>` to toggle status for files or entire folders.
- **Folding**: `h`/`l` to collapse/expand folders. `Shift+H`/`Shift+L` for global fold/unfold.
- **Diff View**: `<Enter>` to view inline diffs with syntax highlighting.
- **Search**: `/` to fuzzy search files.
- **View Toggle**: `v` to switch between **Unified** and **Split** layouts.
- **Theme Cycle**: `t` to quickly switch between visual styles.

## Configuration

You can configure defaults via `git config`:

**Indentation** (Default: 3)
```bash
git config --global twig.indent 2
```

**Collapse Directories** (Default: false)
```bash
git config --global twig.collapse true
```

**Theme** (Default: unicode)
```bash
# Options: ascii, unicode, nerd
git config --global twig.theme nerd
```

1. **Build**: `cargo build`
2. **Run**: `cargo run`
3. **Test**: `cargo test`

## License
MIT
