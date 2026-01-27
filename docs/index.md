---
title: Home
hide:
  - navigation
  - toc
---

<div class="landing-page">
    <div class="bg-mesh"></div>

    <section class="hero">
        <div class="container hero-content">
            <h1>Grow your git workflow<br>into something beautiful.</h1>
            <p>A high-performance, interactive git status visualizer that turns your chaotic diffs into an actionable tree view.</p>
            <div style="display: flex; justify-content: center; gap: 1rem;">
                <a href="introduction.md" class="btn btn-primary">Get Started</a>
                <a href="https://github.com/MartinP7r/git-twig" class="btn btn-secondary">
                    <span style="font-size: 1.2rem; margin-right: 0.5rem;">★</span> Star on GitHub
                </a>
            </div>
        </div>

        <div class="terminal-window">
            <div class="terminal-titlebar">
                <div class="dot dot-red"></div>
                <div class="dot dot-yellow"></div>
                <div class="dot dot-green"></div>
                <span style="color: #666; font-size: 0.8rem; margin-left: 10px;">User@MacBook-Pro: ~/projects/git-twig</span>
            </div>

            <div class="terminal-screen">
                <div class="showcase-tabs">
                    <button class="tab-btn active" onclick="switchView(event, 'unified')">Interactive</button>
                    <button class="tab-btn" onclick="switchView(event, 'split')">Split View</button>
                    <button class="tab-btn" onclick="switchView(event, 'search')">Fuzzy Search</button>
                </div>

                <div id="tui-container" class="tui-block" style="flex: 1; display: flex; flex-direction: column;">
                    <div class="tui-header">
                        <span id="tui-header-title">git-twig interactive | Filter: all</span>
                        <span>[ master ↑1 · +2 ~4 -1 ]</span>
                    </div>

                    <div class="tui-content-area">
                        <!-- Unified View -->
                        <div id="view-unified" class="tui-view active">
                            <div class="tui-line">🌿 <span class="t-green">git-twig</span></div>
                            <div class="tui-line">├── <span class="t-green">[+]</span> src/</div>
                            <div class="tui-line">│   ├── <span class="t-green">[+]</span> main.rs <span class="t-green"> | 8 ++++----</span></div>
                            <div class="tui-line">│   └── <span class="t-yellow">(M)</span> node.rs <span class="t-green"> | 4 ++--</span></div>
                            <div class="tui-line">├── <span class="t-blue">(?)</span> docs/ <span class="t-gray">[folder]</span></div>
                            <div class="tui-line">│   └── <span class="t-blue">(?)</span> roadmap.md</div>
                            <div class="tui-line">└── <span class="t-gray">(D)</span> deprecated.txt</div>
                        </div>

                        <!-- Split View -->
                        <div id="view-split" class="tui-view">
                            <div class="tui-pane active" style="margin-bottom: 10px;">
                                <div class="tui-block-title"> Staged Changes </div>
                                <div class="tui-line">>> <span class="t-green">[+]</span> src/main.rs <span class="t-green"> | 8 ++++----</span></div>
                            </div>
                            <div class="tui-pane">
                                <div class="tui-block-title"> Unstaged Changes </div>
                                <div class="tui-line">   <span class="t-yellow">(M)</span> src/node.rs <span class="t-green"> | 4 ++--</span></div>
                                <div class="tui-line">   <span class="t-blue">(?)</span> docs/roadmap.md</div>
                            </div>
                        </div>

                        <!-- Search View -->
                        <div id="view-search" class="tui-view">
                            <div class="tui-line">🌿 <span class="t-green">git-twig</span></div>
                            <div class="tui-line">├── <span class="t-yellow">(M)</span> src/</div>
                            <div class="tui-line">│   └── <span class="t-yellow">(M)</span> <span class="t-cursor">node.rs</span> <span class="t-green"> | 4 ++--</span></div>
                            <div class="tui-line">└── <span class="t-blue">(?)</span> docs/</div>
                        </div>
                    </div>

                    <div class="tui-footer" id="footer-content">
                        <span class="tui-key">[j/k]</span> Nav <span class="tui-key">[Space]</span> <span class="tui-val">Stage</span> <span class="tui-key">[v]</span> View <span class="tui-key">[/]</span> Search <span class="tui-key">[Enter]</span> Diff
                    </div>
                </div>
            </div>
        </div>
    </section>

    <script>
        function switchView(event, view) {
            // Update tabs
            document.querySelectorAll('.tab-btn').forEach(btn => btn.classList.remove('active'));
            event.currentTarget.classList.add('active');

            // Update views
            document.querySelectorAll('.tui-view').forEach(v => v.classList.remove('active'));
            document.getElementById('view-' + view).classList.add('active');

            // Update contents
            const header = document.getElementById('tui-header-title');
            const footer = document.getElementById('footer-content');

            if (view === 'unified') {
                header.innerText = 'git-twig interactive | Filter: all';
                footer.innerHTML = '<span class="tui-key">[j/k]</span> Nav <span class="tui-key">[Space]</span> <span class="tui-val">Stage</span> <span class="tui-key">[v]</span> View <span class="tui-key">[/]</span> Search <span class="tui-key">[Enter]</span> Diff';
            } else if (view === 'split') {
                header.innerText = 'git-twig interactive | Split Mode';
                footer.innerHTML = '<span class="tui-key">[Tab]</span> Switch Pane <span class="tui-key">[j/k]</span> Nav <span class="tui-key">[v]</span> View <span class="tui-key">[q]</span> Quit';
            } else if (view === 'search') {
                header.innerText = 'git-twig interactive | Search';
                footer.innerHTML = '<span class="t-yellow">/</span><span class="tui-val">node</span><span class="tui-cursor"> </span>' + 
                                 '<span style="margin-left: 20px;" class="t-gray">(matching 1 file)</span>';
            }
        }
    </script>

    <section id="features" class="features container">
        <div class="feature-card">
            <div class="feature-icon">🕹️</div>
            <h3>Interactive TUI</h3>
            <p>Full terminal interface with Vim-style navigation, fuzzy search, and interactive staging/unstaging.</p>
        </div>
        <div class="feature-card">
            <div class="feature-icon">🌳</div>
            <h3>Natural Tree</h3>
            <p>See changes in a beautiful, nested directory structure instead of a confusing flat list.</p>
        </div>
        <div class="feature-card">
            <div class="feature-icon">🎨</div>
            <h3>Visual Themes</h3>
            <p>Choose between ASCII, Unicode, or rich Nerd Font icons to match your terminal aesthetic.</p>
        </div>
        <div class="feature-card">
            <div class="feature-icon">📊</div>
            <h3>Diff Alignment</h3>
            <p>Clean, vertically aligned status indicators and diff bars make it easy to scan massive repositories.</p>
        </div>
        <div class="feature-card">
            <div class="feature-icon">✏️</div>
            <h3>Quick Edit</h3>
            <p>Launch your favorite editor directly from the CLI or TUI to address changes instantly.</p>
        </div>
        <div class="feature-card">
            <div class="feature-icon">🦀</div>
            <h3>Rust Speed</h3>
            <p>Near-instant execution powered by Rust, ensuring a snappy experience even in the largest monorepos.</p>
        </div>
    </section>

    <section id="install" style="padding: 4rem 0; text-align: center;">
        <h2 style="font-size: 2.5rem; margin-bottom: 2rem;">Ready to grow?</h2>
        <div class="terminal" style="max-width: 600px; margin: 0 auto;">
            <div class="terminal-header">
                <span class="dot dot-red"></span>
                <span class="dot dot-yellow"></span>
                <span class="dot dot-green"></span>
                <span style="color: #666; font-size: 0.8rem; margin-left: 10px;">Installation</span>
            </div>
            <div class="terminal-body">
                <div class="line"><span class="t-gray"># Install via Homebrew</span></div>
                <div class="line"><span class="t-green">$</span> brew tap MartinP7r/tap</div>
                <div class="line"><span class="t-green">$</span> brew install git-twig</div>
                <div class="line"></div>
                <div class="line"><span class="t-gray"># Or via cargo</span></div>
                <div class="line"><span class="t-green">$</span> cargo install git-twig</div>
                <div class="line"></div>
                <div class="line"><span class="t-gray"># Start using it</span></div>
                <div class="line"><span class="t-green">$</span> git twig</div>
            </div>
        </div>
    </section>

    <footer style="padding: 4rem 0; border-top: 1px solid var(--border); text-align: center; color: var(--text-muted);">
        <p>&copy; 2026 git-twig contributors. Released under the MIT License.</p>
        <p style="margin-top: 1rem;">Grown with 🦀 and 🌿</p>
    </footer>
</div>
