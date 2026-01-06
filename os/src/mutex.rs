//! # mutex
//!
//! mutex 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Mutex wrapper

use parking_lot::Mutex as ParkingMutex;

/// Mutex wrapper
pub struct Mutex<T> {
    inner: ParkingMutex<T>,
}

impl<T> Mutex<T> {
    /// Create a new mutex
    pub fn new(value: T) -> Self {
        Self {
            inner: ParkingMutex::new(value),
        }
    }

    /// Lock and get a guard
    pub fn lock(&self) -> parking_lot::MutexGuard<'_, T> {
        self.inner.lock()
    }
}

