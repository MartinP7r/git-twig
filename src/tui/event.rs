use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};

use super::app::{App, AppLayout, ViewMode};
use super::ui::ui;

pub fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app.is_typing_search {
                    match key.code {
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                            app.reset_selection();
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                            app.reset_selection();
                        }
                        KeyCode::Esc => {
                            app.is_typing_search = false;
                            app.search_query.clear();
                            app.reset_selection();
                        }
                        KeyCode::Enter => {
                            app.is_typing_search = false;
                        }
                        _ => {}
                    }
                } else {
                    match app.view_mode {
                        ViewMode::Tree => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('/') => {
                                app.is_typing_search = true;
                            }
                            KeyCode::Char('j') | KeyCode::Down => app.next(),
                            KeyCode::Char('k') | KeyCode::Up => app.previous(),
                            KeyCode::Char('h') | KeyCode::Left => {
                                let _ = app.collapse_node();
                            }
                            KeyCode::Char('H') => {
                                let _ = app.collapse_all();
                            }
                            KeyCode::Char('l') | KeyCode::Right => {
                                let _ = app.expand_node();
                            }
                            KeyCode::Char('L') => {
                                let _ = app.expand_all();
                            }
                            KeyCode::Char('d') => app.next_file(),
                            KeyCode::Char('u') => app.previous_file(),
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
                            KeyCode::Char('t') => {
                                let _ = app.toggle_theme();
                            }
                            KeyCode::Tab => {
                                if app.layout == AppLayout::Split {
                                    let _ = app.toggle_focus();
                                }
                            }
                            KeyCode::Enter => {
                                let _ = app.show_diff();
                            }
                            KeyCode::Esc => {
                                app.search_query.clear();
                                app.reset_selection();
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
}
