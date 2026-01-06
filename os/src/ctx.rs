//! # ctx
//!
//! ctx 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Context management

use tokio::task::LocalSet;

/// Context wrapper
pub struct Ctx {
    local_set: LocalSet,
}

impl Ctx {
    /// Create a new context
    pub fn new() -> Self {
        Self {
            local_set: LocalSet::new(),
        }
    }

    /// Run a future in the context
    pub async fn run<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future,
    {
        self.local_set.run_until(future).await
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self::new()
    }
}

