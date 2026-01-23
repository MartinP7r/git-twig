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

### via Homebrew (Recommended)
```bash
brew tap MartinP7r/homebrew-tap
brew install git-twig
```

### From Source (Rust)
```bash
cargo install git-twig
```

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

## Configuration ⚙️

You can customize **git-twig** behavior via `git config`.

### CLI Example
Set a value directly from your terminal:
```bash
git config --global twig.indent 2
```

### `.gitconfig` Example
Add a `[twig]` section to your `~/.gitconfig` file to manage all settings in one place:

```gitconfig
[twig]
    # Visuals
    indent = 3             # 2-10 spaces
    collapse = false       # Collapse single-child directories
    theme = unicode        # ascii, unicode, nerd, rounded

    # Custom Keybindings
    # Format: key.<action> = <key>
    # Keys: a-z, up, down, left, right, space, enter, tab, esc, backspace
    key.stage = k          # Remap stage/unstage to 'k'
    key.up = i             # Use 'i' for up
    key.down = m           # Use 'm' for down
    key.search = s         # Use 's' for search instead of '/'
```

#### Available Actions for Keybindings:
`quit`, `search`, `down`, `up`, `collapse`, `collapse_all`, `expand`, `expand_all`, `next_file`, `prev_file`, `stage`, `filter`, `layout`, `theme`, `switch_pane`, `diff`, `help`, `back`.

1. **Build**: `cargo build`
2. **Run**: `cargo run`
3. **Test**: `cargo test`

## License
MIT
