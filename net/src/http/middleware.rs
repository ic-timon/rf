//! # middleware
//!
//! middleware 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP middleware system

use axum::extract::Request;
use axum::response::Response;
use std::future::Future;
use std::pin::Pin;

/// Type alias for middleware handler future
type MiddlewareFuture = Pin<Box<dyn Future<Output = Result<Response, axum::Error>> + Send>>;

/// Type alias for middleware handler function
type MiddlewareHandler = Box<dyn Fn(Request) -> MiddlewareFuture + Send + Sync>;

/// Middleware trait for HTTP request/response processing
pub trait Middleware: Clone + Send + Sync + 'static {
    /// Process the request and response
    fn process(
        &self,
        request: Request,
        next: Next,
    ) -> MiddlewareFuture;
}

/// Next middleware in the chain
pub struct Next {
    inner: MiddlewareHandler,
}

impl Next {
    /// Create a new Next middleware
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, axum::Error>> + Send + 'static,
    {
        Self {
            inner: Box::new(move |req| Box::pin(f(req))),
        }
    }

    /// Call the next middleware
    pub async fn call(self, request: Request) -> Result<Response, axum::Error> {
        (self.inner)(request).await
    }
}

/// Logging middleware
#[derive(Clone)]
pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn process(
        &self,
        request: Request,
        next: Next,
    ) -> Pin<Box<dyn Future<Output = Result<Response, axum::Error>> + Send>> {
        Box::pin(async move {
            let method = request.method().clone();
            let uri = request.uri().clone();
            tracing::info!("{} {}", method, uri);
            let response = next.call(request).await?;
            tracing::info!("Response status: {}", response.status());
            Ok(response)
        })
    }
}

/// CORS middleware
#[derive(Clone)]
pub struct CorsMiddleware {
    allow_origin: String,
    allow_methods: String,
    allow_headers: String,
}

impl CorsMiddleware {
    /// Create a new CORS middleware
    pub fn new() -> Self {
        Self {
            allow_origin: "*".to_string(),
            allow_methods: "GET, POST, PUT, DELETE, OPTIONS".to_string(),
            allow_headers: "Content-Type, Authorization".to_string(),
        }
    }

    /// Set allowed origin
    pub fn allow_origin(mut self, origin: &str) -> Self {
        self.allow_origin = origin.to_string();
        self
    }

    /// Set allowed methods
    pub fn allow_methods(mut self, methods: &str) -> Self {
        self.allow_methods = methods.to_string();
        self
    }

    /// Set allowed headers
    pub fn allow_headers(mut self, headers: &str) -> Self {
        self.allow_headers = headers.to_string();
        self
    }
}

impl Default for CorsMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl Middleware for CorsMiddleware {
    fn process(
        &self,
        request: Request,
        next: Next,
    ) -> Pin<Box<dyn Future<Output = Result<Response, axum::Error>> + Send>> {
        let allow_origin = self.allow_origin.clone();
        let allow_methods = self.allow_methods.clone();
        let allow_headers = self.allow_headers.clone();
        
        Box::pin(async move {
            // Handle preflight OPTIONS request
            if request.method() == axum::http::Method::OPTIONS {
                let mut response = Response::new(axum::body::Body::empty());
                let headers = response.headers_mut();
                headers.insert(
                    axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    allow_origin.parse().unwrap(),
                );
                headers.insert(
                    axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                    allow_methods.parse().unwrap(),
                );
                headers.insert(
                    axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                    allow_headers.parse().unwrap(),
                );
                return Ok(response);
            }

            let mut response = next.call(request).await?;
            let headers = response.headers_mut();
            headers.insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                allow_origin.parse().unwrap(),
            );
            headers.insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
                allow_methods.parse().unwrap(),
            );
            headers.insert(
                axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                allow_headers.parse().unwrap(),
            );
            Ok(response)
        })
    }
}
