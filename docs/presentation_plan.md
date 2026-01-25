# git-twig Presentation Plan 🌿

## 1. Title Slide
- **Title**: git-twig
- **Subtitle**: Git Status, but Actionable.
- **Visual**: Large ASCII/Unicode tree structure or the logo.
- **Speaker Notes**: "Hi everyone, I'm [Your Name], and I want to show you how to stop reading `git status` and start interacting with it."

## 2. The Problem
- **Headline**: `git status` is passive.
- **Content**:
    - Screenshot of a messy `git status` output.
    - Bullet points:
        - "Read-only list"
        - "Need to copy-paste paths"
        - "Hard to visualize structure"
- **Speaker Notes**: "We run this command 100 times a day. It tells us WHAT changed, but doesn't let us DO anything about it."

## 3. The Solution: git-twig
- **Headline**: Turns your status into a workspace.
- **Key Concepts**:
    - **Visual**: Tree structure (like `tree` command).
    - **Interactive**: Vim-style navigation.
    - **Fast**: Written in Rust.
- **Visual**: Split screen comparison: `git status` (left) vs `git twig` (right).

## 4. Live Demo Flow (The Core)
*Use a screen recording or live terminal for this section.*

### Scene 1: Navigation
- **Action**: Launch `git-twig`.
- **Action**: Move up/down with `j`/`k`. Fold a directory with `h`.
- **Talking Point**: "Muscle memory. If you know Vim, you already know git-twig."

### Scene 2: The "Aha!" Moment - Staging
- **Action**: Hover over a file. Press `<Space>`.
- **Visual**: File turns green (staged).
- **Action**: Hover over a folder. Press `<Space>`.
- **Visual**: Entire folder stages instantly.
- **Talking Point**: "No more `git add This/Is/A/Very/Long/Path.rs`. Just point and shoot."

### Scene 3: Context
- **Action**: Press `<Enter>` on a file.
- **Visual**: Inline Diff appears.
- **Talking Point**: "Review changes right here. No context switching to another tool."

## 5. Under the Hood
- **Headline**: Built with Rust & Ratatui.
- **Tech Stack**:
    - **Language**: Rust (Memory safety, speed).
    - **TUI Library**: Ratatui (The engine).
    - **Git Integration**: Direct git command execution for reliability.

## 6. Roadmap & Future
- **Headline**: What's Next?
- **Items**:
    - **Commit**: Press `c` to commit directly from the UI.
    - **Interactive Patch Staging**: Stage specific lines/hunks from the inline diff.
    - **Remote View**: See upstream changes.
    - **Custom Actions**: Run lints/tests on selected files.

## 7. Call to Action
- **Headline**: Try it today.
- **Install**:
    ```bash
    brew tap MartinP7r/tap
    brew install git-twig
    ```
- **Repo**: github.com/MartinP7r/git-twig
- **Closing**: "Stop reading your history. Start shaping it."

## 8. Gemini Creation Instructions

To generate this presentation using Gemini in Google Slides:

## 8. Gemini Creation Instructions

Since Gemini in Slides generates one slide at a time, use these individual prompts for each slide:

**Slide 1: Title**
> Create a title slide for "git-twig" with the subtitle "Git Status, but Actionable". Use a dark, technical theme with a code-inspired background.

**Slide 2: The Problem**
> Create a slide titled "The Problem: Passive Status". Bullet points: "git status is read-only", "Constant context switching", "Hard to visualize structure". Visual: A messy terminal screenshot.

**Slide 3: The Solution**
> Create a slide titled "The Solution: git-twig". Key points: "Visual tree structure", "Vim-style navigation", "Instant staging". Theme: Modern, clean, green accents.

**Slide 4: Demo / Core Workflow**
> Create a slide titled "The Workflow". Show a flow chart or 3 steps: 1. Navigate (j/k), 2. Stage (Space), 3. Context (Enter for Diff).

**Slide 5: Technical Details**
> Create a slide titled "Built for Speed". Bullet points: "Written in Rust", "Ratatui TUI Engine", "Zero dependencies". Visual: Rust logo or gears.

**Slide 6: Roadmap**
> Create a slide titled "Roadmap". List items: "Interactive Patch Staging", "Commit UI", "Remote Branches View".

**Slide 7: Call to Action**
> Create a slide titled "Try it Today". Large text code block: "brew install git-twig". Subtext: "Star us on GitHub".
