//! # client
//!
//! client 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! gRPC client encapsulation

use rf_errors::Result;
use std::time::Duration;

/// gRPC client builder
pub struct GrpcClient {
    endpoint: String,
    timeout: Option<Duration>,
    retry_count: usize,
}

impl GrpcClient {
    /// Create a new gRPC client
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            timeout: Some(Duration::from_secs(30)),
            retry_count: 3,
        }
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set retry count
    pub fn with_retry(mut self, count: usize) -> Self {
        self.retry_count = count;
        self
    }

    /// Connect to the gRPC server and return a channel
    pub async fn connect(self) -> Result<tonic::transport::Channel> {
        let mut endpoint = tonic::transport::Endpoint::from_shared(self.endpoint)
            .map_err(|e| rf_errors::RfError::Network(format!("Invalid endpoint: {}", e)))?;

        if let Some(timeout) = self.timeout {
            endpoint = endpoint.timeout(timeout);
        }

        let channel = endpoint
            .connect()
            .await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to connect: {}", e)))?;

        Ok(channel)
    }
}

