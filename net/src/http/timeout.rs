//! # timeout
//!
//! timeout 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Request timeout control

use axum::extract::Request;
use axum::response::Response;
use std::time::Duration;
use tokio::time::timeout;

/// Timeout middleware
pub async fn timeout_middleware(
    duration: Duration,
    request: Request,
    next: axum::middleware::Next,
) -> Result<Response, axum::Error> {
    timeout(duration, next.run(request))
        .await
        .map_err(|_| {
            axum::Error::new(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Request timeout",
            ))
        })
}

