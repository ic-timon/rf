//! # res
//!
//! res 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Resource management

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Resource storage
pub struct ResourceStorage {
    resources: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl ResourceStorage {
    /// Create a new resource storage
    pub fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a resource
    pub async fn register(&self, path: &str, data: Vec<u8>) {
        let mut resources = self.resources.write().await;
        resources.insert(path.to_string(), data);
    }

    /// Get embedded resource as bytes
    pub async fn get_bytes(&self, path: &str) -> Option<Vec<u8>> {
        let resources = self.resources.read().await;
        resources.get(path).cloned()
    }

    /// Get embedded resource as string
    pub async fn get_string(&self, path: &str) -> Option<String> {
        self.get_bytes(path).await
            .and_then(|bytes| String::from_utf8(bytes).ok())
    }
}

impl Default for ResourceStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Global resource storage instance
static RESOURCE_STORAGE: once_cell::sync::Lazy<ResourceStorage> = once_cell::sync::Lazy::new(|| {
    ResourceStorage::new()
});

/// Get embedded resource as bytes
pub async fn get_bytes(path: &str) -> Option<Vec<u8>> {
    RESOURCE_STORAGE.get_bytes(path).await
}

/// Get embedded resource as string
pub async fn get_string(path: &str) -> Option<String> {
    RESOURCE_STORAGE.get_string(path).await
}

/// Register a resource
pub async fn register(path: &str, data: Vec<u8>) {
    RESOURCE_STORAGE.register(path, data).await;
}
