# How to Release to Homebrew

There are two main ways to publish your tool: the **Modern Automated Way** (recommended) and the **Manual Way**.

## Option 1: The Modern Automated Way (cargo-dist)
This method automates building binaries for all platforms (macOS, Linux) and updating your Homebrew tap automatically when you push a new tag.

### 1. Install cargo-dist
```bash
cargo install cargo-dist
```

### 2. Initialize in your project
Run the init command and select "Homebrew" when asked about installers:
```bash
cargo dist init
```
This will modify `Cargo.toml` and create a `.github/workflows/release.yml`.

### 3. Create a Tap Repository
Create a new public GitHub repository named `homebrew-tap` (or `homebrew-rustle`). It can be empty initially.

### 4. Configure cargo-dist
Update `Cargo.toml` to point to your new specific tap repository:

```toml
[workspace.metadata.dist]
# The preferred name of your installer
formula = "git-twig"
# ...
tap = "yourusername/homebrew-tap"

# ...

brew create https://github.com/yourusername/git-twig/archive/refs/tags/v0.1.0.tar.gz --tap yourusername/twig

# ...

brew tap yourusername/twig
brew install git-twig
```
