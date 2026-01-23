use anyhow::Result;
use ratatui::widgets::ListState;
use std::collections::HashSet;
use unicode_width::UnicodeWidthStr;

use crate::config::KeyConfig;
use crate::git;
use crate::node::FlatNode;
use crate::theme::{Theme, ThemeType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterMode {
    All,
    Modified, // Hides untracked
    Staged,   // Shows only staged
}

impl FilterMode {
    pub fn next(&self) -> Self {
        match self {
            FilterMode::All => FilterMode::Modified,
            FilterMode::Modified => FilterMode::Staged,
            FilterMode::Staged => FilterMode::All,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            FilterMode::All => "All",
            FilterMode::Modified => "Modified",
            FilterMode::Staged => "Staged",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppLayout {
    Unified,
    Split,
}

impl AppLayout {
    pub fn next(&self) -> Self {
        match self {
            AppLayout::Unified => AppLayout::Split,
            AppLayout::Split => AppLayout::Unified,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Staged,
    Unstaged,
}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::Staged => Focus::Unstaged,
            Focus::Unstaged => Focus::Staged,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Tree,
    Diff,
}

pub struct App {
    pub indent_size: usize,
    pub collapse: bool,
    pub staged_nodes: Vec<FlatNode>,
    pub unstaged_nodes: Vec<FlatNode>,
    pub unified_nodes: Vec<FlatNode>,
    pub collapsed_paths: HashSet<String>,
    pub staged_state: ListState,
    pub unstaged_state: ListState,
    pub unified_state: ListState,
    pub layout: AppLayout,
    pub filter_mode: FilterMode,
    pub focus: Focus,
    pub search_query: String,
    pub is_typing_search: bool,
    pub view_mode: ViewMode,
    pub diff_content: String,
    pub diff_scroll: u16,
    pub theme: Theme,
    pub theme_type: ThemeType,
    pub max_name_width: usize,
    pub show_help: bool,
    pub global_stats: Option<(usize, usize)>,
    pub key_config: KeyConfig,
}

impl App {
    pub fn new(indent_size: usize, collapse: bool, theme: Theme) -> Result<Self> {
        let mut app = App {
            indent_size,
            collapse,
            staged_nodes: Vec::new(),
            unstaged_nodes: Vec::new(),
            unified_nodes: Vec::new(),
            collapsed_paths: HashSet::new(),
            staged_state: ListState::default(),
            unstaged_state: ListState::default(),
            unified_state: ListState::default(),
            layout: AppLayout::Unified,
            filter_mode: FilterMode::All,
            focus: Focus::Unstaged,
            search_query: String::new(),
            is_typing_search: false,
            view_mode: ViewMode::Tree,
            diff_content: String::new(),
            diff_scroll: 0,
            theme: theme.clone(),
            theme_type: ThemeType::Unicode, // Will be set by determine_theme usually
            max_name_width: 0,
            show_help: false,
            global_stats: None,
            key_config: KeyConfig::load(),
        };
        app.refresh()?;
        Ok(app)
    }

    pub fn filter_nodes<'a>(nodes: &'a [FlatNode], query: &str) -> Vec<&'a FlatNode> {
        if query.is_empty() {
            nodes.iter().collect()
        } else {
            let q = query.to_lowercase();
            nodes
                .iter()
                .filter(|n| {
                    n.name.to_lowercase().contains(&q) || n.full_path.to_lowercase().contains(&q)
                })
                .collect()
        }
    }

    pub fn refresh(&mut self) -> Result<()> {
        match self.layout {
            AppLayout::Unified => {
                let (staged, modified) = match self.filter_mode {
                    FilterMode::All => (false, false),
                    FilterMode::Modified => (false, true),
                    FilterMode::Staged => (true, false),
                };

                let tree = git::build_tree_from_git(staged, modified, false)?;
                if let Some(root) = tree {
                    self.unified_nodes = root.flatten(
                        self.indent_size,
                        self.collapse,
                        &self.theme,
                        &self.collapsed_paths,
                    );
                } else {
                    self.unified_nodes = Vec::new();
                }

                self.max_name_width = self
                    .unified_nodes
                    .iter()
                    .map(|n| n.connector.width() + n.name.width())
                    .max()
                    .unwrap_or(0);

                Self::adjust_selection(&self.unified_nodes, &mut self.unified_state, true);
            }
            AppLayout::Split => {
                let staged_tree = git::build_tree_from_git(true, false, false)?;
                if let Some(root) = staged_tree {
                    self.staged_nodes = root.flatten(
                        self.indent_size,
                        self.collapse,
                        &self.theme,
                        &self.collapsed_paths,
                    );
                } else {
                    self.staged_nodes = Vec::new();
                }

                let all_tree = git::build_tree_from_git(false, false, false)?;
                if let Some(root) = all_tree {
                    let all = root.flatten(
                        self.indent_size,
                        self.collapse,
                        &self.theme,
                        &self.collapsed_paths,
                    );
                    self.unstaged_nodes = all
                        .into_iter()
                        .filter(|n| !n.raw_status.ends_with('+'))
                        .collect();
                } else {
                    self.unstaged_nodes = Vec::new();
                }

                let max_staged = self
                    .staged_nodes
                    .iter()
                    .map(|n| n.connector.width() + n.name.width())
                    .max()
                    .unwrap_or(0);
                let max_unstaged = self
                    .unstaged_nodes
                    .iter()
                    .map(|n| n.connector.width() + n.name.width())
                    .max()
                    .unwrap_or(0);

                self.max_name_width = max_staged.max(max_unstaged);

                let staged_active = self.focus == Focus::Staged;
                Self::adjust_selection(&self.staged_nodes, &mut self.staged_state, staged_active);
            }
        }

        // Collect global stats
        let mut stats_map = std::collections::HashMap::new();
        let _ = git::collect_diff_stats(&mut stats_map, &["diff", "--numstat"]);
        let _ = git::collect_diff_stats(&mut stats_map, &["diff", "--cached", "--numstat"]);

        let mut total_added = 0;
        let mut total_deleted = 0;
        for (a, d) in stats_map.values() {
            total_added += a;
            total_deleted += d;
        }
        self.global_stats = Some((total_added, total_deleted));

        Ok(())
    }

    pub fn toggle_filter(&mut self) -> Result<()> {
        if self.layout == AppLayout::Unified {
            self.filter_mode = self.filter_mode.next();
            self.refresh()
        } else {
            Ok(())
        }
    }

    pub fn toggle_layout(&mut self) -> Result<()> {
        self.layout = self.layout.next();
        self.refresh()
    }

    pub fn toggle_theme(&mut self) -> Result<()> {
        self.theme_type = self.theme_type.next();
        self.theme = Theme::new(self.theme_type);
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
        } else if !nodes.is_empty() && (is_active || state.selected().is_none()) {
            state.select(Some(0));
        } else if nodes.is_empty() {
            state.select(None);
        }
    }

    pub fn toggle_focus(&mut self) -> Result<()> {
        self.focus = self.focus.next();
        Ok(())
    }

    pub fn reset_selection(&mut self) {
        self.staged_state.select(Some(0));
        self.unstaged_state.select(Some(0));
        self.unified_state.select(Some(0));
    }

    pub fn show_diff(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if let Some(i) = state.selected() {
            if let Some(node) = filtered.get(i) {
                if node.is_dir {
                    return Ok(());
                }

                let is_staged = node.raw_status.contains('+');
                let is_untracked = node.raw_status == "??";

                match git::get_diff(&node.full_path, is_staged, is_untracked) {
                    Ok(content) => {
                        if content.is_empty() && !is_untracked {
                            self.diff_content = "(No diff or binary file)".to_string();
                        } else {
                            self.diff_content = content;
                        }
                        self.view_mode = ViewMode::Diff;
                        self.diff_scroll = 0;
                    }
                    Err(e) => {
                        self.diff_content = format!("Error running git diff: {}", e);
                        self.view_mode = ViewMode::Diff;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn close_diff(&mut self) {
        self.view_mode = ViewMode::Tree;
        self.diff_content.clear();
    }

    pub fn scroll_diff(&mut self, amount: i16) {
        if amount > 0 {
            self.diff_scroll = self.diff_scroll.saturating_add(amount as u16);
        } else {
            self.diff_scroll = self.diff_scroll.saturating_sub((-amount) as u16);
        }
    }

    pub fn next(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if filtered.is_empty() {
            return;
        }
        let i = match state.selected() {
            Some(i) => {
                if i >= filtered.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if filtered.is_empty() {
            return;
        }
        let i = match state.selected() {
            Some(i) => {
                if i == 0 {
                    filtered.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn next_file(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        let filtered = Self::filter_nodes(nodes, &self.search_query);
        if filtered.is_empty() {
            return;
        }

        let start_idx = state.selected().unwrap_or(0);
        let mut idx = start_idx;

        for _ in 0..filtered.len() {
            if idx >= filtered.len() - 1 {
                idx = 0;
            } else {
                idx += 1;
            }

            if let Some(node) = filtered.get(idx) {
                if !node.is_dir {
                    state.select(Some(idx));
                    return;
                }
            }
        }
    }

    pub fn previous_file(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        let filtered = Self::filter_nodes(nodes, &self.search_query);
        if filtered.is_empty() {
            return;
        }

        let start_idx = state.selected().unwrap_or(0);
        let mut idx = start_idx;

        for _ in 0..filtered.len() {
            if idx == 0 {
                idx = filtered.len() - 1;
            } else {
                idx -= 1;
            }

            if let Some(node) = filtered.get(idx) {
                if !node.is_dir {
                    state.select(Some(idx));
                    return;
                }
            }
        }
    }

    pub fn toggle_stage(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if let Some(i) = state.selected() {
            if let Some(node) = filtered.get(i) {
                let is_staged = node.raw_status.contains('+');
                git::toggle_stage(&node.full_path, is_staged)?;
                self.refresh()?;
            }
        }
        Ok(())
    }

    pub fn expand_node(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };
        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if let Some(i) = state.selected() {
            if let Some(node) = filtered.get(i) {
                if node.is_dir && self.collapsed_paths.contains(&node.full_path) {
                    self.collapsed_paths.remove(&node.full_path);
                    self.refresh()?;
                }
            }
        }
        Ok(())
    }

    pub fn collapse_node(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified => (&self.unified_nodes, &mut self.unified_state),
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };
        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if let Some(i) = state.selected() {
            if let Some(node) = filtered.get(i) {
                if node.is_dir && !self.collapsed_paths.contains(&node.full_path) {
                    self.collapsed_paths.insert(node.full_path.clone());
                    self.refresh()?;
                }
            }
        }
        Ok(())
    }

    pub fn collapse_all(&mut self) -> Result<()> {
        let tree = git::build_tree_from_git(false, false, false)?;
        if let Some(root) = tree {
            root.get_all_dir_paths(&mut self.collapsed_paths);
            self.refresh()?;
        }
        Ok(())
    }

    pub fn expand_all(&mut self) -> Result<()> {
        self.collapsed_paths.clear();
        self.refresh()?;
        Ok(())
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
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

    #[test]
    fn test_filter_nodes() {
        let nodes = vec![
            FlatNode {
                name: "foo.rs".into(),
                name_colored: "foo.rs".into(),
                full_path: "src/foo.rs".into(),
                is_dir: false,
                status: ' ',
                raw_status: "??".into(),
                connector: "".into(),
                stats: None,
            },
            FlatNode {
                name: "bar.rs".into(),
                name_colored: "bar.rs".into(),
                full_path: "src/bar.rs".into(),
                is_dir: false,
                status: ' ',
                raw_status: "??".into(),
                connector: "".into(),
                stats: None,
            },
        ];

        let filtered = App::filter_nodes(&nodes, "foo");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "foo.rs");

        let filtered_all = App::filter_nodes(&nodes, "");
        assert_eq!(filtered_all.len(), 2);

        let filtered_none = App::filter_nodes(&nodes, "baz");
        assert_eq!(filtered_none.len(), 0);
    }
}
