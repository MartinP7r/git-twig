use std::{
    io::{self, Stdout},
    process::Command,
};

use ansi_to_tui::IntoText;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

use crate::build_tree_from_git;
use crate::node::FlatNode;
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FilterMode {
    All,
    Modified, // Hides untracked
    Staged,   // Shows only staged
}

impl FilterMode {
    fn next(&self) -> Self {
        match self {
            FilterMode::All => FilterMode::Modified,
            FilterMode::Modified => FilterMode::Staged,
            FilterMode::Staged => FilterMode::All,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            FilterMode::All => "All",
            FilterMode::Modified => "Modified",
            FilterMode::Staged => "Staged",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppLayout {
    Unified,
    Split,
}

impl AppLayout {
    fn next(&self) -> Self {
        match self {
            AppLayout::Unified => AppLayout::Split,
            AppLayout::Split => AppLayout::Unified,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Focus {
    Staged,
    Unstaged,
}

impl Focus {
    fn next(&self) -> Self {
        match self {
            Focus::Staged => Focus::Unstaged,
            Focus::Unstaged => Focus::Staged,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ViewMode {
    Tree,
    Diff,
}

pub fn run(indent: usize, collapse: bool, theme: Theme) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(indent, collapse, theme)?;
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    indent_size: usize,
    collapse: bool,
    staged_nodes: Vec<FlatNode>,
    unstaged_nodes: Vec<FlatNode>,
    unified_nodes: Vec<FlatNode>, // For Unified view
    staged_state: ListState,
    unstaged_state: ListState,
    unified_state: ListState,
    layout: AppLayout,
    filter_mode: FilterMode,
    focus: Focus,
    view_mode: ViewMode,
    diff_content: String,
    diff_scroll: u16,
    theme: Theme,
    max_name_width: usize,
}

impl App {
    fn new(indent_size: usize, collapse: bool, theme: Theme) -> Result<Self> {
        let mut app = App {
            indent_size,
            collapse,
            staged_nodes: Vec::new(),
            unstaged_nodes: Vec::new(),
            unified_nodes: Vec::new(),
            staged_state: ListState::default(),
            unstaged_state: ListState::default(),
            unified_state: ListState::default(),
            layout: AppLayout::Unified, // Default to Unified
            filter_mode: FilterMode::All,
            focus: Focus::Unstaged,
            view_mode: ViewMode::Tree,
            diff_content: String::new(),
            diff_scroll: 0,
            theme,
            max_name_width: 0,
        };
        app.refresh()?;
        Ok(app)
    }

    fn refresh(&mut self) -> Result<()> {
        match self.layout {
            AppLayout::Unified => {
                let (staged, modified) = match self.filter_mode {
                    FilterMode::All => (false, false),
                    FilterMode::Modified => (false, true),
                    FilterMode::Staged => (true, false),
                };

                let tree = build_tree_from_git(staged, modified, false, false)?;
                if let Some(root) = tree {
                    self.unified_nodes = root.flatten(self.indent_size, self.collapse, &self.theme);
                } else {
                    self.unified_nodes = Vec::new();
                }

                // Calculate width
                self.max_name_width = self
                    .unified_nodes
                    .iter()
                    .map(|n| n.connector.chars().count() + n.name.chars().count())
                    .max()
                    .unwrap_or(0);

                // Adjust selection
                Self::adjust_selection(&self.unified_nodes, &mut self.unified_state, true);
            }
            AppLayout::Split => {
                // 1. Fetch Staged
                let staged_tree = build_tree_from_git(true, false, false, false)?;
                if let Some(root) = staged_tree {
                    self.staged_nodes = root.flatten(self.indent_size, self.collapse, &self.theme);
                } else {
                    self.staged_nodes = Vec::new();
                }

                // 2. Fetch All (Unstaged + Untracked + Staged-mixed)
                let all_tree = build_tree_from_git(false, false, false, false)?;
                if let Some(root) = all_tree {
                    let all = root.flatten(self.indent_size, self.collapse, &self.theme);
                    self.unstaged_nodes = all
                        .into_iter()
                        .filter(|n| !n.raw_status.ends_with('+'))
                        .collect();
                } else {
                    self.unstaged_nodes = Vec::new();
                }

                // Calculate max width across BOTH lists for consistent alignment
                let max_staged = self
                    .staged_nodes
                    .iter()
                    .map(|n| n.connector.chars().count() + n.name.chars().count())
                    .max()
                    .unwrap_or(0);
                let max_unstaged = self
                    .unstaged_nodes
                    .iter()
                    .map(|n| n.connector.chars().count() + n.name.chars().count())
                    .max()
                    .unwrap_or(0);

                self.max_name_width = max_staged.max(max_unstaged);

                // Adjust selections
                let staged_active = self.focus == Focus::Staged;
                Self::adjust_selection(&self.staged_nodes, &mut self.staged_state, staged_active);

                let unstaged_active = self.focus == Focus::Unstaged;
                Self::adjust_selection(
                    &self.unstaged_nodes,
                    &mut self.unstaged_state,
                    unstaged_active,
                );
            }
        }
        Ok(())
    }

    fn toggle_filter(&mut self) -> Result<()> {
        if self.layout == AppLayout::Unified {
            self.filter_mode = self.filter_mode.next();
            self.refresh()
        } else {
            Ok(())
        }
    }

    fn toggle_layout(&mut self) -> Result<()> {
        self.layout = self.layout.next();
        self.refresh()
    }

    fn adjust_selection(nodes: &[FlatNode], state: &mut ListState, is_active: bool) {
        if let Some(selected) = state.selected() {
            if selected >= nodes.len() {
                if !nodes.is_empty() {
                    state.select(Some(nodes.len() - 1));
                } else {
                    state.select(None);
                }
            }
        } else if !nodes.is_empty() && is_active {
            // If we just got items and we are active, select 0
            state.select(Some(0));
        } else if !nodes.is_empty() && state.selected().is_none() {
            // Ensure at least 0 is selected if not empty
            state.select(Some(0));
        } else if nodes.is_empty() {
            state.select(None);
        }
    }

    fn toggle_focus(&mut self) {
        self.focus = self.focus.next();
    }

    fn show_diff(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        if let Some(i) = state.selected() {
            if let Some(node) = nodes.get(i) {
                if node.is_dir {
                    return Ok(());
                }

                // Determine git diff arguments
                let mut args = vec!["diff", "--color=always"];

                // If staged, add --cached
                // Check raw status:
                // "A+" -> staged
                // "M+" -> staged
                // "M" -> unstaged
                // "??" -> untracked
                if node.raw_status.contains('+') {
                    args.push("--cached");
                }

                // If untracked, git diff won't show anything unless we use --no-index /dev/null <file>
                // Or just cat the file.
                if node.raw_status == "??" {
                    // For untracked, simpler to just reading the file
                    // But let's try to keep it consistent.
                    // A hack is to diff against /dev/null
                    // But git diff --no-index /dev/null <file> works
                    args.push("--no-index");
                    args.push("/dev/null");
                }

                args.push(&node.full_path);

                let output = Command::new("git").args(args).output()?;

                if output.status.success() || output.status.code() == Some(1) {
                    // diff returns 1 if differences found
                    let content = String::from_utf8_lossy(&output.stdout).to_string();
                    if content.is_empty() && node.raw_status != "??" {
                        self.diff_content = "(No diff or binary file)".to_string();
                    } else {
                        self.diff_content = content;
                    }
                    self.view_mode = ViewMode::Diff;
                    self.diff_scroll = 0;
                } else {
                    let err = String::from_utf8_lossy(&output.stderr);
                    self.diff_content = format!("Error running git diff: {}", err);
                    self.view_mode = ViewMode::Diff;
                }
            }
        }
        Ok(())
    }

    fn close_diff(&mut self) {
        self.view_mode = ViewMode::Tree;
        self.diff_content.clear();
    }

    fn scroll_diff(&mut self, amount: i16) {
        if amount > 0 {
            self.diff_scroll = self.diff_scroll.saturating_add(amount as u16);
        } else {
            self.diff_scroll = self.diff_scroll.saturating_sub((-amount) as u16);
        }
    }

    fn next(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        if nodes.is_empty() {
            return;
        }
        let i = match state.selected() {
            Some(i) => {
                if i >= nodes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    fn previous(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        if nodes.is_empty() {
            return;
        }
        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    nodes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    fn toggle_stage(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        if let Some(i) = state.selected() {
            if let Some(node) = nodes.get(i) {
                if node.raw_status.contains('+') {
                    // Unstage
                    let status = Command::new("git")
                        .args(["restore", "--staged", &node.full_path])
                        .status()?;
                    if status.success() {
                        self.refresh()?;
                    }
                } else {
                    // Stage
                    let status = Command::new("git")
                        .args(["add", &node.full_path])
                        .status()?;
                    if status.success() {
                        self.refresh()?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.view_mode {
                    ViewMode::Tree => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => app.next(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous(),
                        KeyCode::Char('s') | KeyCode::Char(' ') => {
                            let _ = app.toggle_stage();
                        }
                        KeyCode::Char('f') => {
                            if app.layout == AppLayout::Unified {
                                let _ = app.toggle_filter();
                            }
                        }
                        KeyCode::Char('v') => {
                            let _ = app.toggle_layout();
                        }
                        KeyCode::Tab => {
                            if app.layout == AppLayout::Split {
                                app.toggle_focus();
                            }
                        }
                        KeyCode::Enter => {
                            let _ = app.show_diff();
                        }
                        _ => {}
                    },
                    ViewMode::Diff => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => app.close_diff(),
                        KeyCode::Char('j') | KeyCode::Down => app.scroll_diff(1),
                        KeyCode::Char('k') | KeyCode::Up => app.scroll_diff(-1),
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    // If in diff mode, render diff popup
    if app.view_mode == ViewMode::Diff {
        let area = f.size();
        // Parse ANSI codes
        let text = app
            .diff_content
            .into_text()
            .unwrap_or_else(|_| ratatui::text::Text::raw(&app.diff_content));

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(" Diff "))
            .scroll((app.diff_scroll, 0));
        f.render_widget(paragraph, area);
        return;
    }

    match app.layout {
        AppLayout::Unified => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.size());

            let title = format!(
                " git-twig interactive | Filter: {} ",
                app.filter_mode.as_str()
            );
            render_list(
                f,
                &app.theme,
                app.max_name_width,
                &app.unified_nodes,
                &mut app.unified_state,
                chunks[0],
                &title,
                Focus::Unstaged, // Focus not really used here, pass dummy
                Focus::Unstaged, // Match dummy
            );

            // Help for Unified
            let help_text = vec![
                ratatui::text::Span::raw(" [j/k]"),
                ratatui::text::Span::styled(" Nav", Style::default().fg(Color::Gray)),
                ratatui::text::Span::raw("  [Space]"),
                ratatui::text::Span::styled(" Stage/Unstage", Style::default().fg(Color::Magenta)),
                ratatui::text::Span::raw("  [f]"),
                ratatui::text::Span::styled(" Filter", Style::default().fg(Color::Yellow)),
                ratatui::text::Span::raw("  [v]"),
                ratatui::text::Span::styled(" View", Style::default().fg(Color::Green)),
                ratatui::text::Span::raw("  [Enter]"),
                ratatui::text::Span::styled(" Diff", Style::default().fg(Color::Blue)),
                ratatui::text::Span::raw("  [q]"),
                ratatui::text::Span::styled(" Quit", Style::default().fg(Color::Gray)),
            ];
            let help = Paragraph::new(Line::from(help_text))
                .block(Block::default().borders(Borders::ALL).title(" Help "));
            f.render_widget(help, chunks[1]);
        }
        AppLayout::Split => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                    Constraint::Length(3),
                ])
                .split(f.size());

            // --- Staged List (Top) ---
            render_list(
                f,
                &app.theme,
                app.max_name_width,
                &app.staged_nodes,
                &mut app.staged_state,
                chunks[0],
                " Staged Changes ",
                Focus::Staged,
                app.focus,
            );

            // --- Unstaged List (Bottom) ---
            render_list(
                f,
                &app.theme,
                app.max_name_width,
                &app.unstaged_nodes,
                &mut app.unstaged_state,
                chunks[1],
                " Unstaged Changes ",
                Focus::Unstaged,
                app.focus,
            );

            // --- Help for Split ---
            let help_text = vec![
                ratatui::text::Span::raw(" [Tab]"),
                ratatui::text::Span::styled(" Switch Pane", Style::default().fg(Color::Yellow)),
                ratatui::text::Span::raw("  [j/k]"),
                ratatui::text::Span::styled(" Nav", Style::default().fg(Color::Gray)),
                ratatui::text::Span::raw("  [Space]"),
                ratatui::text::Span::styled(" Stage/Unstage", Style::default().fg(Color::Magenta)),
                ratatui::text::Span::raw("  [v]"),
                ratatui::text::Span::styled(" View", Style::default().fg(Color::Green)),
                ratatui::text::Span::raw("  [Enter]"),
                ratatui::text::Span::styled(" Diff", Style::default().fg(Color::Blue)),
                ratatui::text::Span::raw("  [q]"),
                ratatui::text::Span::styled(" Quit", Style::default().fg(Color::Gray)),
            ];
            let help = Paragraph::new(Line::from(help_text))
                .block(Block::default().borders(Borders::ALL).title(" Help "));
            f.render_widget(help, chunks[2]);
        }
    }
}

fn render_list(
    f: &mut ratatui::Frame,
    theme: &Theme,
    max_name_width: usize,
    nodes: &[FlatNode],
    state: &mut ListState,
    area: ratatui::layout::Rect,
    title: &str,
    target_focus: Focus,
    current_focus: Focus,
) {
    let items: Vec<ListItem> = nodes
        .iter()
        .map(|node| {
            let status_indicator = if node.status == '+' {
                ratatui::text::Span::styled("[+]", Style::default().fg(Color::Green))
            } else if node.status == '?' {
                ratatui::text::Span::styled("[?]", Style::default().fg(Color::Red))
            } else if node.status == ' ' {
                ratatui::text::Span::raw("   ")
            } else {
                ratatui::text::Span::styled("[M]", Style::default().fg(Color::Red))
            };

            let connector = ratatui::text::Span::raw(&node.connector);
            let name_style = if node.is_dir {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let name = ratatui::text::Span::styled(&node.name, name_style);

            // Stats bar logic
            let width = node.connector.chars().count() + node.name.chars().count();
            let padding_len = if max_name_width > width {
                max_name_width - width
            } else {
                0
            };
            let padding = " ".repeat(padding_len);

            let mut spans = vec![
                status_indicator,
                ratatui::text::Span::raw(" "),
                connector,
                name,
            ];

            if let Some((added, deleted)) = node.stats {
                let total = added + deleted;
                if total > 0 {
                    spans.push(ratatui::text::Span::raw(format!("{}{}", padding, " | ")));
                    spans.push(ratatui::text::Span::raw(format!("{} ", total)));

                    let max_bar_width = 10;
                    let (plus_chars, minus_chars) = if total <= max_bar_width {
                        (added, deleted)
                    } else {
                        let ratio = added as f64 / total as f64;
                        let p = (ratio * max_bar_width as f64).round() as usize;
                        let m = max_bar_width - p;
                        (p, m)
                    };

                    spans.push(ratatui::text::Span::styled(
                        theme.diff_bar_plus.to_string().repeat(plus_chars),
                        Style::default().fg(Color::Green),
                    ));
                    spans.push(ratatui::text::Span::styled(
                        theme.diff_bar_minus.to_string().repeat(minus_chars),
                        Style::default().fg(Color::Red),
                    ));
                }
            }

            let line = Line::from(spans);
            ListItem::new(line)
        })
        .collect();

    let border_style = if current_focus == target_focus {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 50))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_mode_transitions() {
        let mode = FilterMode::All;
        assert_eq!(mode.as_str(), "All");

        let mode = mode.next();
        assert_eq!(mode, FilterMode::Modified);
        assert_eq!(mode.as_str(), "Modified");

        let mode = mode.next();
        assert_eq!(mode, FilterMode::Staged);
        assert_eq!(mode.as_str(), "Staged");

        let mode = mode.next();
        assert_eq!(mode, FilterMode::All);
    }

    #[test]
    fn test_app_layout_transitions() {
        let layout = AppLayout::Unified;
        assert_eq!(layout.next(), AppLayout::Split);
        assert_eq!(layout.next().next(), AppLayout::Unified);
    }

    #[test]
    fn test_focus_transitions() {
        let focus = Focus::Staged;
        assert_eq!(focus.next(), Focus::Unstaged);
        assert_eq!(focus.next().next(), Focus::Staged);
    }
}
