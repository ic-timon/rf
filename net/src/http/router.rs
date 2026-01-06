//! # router
//!
//! router 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP router with parameter parsing

use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;
use moka::future::Cache;
use std::time::Duration;

/// Route parameter names extracted from route pattern
#[derive(Debug, Clone)]
pub struct RouteParams {
    names: Vec<String>,
    pattern: Regex,
}

impl RouteParams {
    /// Create route params from pattern
    /// Supports patterns like:
    /// - `/user/:id` -> matches `/user/123` with param `id=123`
    /// - `/user/:id/posts/:post_id` -> matches `/user/123/posts/456` with params `id=123`, `post_id=456`
    /// - `/files/*path` -> matches `/files/a/b/c` with param `path=a/b/c`
    pub fn from_pattern(pattern: &str) -> Self {
        let mut names = Vec::new();
        let mut regex_pattern = String::new();
        let mut in_param = false;
        let mut current_param = String::new();
        
        for ch in pattern.chars() {
            match ch {
                ':' if !in_param => {
                    in_param = true;
                    current_param.clear();
                }
                '/' if in_param => {
                    // End of parameter
                    names.push(current_param.clone());
                    regex_pattern.push_str(r"([^/]+)");
                    in_param = false;
                    regex_pattern.push(ch);
                }
                '*' if !in_param => {
                    // Wildcard parameter
                    in_param = true;
                    current_param = "path".to_string();
                }
                _ if in_param => {
                    current_param.push(ch);
                }
                _ => {
                    if in_param && current_param.is_empty() {
                        // Handle wildcard
                        names.push("path".to_string());
                        regex_pattern.push_str(r"(.*)");
                        in_param = false;
                    } else if in_param {
                        names.push(current_param.clone());
                        regex_pattern.push_str(r"([^/]+)");
                        in_param = false;
                    }
                    // Escape special regex characters
                    if ch == '.' || ch == '+' || ch == '*' || ch == '?' || ch == '^' || ch == '$' || ch == '|' || ch == '(' || ch == ')' || ch == '[' || ch == ']' || ch == '{' || ch == '}' {
                        regex_pattern.push('\\');
                    }
                    regex_pattern.push(ch);
                }
            }
        }
        
        // Handle trailing parameter
        if in_param {
            names.push(current_param);
            regex_pattern.push_str(r"([^/]+)");
        }
        
        regex_pattern = format!("^{}$", regex_pattern);
        let pattern = Regex::new(&regex_pattern)
            .unwrap_or_else(|_| Regex::new(&format!("^{}$", regex::escape(pattern))).unwrap());
        
        Self { names, pattern }
    }
    
    /// Extract parameters from path
    pub fn extract(&self, path: &str) -> Option<HashMap<String, String>> {
        if let Some(captures) = self.pattern.captures(path) {
            let mut params = HashMap::new();
            for (i, name) in self.names.iter().enumerate() {
                if let Some(value) = captures.get(i + 1) {
                    params.insert(name.clone(), value.as_str().to_string());
                }
            }
            Some(params)
        } else {
            None
        }
    }
    
    /// Check if path matches pattern
    pub fn matches(&self, path: &str) -> bool {
        self.pattern.is_match(path)
    }
}

/// Type alias for middleware function
type MiddlewareFn = Arc<dyn Fn(axum::extract::Request, axum::extract::Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<axum::response::Response, axum::Error>> + Send>> + Send + Sync>;

/// Route group for organizing routes
pub struct RouteGroup {
    prefix: String,
    middleware: Vec<MiddlewareFn>,
}

impl RouteGroup {
    /// Create a new route group
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            middleware: Vec::new(),
        }
    }
    
    /// Add middleware to the group
    pub fn middleware<F>(mut self, middleware: F) -> Self
    where
        F: Fn(axum::extract::Request, axum::extract::Request) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<axum::response::Response, axum::Error>> + Send>> + Send + Sync + 'static,
    {
        self.middleware.push(Arc::new(move |req, next| Box::pin(middleware(req, next))));
        self
    }
    
    /// Get the prefix
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

/// Cached route match result
#[derive(Debug, Clone)]
pub struct CachedRouteMatch {
    pub params: Option<HashMap<String, String>>,
    pub matched: bool,
}

/// Route cache for optimizing route matching
pub struct RouteCache {
    cache: Arc<Cache<String, CachedRouteMatch>>,
}

impl RouteCache {
    /// Create a new route cache
    pub fn new(capacity: u64, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(capacity)
            .time_to_live(ttl)
            .build();
        
        Self {
            cache: Arc::new(cache),
        }
    }

    /// Create a default route cache (1000 entries, 5 minutes TTL)
    pub fn with_defaults() -> Self {
        Self::new(1000, Duration::from_secs(300))
    }

    /// Get cached route match result
    pub async fn get(&self, key: &str) -> Option<CachedRouteMatch> {
        self.cache.get(key).await
    }

    /// Set cached route match result
    pub async fn set(&self, key: String, value: CachedRouteMatch) {
        self.cache.insert(key, value).await;
    }

    /// Invalidate cache entry
    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    /// Clear all cache
    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }
}

impl RouteParams {
    /// Extract parameters from path with caching
    pub async fn extract_cached(&self, path: &str, cache: &RouteCache) -> Option<HashMap<String, String>> {
        let cache_key = format!("{}:{}", self.pattern.as_str(), path);
        
        // Check cache first
        if let Some(cached) = cache.get(&cache_key).await {
            return cached.params;
        }
        
        // Extract parameters
        let result = self.extract(path);
        
        // Cache the result
        let cached = CachedRouteMatch {
            params: result.clone(),
            matched: result.is_some(),
        };
        cache.set(cache_key, cached).await;
        
        result
    }
    
    /// Check if path matches pattern with caching
    pub async fn matches_cached(&self, path: &str, cache: &RouteCache) -> bool {
        let cache_key = format!("{}:{}", self.pattern.as_str(), path);
        
        // Check cache first
        if let Some(cached) = cache.get(&cache_key).await {
            return cached.matched;
        }
        
        // Check match
        let matched = self.matches(path);
        
        // Cache the result
        let cached = CachedRouteMatch {
            params: self.extract(path),
            matched,
        };
        cache.set(cache_key, cached).await;
        
        matched
    }
}

/// Route definition with priority
#[derive(Debug, Clone)]
pub struct RouteDefinition {
    pub path: String,
    pub method: String,
    pub priority: i32,
    pub handler: String, // Handler identifier
}

impl RouteDefinition {
    pub fn new(path: String, method: String, priority: i32, handler: String) -> Self {
        Self {
            path,
            method,
            priority,
            handler,
        }
    }
}

/// Route registry for managing routes with priority and conflict detection
pub struct RouteRegistry {
    routes: Vec<RouteDefinition>,
}

impl RouteRegistry {
    /// Create a new route registry
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
        }
    }

    /// Register a route with priority
    pub fn register(&mut self, path: String, method: String, priority: i32, handler: String) {
        self.routes.push(RouteDefinition::new(path, method, priority, handler));
    }

    /// Check for route conflicts (same path and method)
    pub fn check_conflicts(&self) -> Vec<(RouteDefinition, RouteDefinition)> {
        let mut conflicts = Vec::new();
        for (i, route1) in self.routes.iter().enumerate() {
            for route2 in self.routes.iter().skip(i + 1) {
                if route1.path == route2.path && route1.method == route2.method {
                    conflicts.push((route1.clone(), route2.clone()));
                }
            }
        }
        conflicts
    }

    /// Get routes sorted by priority (higher priority first)
    pub fn get_sorted_routes(&self) -> Vec<RouteDefinition> {
        let mut sorted = self.routes.clone();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));
        sorted
    }

    /// Get routes for a specific path, sorted by priority
    pub fn get_routes_for_path(&self, path: &str) -> Vec<RouteDefinition> {
        let mut matching = self.routes.iter()
            .filter(|r| r.path == path)
            .cloned()
            .collect::<Vec<_>>();
        matching.sort_by(|a, b| b.priority.cmp(&a.priority));
        matching
    }

    /// Validate routes and return any conflicts
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let conflicts = self.check_conflicts();
        if conflicts.is_empty() {
            Ok(())
        } else {
            let errors: Vec<String> = conflicts.iter()
                .map(|(r1, r2)| {
                    format!("Route conflict: {} {} (handlers: {} and {}, priorities: {} and {})",
                        r1.method, r1.path, r1.handler, r2.handler, r1.priority, r2.priority)
                })
                .collect();
            Err(errors)
        }
    }
}

impl Default for RouteRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Router helper functions
pub mod router_helpers {
    use super::*;
    
    /// Parse route pattern and extract parameter names
    pub fn parse_route_pattern(pattern: &str) -> RouteParams {
        RouteParams::from_pattern(pattern)
    }
    
    /// Detect route conflicts between two route patterns
    pub fn detect_conflict(pattern1: &str, method1: &str, pattern2: &str, method2: &str) -> bool {
        if method1 != method2 {
            return false;
        }
        
        // Simple conflict detection: exact match or pattern overlap
        if pattern1 == pattern2 {
            return true;
        }
        
        // Check if patterns could match the same paths
        let params1 = RouteParams::from_pattern(pattern1);
        let params2 = RouteParams::from_pattern(pattern2);
        
        // Test with a few sample paths to detect potential conflicts
        let test_paths = vec!["/test", "/test/123", "/test/123/456"];
        for test_path in test_paths {
            if params1.matches(test_path) && params2.matches(test_path) {
                return true;
            }
        }
        
        false
    }
}

