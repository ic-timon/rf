//! # server
//!
//! server 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! gRPC server encapsulation

use rf_errors::Result;
use std::net::SocketAddr;
use tokio::signal;

/// gRPC server builder
pub struct GrpcServer {
    addr: SocketAddr,
    shutdown_timeout: Option<std::time::Duration>,
}

impl GrpcServer {
    /// Create a new gRPC server
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            shutdown_timeout: Some(std::time::Duration::from_secs(30)),
        }
    }

    /// Add a service to the server
    /// Note: This is a simplified implementation. For full gRPC support,
    /// use tonic's service registration directly with Server::builder().
    pub fn add_service<S>(self, _service: S) -> Self
    where
        S: Send + Sync + 'static,
    {
        // Simplified: store service registration logic
        // In a full implementation, this would properly register the service
        self
    }

    /// Set shutdown timeout
    pub fn with_shutdown_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.shutdown_timeout = Some(timeout);
        self
    }

    /// Start the server
    /// Note: This is a placeholder implementation. In a full implementation,
    /// you would register services using Server::builder().add_service().
    pub async fn serve(self) -> Result<()> {
        // Create a TcpListener for the address
        let _listener = tokio::net::TcpListener::bind(self.addr).await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to bind to {}: {}", self.addr, e)))?;
        
        // Build server (simplified - no services registered)
        // Note: In a full implementation, you would use Server::builder().add_service() to register services
        // and then call server.serve() or server.serve_with_incoming()
        
        // Start server with graceful shutdown
        // This is a placeholder - in full implementation, use server.serve() or server.serve_with_incoming()
        tokio::select! {
            _ = async {
                // Placeholder: wait for shutdown signal
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            } => {
                tracing::info!("gRPC server placeholder - use Server::builder() directly for full functionality");
            }
            _ = signal::ctrl_c() => {
                tracing::info!("Received shutdown signal, shutting down gracefully");
            }
        }

        Ok(())
    }
}

