# git-twig 🌿
*(formerly git-status-tree)*

![git-twig logo](docs/gfx/logo.png)

**git-twig** is a high-performance command line tool written in **Rust** that helps you keep track of changes in your git repository. It visualizes your `git status` as a tree, similar to the `tree` command, but with richer context.

Run `git twig` (or `git tree`) to list all files:
- **Untracked** `(?)`
- **Added** `(A)`
- **Modified** `(M)`
- **Deleted** `(D)`
- **Renamed** `(R)`

It includes a visual diff bar (e.g., `++++----`) to show the scale of changes.

![Example Output](https://github.com/user-attachments/assets/f1f15556-bf95-4fe8-8231-a8858e80f20e)

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
-i, --indent <N>       Set indentation (2-10 spaces)
-c, --collapse         Collapse single-child directories
-h, --help             Show help message
```

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

*Note: Legacy `status-tree.*` keys are supported but deprecated.*

## Development

1. **Build**: `cargo build`
2. **Run**: `cargo run`
3. **Test**: `cargo test`

## License
MIT
