use crate::git;
use crossterm::event::KeyCode;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    Search,
    MoveDown,
    MoveUp,
    Collapse,
    CollapseAll,
    Expand,
    ExpandAll,
    NextFile,
    PrevFile,
    Stage,
    Filter,
    Layout,
    Theme,
    SwitchPane,
    Diff,
    Help,
    Back,
    JumpToTop,
    JumpToBottom,
    CenterView,
    PageUp,
    PageDown,
    YankPath,
    VisualMode,
}

#[derive(Clone)]
pub struct KeyConfig {
    pub mappings: HashMap<KeyCode, Action>,
}

impl Default for KeyConfig {
    fn default() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert(KeyCode::Char('q'), Action::Quit);
        mappings.insert(KeyCode::Char('/'), Action::Search);
        mappings.insert(KeyCode::Char('j'), Action::MoveDown);
        mappings.insert(KeyCode::Down, Action::MoveDown);
        mappings.insert(KeyCode::Char('k'), Action::MoveUp);
        mappings.insert(KeyCode::Up, Action::MoveUp);
        mappings.insert(KeyCode::Char('h'), Action::Collapse);
        mappings.insert(KeyCode::Left, Action::Collapse);
        mappings.insert(KeyCode::Char('H'), Action::CollapseAll);
        mappings.insert(KeyCode::Char('l'), Action::Expand);
        mappings.insert(KeyCode::Right, Action::Expand);
        mappings.insert(KeyCode::Char('L'), Action::ExpandAll);
        mappings.insert(KeyCode::Char('d'), Action::NextFile);
        mappings.insert(KeyCode::Char('u'), Action::PrevFile);
        mappings.insert(KeyCode::Char('s'), Action::Stage);
        mappings.insert(KeyCode::Char(' '), Action::Stage);
        mappings.insert(KeyCode::Char('f'), Action::Filter);
        mappings.insert(KeyCode::Char('v'), Action::Layout);
        mappings.insert(KeyCode::Char('t'), Action::Theme);
        mappings.insert(KeyCode::Tab, Action::SwitchPane);
        mappings.insert(KeyCode::Enter, Action::Diff);
        mappings.insert(KeyCode::Char('?'), Action::Help);
        mappings.insert(KeyCode::Esc, Action::Back);
        mappings.insert(KeyCode::Char('G'), Action::JumpToBottom);
        mappings.insert(KeyCode::Char('y'), Action::YankPath);
        mappings.insert(KeyCode::Char('V'), Action::VisualMode);
        KeyConfig { mappings }
    }
}

impl KeyConfig {
    pub fn load() -> Self {
        let mut config = Self::default();
        let custom = git::get_config_regexp("twig.key.");

        for (key, val) in custom {
            let action_name = key.replace("twig.key.", "");
            if let Some(action) = parse_action(&action_name) {
                if let Some(keycode) = parse_keycode(&val) {
                    config.mappings.insert(keycode, action);
                }
            }
        }
        config
    }
}

fn parse_keycode(s: &str) -> Option<KeyCode> {
    if s.len() == 1 {
        return Some(KeyCode::Char(s.chars().next().unwrap()));
    }
    match s.to_lowercase().as_str() {
        "enter" => Some(KeyCode::Enter),
        "tab" => Some(KeyCode::Tab),
        "esc" => Some(KeyCode::Esc),
        "backspace" => Some(KeyCode::Backspace),
        "up" => Some(KeyCode::Up),
        "down" => Some(KeyCode::Down),
        "left" => Some(KeyCode::Left),
        "right" => Some(KeyCode::Right),
        "space" => Some(KeyCode::Char(' ')),
        _ => None,
    }
}

fn parse_action(s: &str) -> Option<Action> {
    match s.to_lowercase().as_str() {
        "quit" => Some(Action::Quit),
        "search" => Some(Action::Search),
        "down" => Some(Action::MoveDown),
        "up" => Some(Action::MoveUp),
        "collapse" => Some(Action::Collapse),
        "collapse_all" => Some(Action::CollapseAll),
        "expand" => Some(Action::Expand),
        "expand_all" => Some(Action::ExpandAll),
        "next_file" => Some(Action::NextFile),
        "prev_file" => Some(Action::PrevFile),
        "stage" => Some(Action::Stage),
        "filter" => Some(Action::Filter),
        "layout" => Some(Action::Layout),
        "theme" => Some(Action::Theme),
        "switch_pane" => Some(Action::SwitchPane),
        "diff" => Some(Action::Diff),
        "help" => Some(Action::Help),
        "back" => Some(Action::Back),
        "top" | "jump_to_top" => Some(Action::JumpToTop),
        "bottom" | "jump_to_bottom" => Some(Action::JumpToBottom),
        "center" | "center_view" => Some(Action::CenterView),
        "page_up" => Some(Action::PageUp),
        "page_down" => Some(Action::PageDown),
        "yank" | "yank_path" => Some(Action::YankPath),
        "visual" | "visual_mode" => Some(Action::VisualMode),
        _ => None,
    }
}
