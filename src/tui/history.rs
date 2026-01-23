#[derive(Debug, Clone, PartialEq)]
pub enum StageAction {
    Stage,
    Unstage,
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub paths: Vec<String>,
    pub action: StageAction,
}

#[derive(Default)]
pub struct ActionHistory {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
}

impl ActionHistory {
    pub fn push_action(&mut self, paths: Vec<String>, action: StageAction) {
        self.undo_stack.push(HistoryEntry { paths, action });
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<HistoryEntry> {
        let entry = self.undo_stack.pop()?;
        self.redo_stack.push(entry.clone());
        Some(entry)
    }

    pub fn redo(&mut self) -> Option<HistoryEntry> {
        let entry = self.redo_stack.pop()?;
        self.undo_stack.push(entry.clone());
        Some(entry)
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_clear_redo() {
        let mut history = ActionHistory::default();
        history.push_action(vec!["a".to_string()], StageAction::Stage);
        assert_eq!(history.undo_stack.len(), 1);
        
        // Push clears redo
        history.redo_stack.push(HistoryEntry {
            paths: vec!["b".to_string()],
            action: StageAction::Unstage,
        });
        history.push_action(vec!["c".to_string()], StageAction::Stage);
        assert_eq!(history.redo_stack.len(), 0);
    }

    #[test]
    fn test_undo_redo_stack() {
        let mut history = ActionHistory::default();
        let paths = vec!["test.rs".to_string()];
        history.push_action(paths.clone(), StageAction::Stage);
        
        // Undo
        let undo_entry = history.undo().unwrap();
        assert_eq!(undo_entry.paths, paths);
        assert_eq!(undo_entry.action, StageAction::Stage);
        assert_eq!(history.undo_stack.len(), 0);
        assert_eq!(history.redo_stack.len(), 1);
        
        // Redo
        let redo_entry = history.redo().unwrap();
        assert_eq!(redo_entry.paths, paths);
        assert_eq!(redo_entry.action, StageAction::Stage);
        assert_eq!(history.undo_stack.len(), 1);
        assert_eq!(history.redo_stack.len(), 0);
    }
}
