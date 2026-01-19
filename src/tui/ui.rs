use ansi_to_tui::IntoText;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

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
                .constraints([Constraint::Min(0), Constraint::Length(3)])
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
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                    Constraint::Length(3),
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
    } else {
        let help_text = if app.layout == AppLayout::Unified {
            vec![
                Span::raw(" [j/k]"),
                Span::styled(" Nav", Style::default().fg(Color::Gray)),
                Span::raw(" [h/l]"),
                Span::styled(" Fold", Style::default().fg(Color::Gray)),
                Span::raw(" [H/L]"),
                Span::styled(" FoldAll", Style::default().fg(Color::Gray)),
                Span::raw(" [d/u]"),
                Span::styled(" Jump", Style::default().fg(Color::Gray)),
                Span::raw("  [Space]"),
                Span::styled(" Stage/Unstage", Style::default().fg(Color::Magenta)),
                Span::raw("  [f]"),
                Span::styled(" Filter", Style::default().fg(Color::Yellow)),
                Span::raw("  [v]"),
                Span::styled(" View", Style::default().fg(Color::Green)),
                Span::raw("  [/]"),
                Span::styled(" Search", Style::default().fg(Color::Cyan)),
                Span::raw("  [t]"),
                Span::styled(" Theme", Style::default().fg(Color::LightMagenta)),
                Span::raw("  [Enter]"),
                Span::styled(" Diff", Style::default().fg(Color::Blue)),
                Span::raw("  [q]"),
                Span::styled(" Quit", Style::default().fg(Color::Gray)),
            ]
        } else {
            vec![
                Span::raw(" [Tab]"),
                Span::styled(" Switch Pane", Style::default().fg(Color::Yellow)),
                Span::raw("  [j/k]"),
                Span::styled(" Nav", Style::default().fg(Color::Gray)),
                Span::raw(" [h/l]"),
                Span::styled(" Fold", Style::default().fg(Color::Gray)),
                Span::raw(" [H/L]"),
                Span::styled(" FoldAll", Style::default().fg(Color::Gray)),
                Span::raw(" [d/u]"),
                Span::styled(" Jump", Style::default().fg(Color::Gray)),
                Span::raw("  [Space]"),
                Span::styled(" Stage/Unstage", Style::default().fg(Color::Magenta)),
                Span::raw("  [v]"),
                Span::styled(" View", Style::default().fg(Color::Green)),
                Span::raw("  [/]"),
                Span::styled(" Search", Style::default().fg(Color::Cyan)),
                Span::raw("  [t]"),
                Span::styled(" Theme", Style::default().fg(Color::LightMagenta)),
                Span::raw("  [Enter]"),
                Span::styled(" Diff", Style::default().fg(Color::Blue)),
                Span::raw("  [q]"),
                Span::styled(" Quit", Style::default().fg(Color::Gray)),
            ]
        };
        let help = Paragraph::new(Line::from(help_text))
            .block(Block::default().borders(Borders::ALL).title(" Help "));
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

            let width = node.connector.chars().count() + node.name.chars().count();
            let padding_len = max_name_width.saturating_sub(width);
            let padding = " ".repeat(padding_len);

            let mut spans = vec![status_indicator, Span::raw(" "), connector, name];

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
