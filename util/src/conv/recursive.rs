//! # recursive
//!
//! recursive 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Recursive conversion optimization

use std::collections::HashSet;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::any::TypeId;

/// Recursive conversion context
pub struct RecursiveContext {
    max_depth: usize,
    current_depth: usize,
    visited: HashSet<u64>, // TypeId + pointer address hash
    cache: Option<Arc<dyn RecursiveCache>>,
}

/// Trait for recursive conversion cache
pub trait RecursiveCache: Send + Sync {
    fn get(&self, key: &str) -> Option<Vec<u8>>;
    fn set(&self, key: String, value: Vec<u8>);
    fn clear(&self);
}

impl RecursiveContext {
    /// Create a new recursive context
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            current_depth: 0,
            visited: HashSet::new(),
            cache: None,
        }
    }

    /// Create with cache
    pub fn with_cache(max_depth: usize, cache: Arc<dyn RecursiveCache>) -> Self {
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

    /// Get cache if available
    pub fn cache(&self) -> Option<&Arc<dyn RecursiveCache>> {
        self.cache.as_ref()
    }
}

impl Default for RecursiveContext {
    fn default() -> Self {
        Self::new(10) // Default max depth of 10
    }
}

/// Helper function to create a recursive context with default settings
pub fn create_context() -> RecursiveContext {
    RecursiveContext::default()
}

/// Helper function to create a recursive context with custom max depth
pub fn create_context_with_depth(max_depth: usize) -> RecursiveContext {
    RecursiveContext::new(max_depth)
}

