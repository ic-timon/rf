//! # middleware
//!
//! middleware 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! gRPC middleware

use tower::Service;
use tonic::{Request, Status};
use std::task::{Context, Poll};
use tracing::info;

/// Logging middleware for gRPC
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> LoggingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for LoggingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = tonic::Response<ResBody>>,
    S::Error: Into<Status>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        // Note: tonic::Request doesn't have uri() method
        // Use metadata or extensions to get method information
        if let Some(path) = req.metadata().get(":path") {
            if let Ok(path_str) = path.to_str() {
                info!("gRPC request: {}", path_str);
            }
        }
        self.inner.call(req)
    }
}

/// Tracing middleware for gRPC (placeholder)
pub struct TracingMiddleware<S> {
    inner: S,
}

impl<S> TracingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for TracingMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = tonic::Response<ResBody>>,
    S::Error: Into<Status>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        // In full implementation, this would add OpenTelemetry tracing
        self.inner.call(req)
    }
}

