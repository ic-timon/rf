//! # interceptor
//!
//! interceptor 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Request/Response interceptor system

use axum::extract::Request;
use axum::response::Response;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Request interceptor function type
pub type RequestInterceptor = Arc<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Request, axum::Error>> + Send>> + Send + Sync>;

/// Response interceptor function type
pub type ResponseInterceptor = Arc<dyn Fn(Response) -> Pin<Box<dyn Future<Output = Result<Response, axum::Error>> + Send>> + Send + Sync>;

/// Interceptor manager
pub struct InterceptorManager {
    request_interceptors: Vec<RequestInterceptor>,
    response_interceptors: Vec<ResponseInterceptor>,
}

impl InterceptorManager {
    /// Create a new interceptor manager
    pub fn new() -> Self {
        Self {
            request_interceptors: Vec::new(),
            response_interceptors: Vec::new(),
        }
    }

    /// Add a request interceptor
    pub fn add_request_interceptor<F, Fut>(&mut self, interceptor: F)
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Request, axum::Error>> + Send + 'static,
    {
        self.request_interceptors.push(Arc::new(move |req| Box::pin(interceptor(req))));
    }

    /// Add a response interceptor
    pub fn add_response_interceptor<F, Fut>(&mut self, interceptor: F)
    where
        F: Fn(Response) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response, axum::Error>> + Send + 'static,
    {
        self.response_interceptors.push(Arc::new(move |res| Box::pin(interceptor(res))));
    }

    /// Apply request interceptors
    pub async fn intercept_request(&self, mut request: Request) -> Result<Request, axum::Error> {
        for interceptor in &self.request_interceptors {
            request = interceptor(request).await?;
        }
        Ok(request)
    }

    /// Apply response interceptors
    pub async fn intercept_response(&self, mut response: Response) -> Result<Response, axum::Error> {
        for interceptor in &self.response_interceptors {
            response = interceptor(response).await?;
        }
        Ok(response)
    }

    /// Get the number of request interceptors
    pub fn request_interceptor_count(&self) -> usize {
        self.request_interceptors.len()
    }

    /// Get the number of response interceptors
    pub fn response_interceptor_count(&self) -> usize {
        self.response_interceptors.len()
    }
}

impl Default for InterceptorManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a request interceptor that adds headers
pub fn add_header_interceptor(name: &'static str, value: &'static str) -> RequestInterceptor {
    Arc::new(move |mut request| {
        let name = name.to_string();
        let value = value.to_string();
        Box::pin(async move {
            request.headers_mut().insert(
                axum::http::HeaderName::from_bytes(name.as_bytes()).unwrap(),
                axum::http::HeaderValue::from_str(&value).unwrap(),
            );
            Ok(request)
        })
    })
}

/// Helper function to create a response interceptor that modifies status code
pub fn status_code_interceptor(status: axum::http::StatusCode) -> ResponseInterceptor {
    Arc::new(move |mut response| {
        let status = status;
        Box::pin(async move {
            *response.status_mut() = status;
            Ok(response)
        })
    })
}

