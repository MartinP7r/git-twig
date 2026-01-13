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
formula = "git-status-tree"
# The GitHub repo where the Formula should be pushed
tap = "yourusername/homebrew-tap"
# ... other config ...
```

### 5. Push and Tag
When you are ready to release:
1. Commit the changes.
2. Push a tag starting with `v` (e.g., `v0.1.0`).

```bash
git tag v0.1.0
git push origin v0.1.0
```
GitHub Actions will kick in, build the binaries, and automatically open a Pull Request to your `homebrew-tap` repo with the updated Formula. Merge that PR, and you're live!

---

## Option 2: The Manual Way (Quick & Dirty)
Best for testing or if you don't want to set up CI yet.

### 1. Create a Tap
Creates a local tap repository at `/usr/local/Homebrew/Library/Taps/yourusername/homebrew-rustle`.
```bash
brew tap-new yourusername/rustle
```

### 2. Create the Formula
Point `brew` to your GitHub release tarball (you need to create a Release on GitHub manually first with the `.tar.gz` of your binary).

```bash
brew create https://github.com/yourusername/git-status-tree/archive/refs/tags/v0.1.0.tar.gz --tap yourusername/rustle
```

### 3. Edit the Formula
It will open `git-status-tree.rb` in an editor. Ensure the `install` section looks like this:

```ruby
def install
  system "cargo", "install", *std_cargo_args
end
```

### 4. Commit and Push
Go to the tap directory and push it to GitHub:
```bash
cd $(brew --repo yourusername/rustle)
git add .
git commit -m "Add git-status-tree v0.1.0"
git push origin main
```

### 5. Install
Users can now install with:
```bash
brew tap yourusername/rustle
brew install git-status-tree
```
