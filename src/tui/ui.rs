use ansi_to_tui::IntoText;
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
        let area = f.size();
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
                .constraints([Constraint::Min(0), Constraint::Length(4)])
                .split(f.size());

            let title = format!(
                " git-twig interactive | Filter: {} ",
                app.filter_mode.as_str()
            );
            let filtered = App::filter_nodes(&app.unified_nodes, &app.search_query);

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
            );

            render_bottom_bar(f, app, chunks[1]);
        }
        AppLayout::Split => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(45),
                    Constraint::Percentage(45),
                    Constraint::Length(4),
                ])
                .split(f.size());

            let filtered_staged = App::filter_nodes(&app.staged_nodes, &app.search_query);
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
            );

            let filtered_unstaged = App::filter_nodes(&app.unstaged_nodes, &app.search_query);
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
            );

            render_bottom_bar(f, app, chunks[2]);
        }
    }

    if app.show_help {
        render_help_modal(f, app);
    }
}

fn render_help_modal(f: &mut Frame, _app: &App) {
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
        Line::from("  v     : Toggle Layout (Unified/Split)"),
        Line::from("  t     : Cycle Theme (Ascii/Unicode/Rounded/Nerd)"),
        Line::from("  Tab   : Switch Pane (Split view)"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "General",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )]),
        Line::from("  ?     : Toggle this help"),
        Line::from("  q, Esc: Quit / Back"),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(ratatui::widgets::Clear, area); // Clear the background
    f.render_widget(paragraph, area);
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
        let help_text = vec![
            Span::raw(" [j/k]"),
            Span::styled(" Nav", Style::default().fg(Color::Gray)),
            Span::raw(" [Space]"),
            Span::styled(" Stage", Style::default().fg(Color::Magenta)),
            Span::raw(" [v]"),
            Span::styled(" View", Style::default().fg(Color::Green)),
            Span::raw(" [/]"),
            Span::styled(" Search", Style::default().fg(Color::Cyan)),
            Span::raw(" [?]"),
            Span::styled(" Help", Style::default().fg(Color::Yellow)),
            Span::raw(" [q]"),
            Span::styled(" Quit", Style::default().fg(Color::Gray)),
        ];

        let summary = if let Some((added, deleted)) = app.global_stats {
            format!(
                " | {} files changed, {}(+), {}(-) ",
                app.staged_nodes.len() + app.unstaged_nodes.len(),
                added,
                deleted
            )
        } else {
            String::new()
        };

        let mut lines = vec![Line::from(help_text)];
        if !summary.is_empty() {
            lines.push(Line::from(Span::styled(
                summary,
                Style::default().fg(Color::Gray),
            )));
        }

        let help =
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Hints "));
        f.render_widget(help, area);
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
) {
    let items: Vec<ListItem> = nodes
        .iter()
        .map(|node| {
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
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let name = Span::styled(&node.name, name_style);

            let mut spans = vec![status_indicator, Span::raw(" "), connector, name];
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

            ListItem::new(Line::from(spans))
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
