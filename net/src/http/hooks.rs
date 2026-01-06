//! # hooks
//!
//! hooks 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP server hooks

use axum::extract::Request;
use axum::response::Response;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// Hook name type
pub type HookName = &'static str;

/// Hook function type
pub type HookFn = Arc<dyn Fn(&Request, &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>;

/// Hook manager
pub struct HookManager {
    hooks: Arc<Mutex<HashMap<HookName, Vec<HookFn>>>>,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new() -> Self {
        Self {
            hooks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a hook
    pub async fn register(&self, name: HookName, hook: HookFn) {
        let mut hooks = self.hooks.lock().await;
        hooks.entry(name).or_insert_with(Vec::new).push(hook);
    }

    /// Call hooks for a given name
    pub async fn call(&self, name: HookName, request: &Request, response: &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let hooks = self.hooks.lock().await;
        if let Some(hook_list) = hooks.get(name) {
            for hook in hook_list.iter() {
                hook(request, response)?;
            }
        }
        Ok(())
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook names
pub const HOOK_BEFORE_SERVE: HookName = "HOOK_BEFORE_SERVE";
pub const HOOK_AFTER_SERVE: HookName = "HOOK_AFTER_SERVE";
pub const HOOK_BEFORE_OUTPUT: HookName = "HOOK_BEFORE_OUTPUT";
pub const HOOK_AFTER_OUTPUT: HookName = "HOOK_AFTER_OUTPUT";

