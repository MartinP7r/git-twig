
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use super::app::{App, AppLayout, Focus, ViewMode};
use crate::node::FlatNode;
use crate::theme::Theme;

pub fn ui(f: &mut Frame, app: &mut App) {
    if app.view_mode == ViewMode::Diff {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(if app.is_diff_search || !app.diff_search_query.is_empty() {
                    3
                } else {
                    0
                }),
            ])
            .split(f.size());

        let text = if app.patch_mode {
            let mut lines = Vec::new();
            let raw_lines: Vec<&str> = app.diff_content.lines().collect();
            let selected_hunk_range = app.selected_hunk_idx.and_then(|idx| {
                app.diff_hunks
                    .get(idx)
                    .map(|hunk| (hunk.display_start, hunk.display_end))
            });

            for (i, line) in raw_lines.iter().enumerate() {
                let mut style = Style::default();

                // Basic syntax highlighting
                if line.starts_with('+') {
                    style = style.fg(Color::Green);
                } else if line.starts_with('-') {
                    style = style.fg(Color::Red);
                } else if line.starts_with("@@") {
                    style = style.fg(Color::Cyan);
                }

                // Selection Highlighting
                if let Some((start, end)) = selected_hunk_range {
                    if i >= start && i <= end {
                        style = style
                            .bg(Color::Rgb(50, 50, 50))
                            .add_modifier(Modifier::BOLD);
                    } else {
                        style = style.add_modifier(Modifier::DIM); // Dim others
                    }
                }

                lines.push(Line::from(Span::styled(*line, style)));
            }
            ratatui::text::Text::from(lines)
        } else {
            let mut lines = Vec::new();
            for line in app.diff_content.lines() {
                let mut style = Style::default();
                if line.starts_with('+') {
                    style = style.fg(Color::Green);
                } else if line.starts_with('-') {
                    style = style.fg(Color::Red);
                } else if line.starts_with("@@") {
                    style = style.fg(Color::Cyan);
                }
                lines.push(Line::from(Span::styled(line, style)));
            }
            ratatui::text::Text::from(lines)
        };

        let title = if app.patch_mode {
            " Diff (Patch Mode: Space to Stage, p to Exit) "
        } else {
            " Diff (p to Patch) "
        };

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title(title))
            .scroll((app.diff_scroll, 0));
        f.render_widget(paragraph, chunks[0]);

        if app.is_diff_search || !app.diff_search_query.is_empty() {
            let count_text = if app.diff_matches.is_empty() {
                " (no matches) ".to_string()
            } else {
                format!(
                    " [{}/{}] ",
                    app.current_diff_match.map(|i| i + 1).unwrap_or(0),
                    app.diff_matches.len()
                )
            };

            let search_bar = Paragraph::new(Line::from(vec![
                Span::styled(" Search: ", Style::default().fg(Color::Yellow)),
                Span::raw(&app.diff_search_query),
                Span::styled(count_text, Style::default().fg(Color::DarkGray)),
            ]))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            );
            f.render_widget(search_bar, chunks[1]);

            if app.is_diff_search {
                f.set_cursor(
                    chunks[1].x + 10 + app.diff_search_query.width() as u16,
                    chunks[1].y + 1,
                );
            }
        }
        return;
    }

    match app.layout {
        AppLayout::Unified | AppLayout::Compact => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.size());

            let title = if app.layout == AppLayout::Compact {
                format!(
                    " git-twig interactive | Filter: {} (Compact) ",
                    app.filter_mode.as_str()
                )
            } else {
                format!(
                    " git-twig interactive | Filter: {} ",
                    app.filter_mode.as_str()
                )
            };
            let filtered = App::filter_nodes(&app.unified_nodes, &app.search_query);

            let visual_range = app.get_visual_range();

            render_list(
                f,
                &app.theme,
                app.max_name_width,
                &filtered,
                &mut app.unified_state,
                chunks[0],
                &title,
                Focus::Unstaged,
                Focus::Unstaged,
                visual_range,
                app.layout == AppLayout::EasterEgg,
            );

            render_bottom_bar(f, app, chunks[1]);
        }
        AppLayout::Split => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(45),
                    Constraint::Percentage(45),
                    Constraint::Length(3),
                ])
                .split(f.size());

            let filtered_staged = App::filter_nodes(&app.staged_nodes, &app.search_query);
            let staged_visual_range = if app.focus == Focus::Staged {
                app.get_visual_range()
            } else {
                None
            };
            render_list(
                f,
                &app.theme,
                app.max_name_width,
                &filtered_staged,
                &mut app.staged_state,
                chunks[0],
                " Staged Changes ",
                Focus::Staged,
                app.focus,
                staged_visual_range,
                false,
            );

            let filtered_unstaged = App::filter_nodes(&app.unstaged_nodes, &app.search_query);
            let unstaged_visual_range = if app.focus == Focus::Unstaged {
                app.get_visual_range()
            } else {
                None
            };
            render_list(
                f,
                &app.theme,
                app.max_name_width,
                &filtered_unstaged,
                &mut app.unstaged_state,
                chunks[1],
                " Unstaged Changes ",
                Focus::Unstaged,
                app.focus,
                unstaged_visual_range,
                false,
            );

            render_bottom_bar(f, app, chunks[2]);
        }
        AppLayout::EasterEgg => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.size());

            let filtered = App::filter_nodes(&app.unified_nodes, &app.search_query);
            let visual_range = app.get_visual_range();

            render_list(
                f,
                &app.theme,
                app.max_name_width,
                &filtered,
                &mut app.unified_state,
                chunks[0],
                " 🎄 actual tree view 🎄 ",
                Focus::Unstaged,
                Focus::Unstaged,
                visual_range,
                true,
            );

            render_bottom_bar(f, app, chunks[1]);
        }
    }

    if app.show_help {
        render_help_modal(f, app);
    }

    if app.show_worktrees {
        render_worktree_selector(f, app);
    }

    if app.show_commit_dialog {
        render_commit_dialog(f, app);
    }
}

fn render_help_modal(f: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 60, f.size());
    let help_text = vec![
        Line::from(vec![
            Span::styled(
                "git-twig ",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Green),
            ),
            Span::raw(format!("v{}", env!("CARGO_PKG_VERSION"))),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )]),
        Line::from("  j/k, Up/Down : Move cursor"),
        Line::from("  h/l, Left/Right : Fold/Expand directory"),
        Line::from("  H/L : Fold/Expand All"),
        Line::from("  d/u : Jump to next/prev file"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Actions",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )]),
        Line::from("  Space : Stage/Unstage file or directory"),
        Line::from("  Enter : View inline diff"),
        Line::from("  /     : Search files"),
        Line::from("  f     : Toggle Filter (Unified view)"),
        Line::from("  t      : Cycle Theme (Ascii/Unicode/Rounded/Nerd)"),
        Line::from("  V      : Visual Selection Mode"),
        Line::from("  Tab    : Switch Pane (Split view)"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "General",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )]),
        Line::from("  ?     : Toggle this help"),
        Line::from("  q, Esc: Quit / Back"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Advanced",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )]),
        Line::from("  gg    : Jump to top"),
        Line::from("  G     : Jump to bottom"),
        Line::from("  zz    : Center cursor"),
        Line::from("  u/Ctrl+r : Undo / Redo stage"),
        Line::from("  Alt+V : Easter Egg tree view"),
        Line::from("  y     : Yank path to clipboard"),
        Line::from("  w     : Switch Worktree"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Tips",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )]),
        Line::from("  - Use visual mode for bulk staging"),
        Line::from("  - Compact view helps with deep trees"),
        Line::from("  - Config is loaded from git config"),
        Line::from(""),
        Line::from("  (more content to test scrolling...)"),
        Line::from("  Line 1"),
        Line::from("  Line 2"),
        Line::from("  Line 3"),
        Line::from("  Line 4"),
        Line::from("  Line 5"),
    ];

    let block = Block::default()
        .title(" Help ")
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let visible_height = area.height.saturating_sub(2);
    app.max_help_scroll = (help_text.len() as u16).saturating_sub(visible_height);
    if app.help_scroll > app.max_help_scroll {
        app.help_scroll = app.max_help_scroll;
    }

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .scroll((app.help_scroll, 0))
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(ratatui::widgets::Clear, area); // Clear the background
    f.render_widget(paragraph, area);
}

fn render_commit_dialog(f: &mut Frame, app: &mut App) {
    let area = centered_rect(50, 20, f.size());
    let block = Block::default()
        .title(" Commit Message ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let text = vec![
        Line::from(vec![
            Span::raw(&app.commit_message),
            Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to commit, "),
            Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to cancel"),
        ]),
    ];

    let p = Paragraph::new(text)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_widget(p, area);
}

fn render_worktree_selector(f: &mut Frame, app: &mut App) {
    let area = centered_rect(60, 40, f.size());
    let items: Vec<ListItem> = app
        .worktrees
        .iter()
        .map(|wt| {
            let branch = if wt.branch.is_empty() {
                "(no branch)".to_string()
            } else {
                wt.branch.clone()
            };
            let content = vec![
                Line::from(vec![
                    Span::styled(format!("{:<15}", branch), Style::default().fg(Color::Cyan)),
                    Span::raw(format!(" {}", wt.path)),
                ]),
                Line::from(vec![Span::styled(
                    format!("  HEAD: {}", wt.head),
                    Style::default().fg(Color::DarkGray),
                )]),
            ];
            ListItem::new(content)
        })
        .collect();

    let block = Block::default()
        .title(" Switch Worktree ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_widget(ratatui::widgets::Clear, area);
    f.render_stateful_widget(list, area, &mut app.worktree_state);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn render_bottom_bar(f: &mut Frame, app: &App, area: Rect) {
    if app.is_typing_search || !app.search_query.is_empty() {
        let prefix = if app.is_typing_search {
            "/"
        } else {
            "Search: "
        };
        let text = format!("{}{}", prefix, app.search_query);
        let p = Paragraph::new(text)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(" Search "));
        f.render_widget(p, area);

        if app.is_typing_search {
            f.set_cursor(
                area.x + (prefix.width() + app.search_query.width()) as u16 + 1,
                area.y + 1,
            );
        }
    } else {
        let (added, deleted) = app.global_stats.unwrap_or((0, 0));
        let total = added + deleted;
        let mut stats_spans = vec![Span::raw(format!(
            " {} files changed ",
            app.staged_nodes.len() + app.unstaged_nodes.len()
        ))];

        if total > 0 {
            stats_spans.push(Span::raw("| "));
            stats_spans.push(Span::styled(
                format!("{} ", added),
                Style::default().fg(Color::Green),
            ));

            let max_bar_width = 15;
            let (plus_chars, minus_chars) = if total <= max_bar_width {
                (added, deleted)
            } else {
                let ratio = added as f64 / total as f64;
                let p = (ratio * max_bar_width as f64).round() as usize;
                let m = max_bar_width - p;
                (p, m)
            };

            stats_spans.push(Span::styled(
                app.theme.diff_bar_plus.to_string().repeat(plus_chars),
                Style::default().fg(Color::Green),
            ));
            stats_spans.push(Span::styled(
                app.theme.diff_bar_minus.to_string().repeat(minus_chars),
                Style::default().fg(Color::Red),
            ));
            stats_spans.push(Span::styled(
                format!(" {}", deleted),
                Style::default().fg(Color::Red),
            ));
        }

        let left_content = Line::from(stats_spans);
        let right_content = Line::from(vec![
            Span::raw(" ["),
            Span::styled("?", Style::default().fg(Color::Yellow)),
            Span::raw("] Help "),
        ]);

        let block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Rgb(60, 60, 60)));
        let inner_area = block.inner(area);
        f.render_widget(block, area);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(10)])
            .split(inner_area);

        f.render_widget(Paragraph::new(left_content), layout[0]);
        f.render_widget(
            Paragraph::new(right_content).alignment(ratatui::layout::Alignment::Right),
            layout[1],
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn render_list(
    f: &mut Frame,
    theme: &Theme,
    max_name_width: usize,
    nodes: &[&FlatNode],
    state: &mut ratatui::widgets::ListState,
    area: Rect,
    title: &str,
    target_focus: Focus,
    current_focus: Focus,
    visual_range: Option<(usize, usize)>,
    is_easter_egg: bool,
) {
    let items: Vec<ListItem> = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let mut item_style = Style::default();
            if let Some((start, end)) = visual_range {
                if i >= start && i <= end {
                    item_style = item_style.bg(Color::Rgb(60, 60, 60));
                }
            }

            let mut prefix = String::new();
            if is_easter_egg {
                if node.depth == 0 {
                    // Center the root
                    prefix = " ".repeat(area.width as usize / 2);
                } else {
                    // Alternate sides based on index or name hash
                    let side = (i % 2) == 0;
                    if side {
                        prefix = " ".repeat(4);
                    } else {
                        prefix = " ".repeat(area.width as usize / 2 + 4);
                    }
                }
            }
            let status_indicator = if node.status == '+' {
                Span::styled("[+]", Style::default().fg(Color::Green))
            } else if node.status == '?' {
                Span::styled("[?]", Style::default().fg(Color::Red))
            } else if node.status == ' ' {
                Span::raw("   ")
            } else {
                Span::styled("[M]", Style::default().fg(Color::Red))
            };

            let connector = Span::raw(&node.connector);
            let name_style = if node.is_dir {
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(theme.color_dir)
            } else {
                Style::default().fg(theme.color_file)
            };
            let name = Span::styled(&node.name, name_style);

            let mut spans = vec![
                Span::raw(prefix),
                status_indicator,
                Span::raw(" "),
                connector,
                name,
            ];
            let width = node.connector.width() + node.name.width();
            let padding_len = max_name_width.saturating_sub(width);
            let padding = " ".repeat(padding_len);

            if let Some((added, deleted)) = node.stats {
                let total = added + deleted;
                if total > 0 {
                    spans.push(Span::raw(format!("{}{}", padding, " | ")));
                    spans.push(Span::raw(format!("{} ", total)));

                    let max_bar_width = 10;
                    let (plus_chars, minus_chars) = if total <= max_bar_width {
                        (added, deleted)
                    } else {
                        let ratio = added as f64 / total as f64;
                        let p = (ratio * max_bar_width as f64).round() as usize;
                        let m = max_bar_width - p;
                        (p, m)
                    };

                    spans.push(Span::styled(
                        theme.diff_bar_plus.to_string().repeat(plus_chars),
                        Style::default().fg(Color::Green),
                    ));
                    spans.push(Span::styled(
                        theme.diff_bar_minus.to_string().repeat(minus_chars),
                        Style::default().fg(Color::Red),
                    ));
                }
            }

            ListItem::new(Line::from(spans)).style(item_style)
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
