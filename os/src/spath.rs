//! # spath
//!
//! spath 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Path utilities

use std::path::{Path, PathBuf};

/// Get absolute path
pub fn abs(path: &Path) -> std::io::Result<PathBuf> {
    path.canonicalize()
}

/// Join paths
pub fn join<P: AsRef<Path>>(base: &Path, path: P) -> PathBuf {
    base.join(path)
}

