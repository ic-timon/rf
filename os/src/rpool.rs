//! # rpool
//!
//! rpool 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Runtime pool (task pool)

use tokio::task;

/// Spawn a task
pub fn spawn<F>(future: F) -> task::JoinHandle<F::Output>
where
    F: std::future::Future + Send + 'static,
    F::Output: Send + 'static,
{
    task::spawn(future)
}

/// Spawn a blocking task
pub fn spawn_blocking<F, R>(f: F) -> task::JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    task::spawn_blocking(f)
}

