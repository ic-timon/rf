//! # mlock
//!
//! mlock 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Memory lock

use parking_lot::Mutex;

/// Memory lock wrapper
pub struct MLock<T> {
    inner: Mutex<T>,
}

impl<T> MLock<T> {
    /// Create a new memory lock
    pub fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }

    /// Lock and get a guard
    pub fn lock(&self) -> parking_lot::MutexGuard<'_, T> {
        self.inner.lock()
    }
}

