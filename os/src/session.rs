//! # session
//!
//! session 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Session management

pub mod storage;

pub use storage::*;

use axum_sessions::async_session::Session;
use rf_errors::Result;
use uuid::Uuid;

/// Session manager
pub struct SessionManager {
    storage: Box<dyn SessionStorage>,
    ttl: Option<std::time::Duration>,
}

impl SessionManager {
    /// Create a new session manager with memory storage
    pub fn new() -> Self {
        Self {
            storage: Box::new(MemorySessionStorage::new()),
            ttl: None,
        }
    }

    /// Create a new session manager with custom storage
    pub fn with_storage(storage: Box<dyn SessionStorage>) -> Self {
        Self {
            storage,
            ttl: None,
        }
    }

    /// Set TTL for sessions
    pub fn with_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Generate a new session ID
    pub fn generate_session_id(&self) -> String {
        Uuid::new_v4().to_string()
    }

    /// Get a session by ID
    pub fn get(&self, id: &str) -> Result<Option<Session>> {
        self.storage.get(id)
    }

    /// Store a session
    pub fn store(&self, session: Session) -> Result<()> {
        self.storage.store(session)
    }

    /// Delete a session
    pub fn delete(&self, id: &str) -> Result<()> {
        self.storage.delete(id)
    }

    /// Clean up expired sessions (for file/database storage)
    pub fn cleanup_expired(&self) -> Result<()> {
        self.storage.cleanup_expired()
    }

    /// Start automatic expiration cleanup task
    pub fn start_cleanup_task(&self, interval: std::time::Duration) -> tokio::task::JoinHandle<()> {
        let storage = self.storage.as_ref();
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            loop {
                interval_timer.tick().await;
                let _ = storage.cleanup_expired();
            }
        })
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
