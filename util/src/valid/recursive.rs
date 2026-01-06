//! # recursive
//!
//! recursive 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Recursive validation optimization

use std::collections::HashSet;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::any::TypeId;
use moka::future::Cache;
use std::time::Duration;

/// Recursive validation context
pub struct RecursiveValidationContext {
    max_depth: usize,
    current_depth: usize,
    visited: HashSet<u64>, // TypeId + pointer address hash
    cache: Option<Arc<Cache<String, bool>>>, // Validation result cache
}

impl RecursiveValidationContext {
    /// Create a new recursive validation context
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            current_depth: 0,
            visited: HashSet::new(),
            cache: None,
        }
    }

    /// Create with cache
    pub fn with_cache(max_depth: usize, cache: Arc<Cache<String, bool>>) -> Self {
        Self {
            max_depth,
            current_depth: 0,
            visited: HashSet::new(),
            cache: Some(cache),
        }
    }

    /// Check if we can recurse deeper
    pub fn can_recurse(&self) -> bool {
        self.current_depth < self.max_depth
    }

    /// Enter a recursive level
    pub fn enter(&mut self) -> Result<(), String> {
        if self.current_depth >= self.max_depth {
            return Err(format!("Maximum recursion depth {} exceeded", self.max_depth));
        }
        self.current_depth += 1;
        Ok(())
    }

    /// Exit a recursive level
    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }

    /// Check if we've visited this object (cycle detection)
    pub fn is_visited(&self, type_id: TypeId, ptr: usize) -> bool {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        type_id.hash(&mut hasher);
        ptr.hash(&mut hasher);
        let hash = hasher.finish();
        self.visited.contains(&hash)
    }

    /// Mark an object as visited
    pub fn mark_visited(&mut self, type_id: TypeId, ptr: usize) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        type_id.hash(&mut hasher);
        ptr.hash(&mut hasher);
        let hash = hasher.finish();
        self.visited.insert(hash);
    }

    /// Clear visited set
    pub fn clear_visited(&mut self) {
        self.visited.clear();
    }

    /// Get current depth
    pub fn depth(&self) -> usize {
        self.current_depth
    }

    /// Check cache for validation result
    pub async fn get_cached(&self, key: &str) -> Option<bool> {
        if let Some(ref cache) = self.cache {
            cache.get(key).await
        } else {
            None
        }
    }

    /// Cache validation result
    pub async fn cache_result(&self, key: String, result: bool) {
        if let Some(ref cache) = self.cache {
            cache.insert(key, result).await;
        }
    }
}

impl Default for RecursiveValidationContext {
    fn default() -> Self {
        Self::new(10) // Default max depth of 10
    }
}

/// Create a validation cache
pub fn create_validation_cache(capacity: u64, ttl: Duration) -> Arc<Cache<String, bool>> {
    Arc::new(
        Cache::builder()
            .max_capacity(capacity)
            .time_to_live(ttl)
            .build()
    )
}

/// Helper function to create a recursive validation context
pub fn create_validation_context() -> RecursiveValidationContext {
    RecursiveValidationContext::default()
}

/// Helper function to create a recursive validation context with cache
pub fn create_validation_context_with_cache(
    max_depth: usize,
    cache: Arc<Cache<String, bool>>
) -> RecursiveValidationContext {
    RecursiveValidationContext::with_cache(max_depth, cache)
}

