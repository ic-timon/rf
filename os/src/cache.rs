//! # cache
//!
//! cache 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Cache system

use moka::future::Cache;
use std::hash::Hash;
use std::time::Duration;

/// Generic cache wrapper
pub struct CacheContainer<K, V> {
    cache: Cache<K, V>,
}

impl<K: Hash + Eq + Send + Sync + 'static, V: Clone + Send + Sync + 'static> CacheContainer<K, V> {
    /// Create a new cache with capacity
    pub fn new(capacity: u64) -> Self {
        Self {
            cache: Cache::new(capacity),
        }
    }

    /// Create a new cache with TTL
    pub fn with_ttl(capacity: u64, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(capacity)
            .time_to_live(ttl)
            .build();
        Self { cache }
    }

    /// Get a value
    pub async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    /// Insert a value
    pub async fn insert(&self, key: K, value: V) {
        self.cache.insert(key, value).await;
    }

    /// Remove a value
    pub async fn remove(&self, key: &K) {
        self.cache.invalidate(key).await;
    }

    /// Clear all entries
    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }
}
