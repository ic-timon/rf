//! # status_handler
//!
//! status_handler 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Custom status code handlers

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;

/// Custom status code handler function type
pub type StatusHandler = Arc<dyn Fn() -> Response + Send + Sync + 'static>;

/// Status code handler manager
pub struct StatusHandlerManager {
    handlers: HashMap<StatusCode, StatusHandler>,
    default_handler: Option<StatusHandler>,
}

impl StatusHandlerManager {
    /// Create a new status handler manager
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            default_handler: None,
        }
    }

    /// Register a handler for a specific status code
    pub fn register(&mut self, status: StatusCode, handler: StatusHandler) {
        self.handlers.insert(status, handler);
    }

    /// Register a default handler for unhandled status codes
    pub fn register_default(&mut self, handler: StatusHandler) {
        self.default_handler = Some(handler);
    }

    /// Handle a status code, returning a response
    pub fn handle(&self, status: StatusCode) -> Response {
        if let Some(handler) = self.handlers.get(&status) {
            handler()
        } else if let Some(ref default_handler) = self.default_handler {
            default_handler()
        } else {
            // Default response
            status.into_response()
        }
    }

    /// Check if a status code has a custom handler
    pub fn has_handler(&self, status: StatusCode) -> bool {
        self.handlers.contains_key(&status)
    }
}

impl Default for StatusHandlerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a JSON error response handler
pub fn json_error_handler<T: Serialize>(data: T) -> StatusHandler {
    Arc::new(move || {
        axum::response::Json(data).into_response()
    })
}

/// Create a text error response handler
pub fn text_error_handler(message: String) -> StatusHandler {
    Arc::new(move || {
        (StatusCode::INTERNAL_SERVER_ERROR, message.clone()).into_response()
    })
}

/// Create an HTML error response handler
pub fn html_error_handler(html: String) -> StatusHandler {
    Arc::new(move || {
        axum::response::Html(html.clone()).into_response()
    })
}

