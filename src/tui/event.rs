use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};

use super::app::{App, AppLayout, ViewMode};
use super::ui::ui;
use crate::config::Action;

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
                    let mut action = app.key_config.mappings.get(&key.code).cloned();

                    // Handle pending keys (gg, zz)
                    if let KeyCode::Char(c) = key.code {
                        if let Some(pending) = app.pending_key {
                            match (pending, c) {
                                ('g', 'g') => action = Some(Action::JumpToTop),
                                ('z', 'z') => action = Some(Action::CenterView),
                                _ => {}
                            }
                            app.pending_key = None;
                        } else if c == 'g' || c == 'z' {
                            app.pending_key = Some(c);
                            continue;
                        }
                    } else {
                        app.pending_key = None;
                    }

                    // Handle Ctrl modifiers
                    if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                        if let KeyCode::Char('u') = key.code {
                            action = Some(Action::PageUp);
                        } else if let KeyCode::Char('d') = key.code {
                            action = Some(Action::PageDown);
                        }
                    }

                    match app.view_mode {
                        ViewMode::Tree => {
                            if let Some(action) = action {
                                match action {
                                    Action::Quit => return Ok(()),
                                    Action::Search => {
                                        app.is_typing_search = true;
                                    }
                                    Action::MoveDown => app.next(),
                                    Action::MoveUp => app.previous(),
                                    Action::Collapse => {
                                        let _ = app.collapse_node();
                                    }
                                    Action::CollapseAll => {
                                        let _ = app.collapse_all();
                                    }
                                    Action::Expand => {
                                        let _ = app.expand_node();
                                    }
                                    Action::ExpandAll => {
                                        let _ = app.expand_all();
                                    }
                                    Action::NextFile => app.next_file(),
                                    Action::PrevFile => app.previous_file(),
                                    Action::Stage => {
                                        let _ = app.toggle_stage();
                                    }
                                    Action::Filter => {
                                        if app.layout == AppLayout::Unified {
                                            let _ = app.toggle_filter();
                                        }
                                    }
                                    Action::Layout => {
                                        let _ = app.toggle_layout();
                                    }
                                    Action::Theme => {
                                        let _ = app.toggle_theme();
                                    }
                                    Action::SwitchPane => {
                                        if app.layout == AppLayout::Split {
                                            let _ = app.toggle_focus();
                                        }
                                    }
                                    Action::Diff => {
                                        let _ = app.show_diff();
                                    }
                                    Action::Help => {
                                        app.toggle_help();
                                    }
                                    Action::Back => {
                                        if app.show_help {
                                            app.show_help = false;
                                        } else if app.is_visual_mode {
                                            app.is_visual_mode = false;
                                            app.visual_origin = None;
                                        } else {
                                            app.search_query.clear();
                                            app.reset_selection();
                                        }
                                    }
                                    Action::VisualMode => {
                                        app.toggle_visual_mode();
                                    }
                                    Action::JumpToTop => app.jump_to_top(),
                                    Action::JumpToBottom => app.jump_to_bottom(),
                                    Action::CenterView => {
                                        // CenterView is handled in UI by ensuring ListState offset
                                        // But for now, we don't have a direct way to force offset in ratatui List
                                        // We might need to implement a custom scroll logic if we want true 'zz'
                                    }
                                    Action::PageUp => app.scroll_paging(-15),
                                    Action::PageDown => app.scroll_paging(15),
                                    Action::YankPath => {
                                        let _ = app.yank_path();
                                    }
                                }
                            }
                        }
                        ViewMode::Diff => {
                            if let Some(action) = action {
                                match action {
                                    Action::Quit | Action::Back | Action::Diff => app.close_diff(),
                                    Action::MoveDown => app.scroll_diff(1),
                                    Action::MoveUp => app.scroll_diff(-1),
                                    Action::PageUp => app.scroll_diff(-15),
                                    Action::PageDown => app.scroll_diff(15),
                                    Action::JumpToTop => app.scroll_diff(-1000), // Jump to top of diff
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
