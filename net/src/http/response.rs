//! # response
//!
//! response 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP response wrapper

use axum::http::StatusCode;
use axum::response::{Html, Json as AxumJson, Response as AxumResponse};
use axum::response::IntoResponse;
use rf_errors::Result;
use serde::Serialize;

/// HTTP response wrapper
pub struct Response {
    inner: AxumResponse,
}

impl Response {
    /// Create a new response
    pub fn new(body: axum::body::Body) -> Self {
        Self {
            inner: AxumResponse::new(body),
        }
    }

    /// Create a JSON response
    pub fn json<T: Serialize>(data: &T) -> Result<Self> {
        Ok(Self {
            inner: AxumJson(data).into_response(),
        })
    }

    /// Create an HTML response
    pub fn html(html: impl Into<String>) -> Self {
        Self {
            inner: Html(html.into()).into_response(),
        }
    }

    /// Create a text response
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            inner: text.into().into_response(),
        }
    }

    /// Set status code
    pub fn status(mut self, status: StatusCode) -> Self {
        *self.inner.status_mut() = status;
        self
    }

    /// Get status code
    pub fn status_code(&self) -> StatusCode {
        self.inner.status()
    }

    /// Set header
    pub fn header(mut self, name: &str, value: &str) -> Result<Self> {
        let name = axum::http::HeaderName::from_bytes(name.as_bytes())
            .map_err(|e| rf_errors::RfError::Network(format!("Invalid header name: {}", e)))?;
        let value = axum::http::HeaderValue::from_str(value)
            .map_err(|e| rf_errors::RfError::Network(format!("Invalid header value: {}", e)))?;
        self.inner.headers_mut().insert(name, value);
        Ok(self)
    }

    /// Get the raw response
    pub fn into_inner(self) -> AxumResponse {
        self.inner
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> AxumResponse {
        self.inner
    }
}

impl From<AxumResponse> for Response {
    fn from(response: AxumResponse) -> Self {
        Self { inner: response }
    }
}
