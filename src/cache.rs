//! Git data caching layer
//!
//! Caches git status, diff stats, and other frequently-accessed data
//! to reduce redundant git command invocations.

use anyhow::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::git::{self, DiffStats};
use crate::node::Node;

/// Default cache TTL (time-to-live) in milliseconds
const DEFAULT_TTL_MS: u64 = 100;

/// Centralized cache for git data
#[derive(Debug)]
pub struct GitCache {
    /// Cached tree for all files
    tree_all: Option<Node>,
    /// Cached tree for staged files only
    tree_staged: Option<Node>,
    /// Cached diff stats (combined staged + unstaged)
    diff_stats: DiffStats,
    /// Last refresh timestamp
    last_refresh: Instant,
    /// Cache TTL
    #[allow(dead_code)] // Will be used in Phase 3 for TTL-based refresh
    ttl: Duration,
}

impl Default for GitCache {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)] // Methods will be used in Phase 3 (incremental updates)
impl GitCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            tree_all: None,
            tree_staged: None,
            diff_stats: HashMap::new(),
            last_refresh: Instant::now(),
            ttl: Duration::from_millis(DEFAULT_TTL_MS),
        }
    }

    /// Check if the cache is stale
    pub fn is_stale(&self) -> bool {
        self.last_refresh.elapsed() > self.ttl
    }

    /// Invalidate the cache (force next refresh to fetch fresh data)
    pub fn invalidate(&mut self) {
        self.tree_all = None;
        self.tree_staged = None;
        self.diff_stats.clear();
        // Set last_refresh to epoch so is_stale() returns true
        self.last_refresh = Instant::now() - Duration::from_secs(3600);
    }

    /// Refresh the cache with fresh data from git
    pub fn refresh(&mut self) -> Result<()> {
        // Fetch all files tree (this also gets diff stats)
        let (all_tree, stats) = git::build_tree_from_git(false, false, false)?;
        self.tree_all = all_tree;
        self.diff_stats = stats;

        // Fetch staged-only tree
        let (staged_tree, _) = git::build_tree_from_git(true, false, false)?;
        self.tree_staged = staged_tree;

        self.last_refresh = Instant::now();
        Ok(())
    }

    /// Get the tree for all files, refreshing if stale
    pub fn get_tree_all(&mut self) -> Result<Option<&Node>> {
        if self.is_stale() || self.tree_all.is_none() {
            self.refresh()?;
        }
        Ok(self.tree_all.as_ref())
    }

    /// Get the tree for staged files only, refreshing if stale
    pub fn get_tree_staged(&mut self) -> Result<Option<&Node>> {
        if self.is_stale() || self.tree_staged.is_none() {
            self.refresh()?;
        }
        Ok(self.tree_staged.as_ref())
    }

    /// Get diff stats, refreshing if stale
    pub fn get_diff_stats(&mut self) -> Result<&DiffStats> {
        if self.is_stale() {
            self.refresh()?;
        }
        Ok(&self.diff_stats)
    }

    /// Get cached diff stats without refresh check (for use after manual refresh)
    pub fn diff_stats(&self) -> &DiffStats {
        &self.diff_stats
    }

    /// Get the global stats (total added/deleted lines)
    pub fn get_global_stats(&mut self) -> Result<(usize, usize)> {
        if self.is_stale() {
            self.refresh()?;
        }

        let mut total_added = 0;
        let mut total_deleted = 0;
        for (a, d) in self.diff_stats.values() {
            total_added += a;
            total_deleted += d;
        }
        Ok((total_added, total_deleted))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_is_stale() {
        let cache = GitCache::new();
        // Fresh cache should not be stale
        assert!(!cache.is_stale());
    }

    #[test]
    fn test_cache_invalidate() {
        let mut cache = GitCache::new();
        cache.invalidate();
        // After invalidation, cache should be stale
        assert!(cache.is_stale());
    }
}
