use anyhow::Result;
use ratatui::widgets::ListState;
use std::collections::HashSet;
use unicode_width::UnicodeWidthStr;

use crate::config::KeyConfig;
use crate::git::{self, Worktree};
use crate::node::FlatNode;
use crate::theme::{Theme, ThemeType};
use crate::tui::history::{ActionHistory, StageAction};

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
    Compact,
    EasterEgg,
}

impl AppLayout {
    pub fn next(&self) -> Self {
        match self {
            AppLayout::Unified => AppLayout::Split,
            AppLayout::Split => AppLayout::Compact,
            AppLayout::Compact => AppLayout::Unified,
            AppLayout::EasterEgg => AppLayout::Unified,
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
    pub pending_key: Option<char>,
    pub is_visual_mode: bool,
    pub visual_origin: Option<usize>,
    pub hit_top_edge: bool,
    pub hit_bottom_edge: bool,
    pub history: ActionHistory,
    pub help_scroll: u16,
    pub worktrees: Vec<Worktree>,
    pub worktree_state: ListState,
    pub show_worktrees: bool,
    pub max_help_scroll: u16,
    pub diff_search_query: String,
    pub is_diff_search: bool,
    pub diff_matches: Vec<usize>,
    pub current_diff_match: Option<usize>,
    pub show_commit_dialog: bool,
    pub commit_message: String,
    // Patch Mode
    pub patch_mode: bool,
    pub diff_headers: Vec<String>,
    pub diff_hunks: Vec<crate::git::Hunk>,
    pub selected_hunk_idx: Option<usize>,
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
            pending_key: None,
            is_visual_mode: false,
            visual_origin: None,
            hit_top_edge: false,
            hit_bottom_edge: false,
            history: ActionHistory::default(),
            help_scroll: 0,
            worktrees: Vec::new(),
            worktree_state: ListState::default(),
            show_worktrees: false,
            max_help_scroll: 0,
            diff_search_query: String::new(),
            is_diff_search: false,
            diff_matches: Vec::new(),
            current_diff_match: None,
            show_commit_dialog: false,
            commit_message: String::new(),
            patch_mode: false,
            diff_headers: Vec::new(),
            diff_hunks: Vec::new(),
            selected_hunk_idx: None,
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
            AppLayout::Unified | AppLayout::EasterEgg => {
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
                let unstaged_active = self.focus == Focus::Unstaged;
                Self::adjust_selection(
                    &self.unstaged_nodes,
                    &mut self.unstaged_state,
                    unstaged_active,
                );
            }
            AppLayout::Compact => {
                let tree = git::build_tree_from_git(false, false, false)?;
                if let Some(root) = tree {
                    self.unified_nodes = root.flatten(
                        self.indent_size,
                        true, // Force collapse for compact view
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
        self.is_visual_mode = false;
        self.visual_origin = None;
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
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
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
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
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
                    if self.hit_bottom_edge {
                        self.hit_bottom_edge = false;
                        0
                    } else {
                        self.hit_bottom_edge = true;
                        i
                    }
                } else {
                    self.hit_bottom_edge = false;
                    self.hit_top_edge = false;
                    i + 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
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
                    if self.hit_top_edge {
                        self.hit_top_edge = false;
                        filtered.len() - 1
                    } else {
                        self.hit_top_edge = true;
                        0
                    }
                } else {
                    self.hit_top_edge = false;
                    self.hit_bottom_edge = false;
                    i - 1
                }
            }
            None => 0,
        };
        state.select(Some(i));
    }

    pub fn next_file(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
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

        // Reset hit_top_edge when moving down
        self.hit_top_edge = false;

        for _ in 0..filtered.len() {
            if idx >= filtered.len() - 1 {
                if self.hit_bottom_edge {
                    self.hit_bottom_edge = false;
                    idx = 0;
                } else {
                    self.hit_bottom_edge = true;
                    state.select(Some(idx));
                    return;
                }
            } else {
                idx += 1;
            }

            if let Some(node) = filtered.get(idx) {
                if !node.is_dir {
                    self.hit_bottom_edge = false;
                    state.select(Some(idx));
                    return;
                }
            }
        }
    }

    pub fn previous_file(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
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

        // Reset hit_bottom_edge when moving up
        self.hit_bottom_edge = false;

        for _ in 0..filtered.len() {
            if idx == 0 {
                if self.hit_top_edge {
                    self.hit_top_edge = false;
                    idx = filtered.len() - 1;
                } else {
                    self.hit_top_edge = true;
                    state.select(Some(0));
                    return;
                }
            } else {
                idx -= 1;
            }

            if let Some(node) = filtered.get(idx) {
                if !node.is_dir {
                    self.hit_top_edge = false;
                    state.select(Some(idx));
                    return;
                }
            }
        }
    }

    pub fn toggle_stage(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if self.is_visual_mode {
            if let Some((start, end)) = self.get_visual_range() {
                let mut paths = Vec::new();
                let mut bulk_action = None;

                for i in start..=end {
                    if let Some(node) = filtered.get(i) {
                        paths.push(node.full_path.clone());
                        if bulk_action.is_none() {
                            bulk_action = Some(if node.raw_status.contains('+') {
                                StageAction::Unstage
                            } else {
                                StageAction::Stage
                            });
                        }
                    }
                }

                if let Some(action) = bulk_action {
                    for path in &paths {
                        git::toggle_stage(path, action == StageAction::Unstage)?;
                    }
                    self.history.push_action(paths, action);
                }

                self.is_visual_mode = false;
                self.visual_origin = None;
                self.refresh()?;
            }
        } else if let Some(i) = state.selected() {
            if let Some(node) = filtered.get(i) {
                let is_staged = node.raw_status.contains('+');
                let action = if is_staged {
                    StageAction::Unstage
                } else {
                    StageAction::Stage
                };

                git::toggle_stage(&node.full_path, is_staged)?;
                self.history
                    .push_action(vec![node.full_path.clone()], action);
                self.refresh()?;
            }
        }
        Ok(())
    }

    pub fn undo_staging(&mut self) -> Result<()> {
        if let Some(entry) = self.history.undo() {
            for path in entry.paths {
                let to_unstage = entry.action == StageAction::Stage;
                git::toggle_stage(&path, to_unstage)?;
            }
            self.refresh()?;
        }
        Ok(())
    }

    pub fn redo_staging(&mut self) -> Result<()> {
        if let Some(entry) = self.history.redo() {
            for path in entry.paths {
                let to_unstage = entry.action == StageAction::Unstage;
                git::toggle_stage(&path, to_unstage)?;
            }
            self.refresh()?;
        }
        Ok(())
    }

    pub fn expand_node(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };
        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if self.is_visual_mode {
            if let Some((start, end)) = self.get_visual_range() {
                for i in start..=end {
                    if let Some(node) = filtered.get(i) {
                        if node.is_dir && self.collapsed_paths.contains(&node.full_path) {
                            self.collapsed_paths.remove(&node.full_path);
                        }
                    }
                }
                self.is_visual_mode = false;
                self.visual_origin = None;
                self.refresh()?;
            }
        } else if let Some(i) = state.selected() {
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
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };
        let filtered = Self::filter_nodes(nodes, &self.search_query);

        if self.is_visual_mode {
            if let Some((start, end)) = self.get_visual_range() {
                for i in start..=end {
                    if let Some(node) = filtered.get(i) {
                        if node.is_dir && !self.collapsed_paths.contains(&node.full_path) {
                            self.collapsed_paths.insert(node.full_path.clone());
                        }
                    }
                }
                self.is_visual_mode = false;
                self.visual_origin = None;
                self.refresh()?;
            }
        } else if let Some(i) = state.selected() {
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
        self.help_scroll = 0;
        self.is_diff_search = false;
    }

    pub fn scroll_help(&mut self, amount: i16) {
        if amount > 0 {
            self.help_scroll = self.help_scroll.saturating_add(amount as u16);
            if self.help_scroll > self.max_help_scroll {
                self.help_scroll = self.max_help_scroll;
            }
        } else {
            self.help_scroll = self.help_scroll.saturating_sub((-amount) as u16);
        }
    }

    pub fn toggle_worktrees(&mut self) -> Result<()> {
        if !self.show_worktrees {
            self.worktrees = git::get_worktrees()?;
            if !self.worktrees.is_empty() {
                self.worktree_state.select(Some(0));
            }
        }
        self.show_worktrees = !self.show_worktrees;
        Ok(())
    }

    pub fn switch_worktree(&mut self) -> Result<()> {
        if let Some(i) = self.worktree_state.selected() {
            if let Some(wt) = self.worktrees.get(i) {
                std::env::set_current_dir(&wt.path)?;
                self.show_worktrees = false;
                self.refresh()?;
            }
        }
        Ok(())
    }

    pub fn search_diff(&mut self) {
        self.diff_matches.clear();
        self.current_diff_match = None;

        if self.diff_search_query.is_empty() {
            return;
        }

        let query = self.diff_search_query.to_lowercase();
        for (i, line) in self.diff_content.lines().enumerate() {
            // Strip ANSI codes for searching
            let clean_line = strip_ansi_codes(line).to_lowercase();
            if clean_line.contains(&query) {
                self.diff_matches.push(i);
            }
        }

        if !self.diff_matches.is_empty() {
            self.current_diff_match = Some(0);
            self.jump_to_diff_match();
        }
    }

    pub fn open_commit_dialog(&mut self) {
        self.show_commit_dialog = true;
        self.commit_message.clear();
    }

    pub fn close_commit_dialog(&mut self) {
        self.show_commit_dialog = false;
        self.commit_message.clear();
    }

    pub fn confirm_commit(&mut self) -> Result<()> {
        if !self.commit_message.is_empty() {
            git::commit(&self.commit_message)?;
            self.close_commit_dialog();
            self.refresh()?;
        }
        Ok(())
    }

    pub fn toggle_patch_mode(&mut self) {
        if self.view_mode != ViewMode::Diff {
            return;
        }

        self.patch_mode = !self.patch_mode;
        if self.patch_mode {
            let (headers, hunks) = git::patch::parse_diff(&self.diff_content);
            self.diff_headers = headers;
            self.diff_hunks = hunks;
            if !self.diff_hunks.is_empty() {
                self.selected_hunk_idx = Some(0);
                self.jump_to_hunk();
            }
        } else {
            self.diff_headers.clear();
            self.diff_hunks.clear();
            self.selected_hunk_idx = None;
        }
    }

    pub fn next_hunk(&mut self) {
        if let Some(i) = self.selected_hunk_idx {
            if i < self.diff_hunks.len() - 1 {
                self.selected_hunk_idx = Some(i + 1);
                self.jump_to_hunk();
            }
        }
    }

    pub fn prev_hunk(&mut self) {
        if let Some(i) = self.selected_hunk_idx {
            if i > 0 {
                self.selected_hunk_idx = Some(i - 1);
                self.jump_to_hunk();
            }
        }
    }

    fn jump_to_hunk(&mut self) {
        if let Some(i) = self.selected_hunk_idx {
            if let Some(hunk) = self.diff_hunks.get(i) {
                // Adjust scroll to center the hunk
                let target = hunk.display_start as u16;
                // Simple version: jump to top
                self.diff_scroll = target;
            }
        }
    }

    pub fn stage_hunk(&mut self) -> Result<()> {
        if let Some(i) = self.selected_hunk_idx {
            if let Some(hunk) = self.diff_hunks.get(i) {
                let _is_staged = if let Some(_idx) = self.unified_state.selected() {
                    // This is tricky: we need to know if the CURRENT file is staged or not
                    // But patch mode is generic.
                    // Generally, if we are in Diff view, we are diffing a specific file node.
                    // And we know the status of that node.
                    // We should pass 'stage' boolean direction.
                    // However, git apply --cached applies TO the index (staging it).
                    // git apply --reverse --cached would UNSTAGE it.
                    // We need to know if we are 'Adding' or 'Resetting'.
                    // Let's rely on the raw_status of the selected node.
                    false // FIXME: Logic needed below
                } else {
                    false
                };

                // For now, let's assume this is mostly for STAGING (add -p).
                // But unstage -p is also valid.
                // We need to look up the node again.
                // This is slightly inefficient but safe.

                let node = self.get_selected_node();
                if let Some(n) = node {
                    let is_staged = n.raw_status.contains('+');
                    if is_staged {
                        // Unstage: git apply --cached --reverse
                        // Not implemented in helper yet, but we can just use `git restore --patch`?
                        // Or update apply_patch to support reverse.
                        // Let's update apply_patch first.
                        // For now, let's just support Staging (add -p equivalent).
                        // If user tries to stage checks on a staged file, it does nothing or errors.

                        // Actually, apply_patch takes 'headers'.
                        // If we are unstaging, we might need --reverse.
                        // Let's start with just handling Staging for v1.3.0 scope if complex.
                        // roadmap says "Interactive Patch Staging".

                        git::patch::apply_patch(&self.diff_headers, hunk, true)?;
                    } else {
                        // Stage: git apply --cached
                        git::patch::apply_patch(&self.diff_headers, hunk, true)?;
                    }

                    // Refresh to show status change
                    // self.refresh()?;
                    // Ideally we stay in diff mode but refresh the underlying diff?
                    // If we stage a hunk, it disappears from "Unstaged" diff.
                    // So refreshing is good.
                    self.refresh()?;

                    // Re-parse diff to keep index validity?
                    // Or just close patch mode?
                    // Most robust: close patch mode or re-parse.
                    self.toggle_patch_mode(); // Close for safety
                    self.show_diff()?; // Re-open diff (will fetch new content)
                    self.toggle_patch_mode(); // Re-enable? Maybe annoying.
                                              // Let's just exit patch mode and let user re-enter if they want more.
                    self.toggle_patch_mode(); // Off
                    self.show_diff()?;
                }
            }
        }
        Ok(())
    }

    fn get_selected_node(&self) -> Option<&crate::node::FlatNode> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &self.unified_state)
            }
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &self.unstaged_state),
            },
        };
        // Simplified: use self.filter_nodes logic...
        // Actually, we are already in Diff View, so we must have a selected node.
        // We can just re-use the logic from show_diff.
        // BUT, retrieving it cleanly is better.

        let filtered = Self::filter_nodes(nodes, &self.search_query);
        if let Some(i) = state.selected() {
            filtered.get(i).copied()
        } else {
            None
        }
    }
    pub fn next_diff_match(&mut self) {
        if self.diff_matches.is_empty() {
            return;
        }
        let current = self.current_diff_match.unwrap_or(0);
        let next = (current + 1) % self.diff_matches.len();
        self.current_diff_match = Some(next);
        self.jump_to_diff_match();
    }

    pub fn prev_diff_match(&mut self) {
        if self.diff_matches.is_empty() {
            return;
        }
        let current = self.current_diff_match.unwrap_or(0);
        let prev = if current == 0 {
            self.diff_matches.len() - 1
        } else {
            current - 1
        };
        self.current_diff_match = Some(prev);
        self.jump_to_diff_match();
    }

    fn jump_to_diff_match(&mut self) {
        if let Some(i) = self.current_diff_match {
            if let Some(&line_idx) = self.diff_matches.get(i) {
                self.diff_scroll = line_idx as u16;
            }
        }
    }

    pub fn jump_to_top(&mut self) {
        let state = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                &mut self.unified_state
            }
            AppLayout::Split => match self.focus {
                Focus::Staged => &mut self.staged_state,
                Focus::Unstaged => &mut self.unstaged_state,
            },
        };
        state.select(Some(0));
        self.hit_top_edge = false;
        self.hit_bottom_edge = false;
    }

    pub fn jump_to_bottom(&mut self) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };
        let filtered = Self::filter_nodes(nodes, &self.search_query);
        if !filtered.is_empty() {
            state.select(Some(filtered.len() - 1));
        }
        self.hit_top_edge = false;
        self.hit_bottom_edge = false;
    }

    pub fn yank_path(&mut self) -> Result<()> {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
            AppLayout::Split => match self.focus {
                Focus::Staged => (&self.staged_nodes, &mut self.staged_state),
                Focus::Unstaged => (&self.unstaged_nodes, &mut self.unstaged_state),
            },
        };

        let filtered = Self::filter_nodes(nodes, &self.search_query);
        let mut clipboard = arboard::Clipboard::new()?;

        if self.is_visual_mode {
            if let Some((start, end)) = self.get_visual_range() {
                let paths: Vec<String> = filtered[start..=end]
                    .iter()
                    .map(|n| n.full_path.clone())
                    .collect();
                clipboard.set_text(paths.join("\n"))?;
                self.is_visual_mode = false;
                self.visual_origin = None;
            }
        } else if let Some(i) = state.selected() {
            if let Some(node) = filtered.get(i) {
                clipboard.set_text(node.full_path.clone())?;
            }
        }
        Ok(())
    }

    pub fn toggle_visual_mode(&mut self) {
        if self.is_visual_mode {
            self.is_visual_mode = false;
            self.visual_origin = None;
        } else {
            let state = match self.layout {
                AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                    &self.unified_state
                }
                AppLayout::Split => match self.focus {
                    Focus::Staged => &self.staged_state,
                    Focus::Unstaged => &self.unstaged_state,
                },
            };
            if let Some(i) = state.selected() {
                self.is_visual_mode = true;
                self.visual_origin = Some(i);
            }
        }
    }

    pub fn get_visual_range(&self) -> Option<(usize, usize)> {
        if !self.is_visual_mode {
            return None;
        }

        let origin = self.visual_origin?;
        let state = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => &self.unified_state,
            AppLayout::Split => match self.focus {
                Focus::Staged => &self.staged_state,
                Focus::Unstaged => &self.unstaged_state,
            },
        };
        let current = state.selected()?;

        if origin < current {
            Some((origin, current))
        } else {
            Some((current, origin))
        }
    }

    pub fn scroll_paging(&mut self, amount: i32) {
        let (nodes, state) = match self.layout {
            AppLayout::Unified | AppLayout::Compact | AppLayout::EasterEgg => {
                (&self.unified_nodes, &mut self.unified_state)
            }
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
                if amount > 0 {
                    (i + amount as usize).min(filtered.len() - 1)
                } else {
                    i.saturating_sub((-amount) as usize)
                }
            }
            None => 0,
        };
        state.select(Some(i));
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
        assert_eq!(layout.next().next(), AppLayout::Compact);
        assert_eq!(layout.next().next().next(), AppLayout::Unified);
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
                depth: 0,
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
                depth: 0,
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

fn strip_ansi_codes(s: &str) -> String {
    let re = regex::Regex::new(r"\x1B\[[0-9;]*[mK]").unwrap();
    re.replace_all(s, "").to_string()
}
