//! # fsnotify
//!
//! fsnotify 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! File system notification

use notify::{RecommendedWatcher, Watcher};
use rf_errors::Result;
use std::path::Path;

/// File system watcher
pub struct FsNotify {
    watcher: RecommendedWatcher,
}

impl FsNotify {
    /// Create a new file system watcher
    pub fn new() -> Result<Self> {
        let watcher = notify::recommended_watcher(|_| {})
            .map_err(|e| rf_errors::RfError::Internal(format!("Failed to create watcher: {}", e)))?;
        Ok(Self { watcher })
    }

    /// Watch a path
    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.watcher
            .watch(path.as_ref(), notify::RecursiveMode::Recursive)
            .map_err(|e| rf_errors::RfError::Internal(format!("Failed to watch path: {}", e)))?;
        Ok(())
    }
}

