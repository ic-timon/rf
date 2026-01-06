//! # cache
//!
//! cache 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Type conversion caching

use moka::future::Cache;
use std::hash::Hash;
use std::time::Duration;

/// Conversion cache for optimizing repeated conversions
pub struct ConversionCache<K, V> {
    cache: Cache<K, V>,
}

impl<K, V> ConversionCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create a new conversion cache
    pub fn new(capacity: u64, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(capacity)
            .time_to_live(ttl)
            .build();
        
        Self { cache }
    }

    /// Get cached conversion result
    pub async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    /// Set cached conversion result
    pub async fn set(&self, key: K, value: V) {
        self.cache.insert(key, value).await;
    }

    /// Invalidate cache entry
    pub fn invalidate(&self, key: &K) {
        self.cache.invalidate(key);
    }

    /// Clear all cache
    pub fn clear(&self) {
        self.cache.invalidate_all();
    }
}

impl<K, V> Default for ConversionCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new(1000, Duration::from_secs(300))
    }
}

