//! # static_files
//!
//! static_files 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Static file serving

use std::path::PathBuf;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

/// Static file server configuration
pub struct StaticFileServer {
    path: PathBuf,
    prefix: String,
}

impl StaticFileServer {
    /// Create a new static file server
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            prefix: "/".to_string(),
        }
    }

    /// Set URL prefix
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    /// Get the path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the prefix
    pub fn prefix_str(&self) -> &str {
        &self.prefix
    }
}

/// Create a static file service router
pub fn create_static_service(server: StaticFileServer) -> axum::Router {
    let serve_dir = ServeDir::new(server.path())
        .precompressed_gzip()
        .precompressed_br();

    axum::Router::new()
        .route_service(&format!("{}*path", server.prefix_str()), serve_dir)
        .layer(TraceLayer::new_for_http())
}

/// Static file cache configuration
pub struct StaticFileCache {
    max_age: Option<std::time::Duration>,
    etag: bool,
    cache_control: Option<String>,
}

impl StaticFileCache {
    /// Create a new static file cache configuration
    pub fn new() -> Self {
        Self {
            max_age: Some(std::time::Duration::from_secs(3600)), // 1 hour default
            etag: true,
            cache_control: None,
        }
    }

    /// Set max age for cache
    pub fn with_max_age(mut self, max_age: std::time::Duration) -> Self {
        self.max_age = Some(max_age);
        self
    }

    /// Enable/disable ETag
    pub fn with_etag(mut self, enabled: bool) -> Self {
        self.etag = enabled;
        self
    }

    /// Set cache control header
    pub fn with_cache_control(mut self, value: String) -> Self {
        self.cache_control = Some(value);
        self
    }
}

impl Default for StaticFileCache {
    fn default() -> Self {
        Self::new()
    }
}

