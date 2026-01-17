use std::{
    io::{self, Stdout},
    process::Command,
};

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

pub fn run(indent: usize, collapse: bool) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(indent, collapse)?;
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
    nodes: Vec<FlatNode>,
    state: ListState,
    filter_mode: FilterMode,
}

impl App {
    fn new(indent_size: usize, collapse: bool) -> Result<Self> {
        let mut app = App {
            indent_size,
            collapse,
            nodes: Vec::new(),
            state: ListState::default(),
            filter_mode: FilterMode::All,
        };
        app.refresh()?;
        Ok(app)
    }

    fn refresh(&mut self) -> Result<()> {
        let (staged, modified) = match self.filter_mode {
            FilterMode::All => (false, false),
            FilterMode::Modified => (false, true),
            FilterMode::Staged => (true, false),
        };

        let tree = build_tree_from_git(staged, modified)?;
        if let Some(root) = tree {
            self.nodes = root.flatten(self.indent_size, self.collapse);
            
            // Adjust selection if out of bounds
            if let Some(selected) = self.state.selected() {
                if selected >= self.nodes.len() {
                    if !self.nodes.is_empty() {
                         self.state.select(Some(self.nodes.len() - 1));
                    } else {
                         self.state.select(None);
                    }
                }
            } else if !self.nodes.is_empty() {
                 self.state.select(Some(0));
            } else {
                 self.state.select(None);
            }
        } else {
            self.nodes = Vec::new();
            self.state.select(None);
        }
        Ok(())
    }

    fn toggle_filter(&mut self) -> Result<()> {
        self.filter_mode = self.filter_mode.next();
        self.refresh()
    }

    fn next(&mut self) {
        if self.nodes.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.nodes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        if self.nodes.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.nodes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn stage(&mut self) -> Result<()> {
        if let Some(i) = self.state.selected() {
            if let Some(node) = self.nodes.get(i) {
                // Skip if already staged removed (D+)
                if node.raw_status == "D+" {
                    return Ok(());
                }

                let status = Command::new("git")
                    .args(["add", &node.full_path])
                    .status()?;
                if status.success() {
                    self.refresh()?;
                }
            }
        }
        Ok(())
    }

    fn unstage(&mut self) -> Result<()> {
        if let Some(i) = self.state.selected() {
            if let Some(node) = self.nodes.get(i) {
                // For unstaging, we use reset for files or restore --staged
                // restore --staged is modern
                let status = Command::new("git")
                    .args(["restore", "--staged", &node.full_path])
                    .status()?;
                if status.success() {
                    self.refresh()?;
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
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char('s') | KeyCode::Char(' ') => {
                        let _ = app.stage();
                    }
                    KeyCode::Char('u') => {
                        let _ = app.unstage();
                    }
                    KeyCode::Char('f') => {
                        let _ = app.toggle_filter();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.size());

    let items: Vec<ListItem> = app
        .nodes
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

            let line = Line::from(vec![
                status_indicator,
                ratatui::text::Span::raw(" "),
                connector,
                name,
            ]);
            ListItem::new(line)
        })
        .collect();

    let title = format!(" git-twig interactive | Filter: {} ", app.filter_mode.as_str());

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 50))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let help_text = vec![
        ratatui::text::Span::raw(" [j/k]"),
        ratatui::text::Span::styled(" Nav", Style::default().fg(Color::Gray)),
        ratatui::text::Span::raw("  [Space]"),
        ratatui::text::Span::styled(" Stage", Style::default().fg(Color::Green)),
        ratatui::text::Span::raw("  [u]"),
        ratatui::text::Span::styled(" Unstage", Style::default().fg(Color::Red)),
        ratatui::text::Span::raw("  [f]"),
        ratatui::text::Span::styled(" Filter", Style::default().fg(Color::Yellow)),
        ratatui::text::Span::raw("  [q]"),
        ratatui::text::Span::styled(" Quit", Style::default().fg(Color::Gray)),
    ];
    let help = Paragraph::new(Line::from(help_text))
        .block(Block::default().borders(Borders::ALL).title(" Help "));
    f.render_widget(help, chunks[1]);
}
