//! # rate_limit
//!
//! rate_limit 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Request rate limiting

use axum::extract::Request;
use axum::response::Response;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

/// Rate limiter using token bucket algorithm
pub struct RateLimiter {
    capacity: u64,
    tokens: Arc<Mutex<u64>>,
    refill_rate: u64, // tokens per second
    last_refill: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        Self {
            capacity,
            tokens: Arc::new(Mutex::new(capacity)),
            refill_rate,
            last_refill: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Check if a request is allowed
    pub async fn allow(&self) -> bool {
        let mut tokens = self.tokens.lock().await;
        let mut last_refill = self.last_refill.lock().await;
        
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate as f64) as u64;
        
        if tokens_to_add > 0 {
            *tokens = (*tokens + tokens_to_add).min(self.capacity);
            *last_refill = now;
        }
        
        if *tokens > 0 {
            *tokens -= 1;
            true
        } else {
            false
        }
    }
}

/// Rate limit middleware
pub async fn rate_limit_middleware(
    limiter: Arc<RateLimiter>,
    request: Request,
    next: axum::middleware::Next,
) -> Result<Response, axum::Error> {
    if limiter.allow().await {
        Ok(next.run(request).await)
    } else {
        let mut response = Response::new(axum::body::Body::empty());
        *response.status_mut() = axum::http::StatusCode::TOO_MANY_REQUESTS;
        Ok(response)
    }
}

