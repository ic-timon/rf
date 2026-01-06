//! # rewrite
//!
//! rewrite 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP URL rewrite middleware

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use regex::Regex;
use std::sync::Arc;

/// URL rewrite rule
#[derive(Debug, Clone)]
pub struct RewriteRule {
    pattern: Regex,
    replacement: String,
    condition: Option<Arc<dyn Fn(&Request) -> bool + Send + Sync>>,
}

impl RewriteRule {
    /// Create a new rewrite rule
    pub fn new(pattern: &str, replacement: &str) -> Result<Self, regex::Error> {
        let regex_pattern = format!("^{}$", pattern);
        let pattern = Regex::new(&regex_pattern)?;
        Ok(Self {
            pattern,
            replacement: replacement.to_string(),
            condition: None,
        })
    }

    /// Create a rewrite rule with condition
    pub fn with_condition<F>(mut self, condition: F) -> Self
    where
        F: Fn(&Request) -> bool + Send + Sync + 'static,
    {
        self.condition = Some(Arc::new(condition));
        self
    }

    /// Apply rewrite rule to a path
    pub fn rewrite(&self, path: &str, request: &Request) -> Option<String> {
        // Check condition if present
        if let Some(ref condition) = self.condition {
            if !condition(request) {
                return None;
            }
        }

        // Apply regex replacement
        if self.pattern.is_match(path) {
            let result = self.pattern.replace(path, &self.replacement);
            Some(result.to_string())
        } else {
            None
        }
    }
}

/// URL rewrite middleware
pub struct RewriteMiddleware {
    rules: Vec<RewriteRule>,
}

impl RewriteMiddleware {
    /// Create a new rewrite middleware
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    /// Add a rewrite rule
    pub fn add_rule(mut self, rule: RewriteRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Create middleware function
    pub fn middleware(self) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> + Clone {
        let rules = Arc::new(self.rules);
        move |mut request: Request, next: Next| {
            let rules = rules.clone();
            Box::pin(async move {
                // Get the path from the request URI
                let path = request.uri().path();
                
                // Try to apply rewrite rules
                for rule in rules.iter() {
                    if let Some(rewritten) = rule.rewrite(path, &request) {
                        // Update the request URI with the rewritten path
                        let mut parts = request.uri().clone().into_parts();
                        parts.path_and_query = rewritten.parse().ok();
                        if let Ok(new_uri) = axum::http::Uri::from_parts(parts) {
                            *request.uri_mut() = new_uri;
                        }
                        break;
                    }
                }
                
                next.run(request).await
            })
        }
    }
}

impl Default for RewriteMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a simple rewrite rule
pub fn rewrite_rule(pattern: &str, replacement: &str) -> Result<RewriteRule, regex::Error> {
    RewriteRule::new(pattern, replacement)
}

