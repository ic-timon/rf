//! # server_manager
//!
//! server_manager 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP server instance manager

use super::server::HttpServer;
use rf_errors::Result;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Server instance identifier
pub type ServerId = String;

/// Server configuration for registration
#[derive(Clone)]
pub struct ServerConfig {
    pub id: ServerId,
    pub addr: SocketAddr,
    pub router: axum::Router,
    pub shutdown_timeout: Option<std::time::Duration>,
    pub max_request_body_size: Option<usize>,
}

impl ServerConfig {
    pub fn new(id: ServerId, addr: SocketAddr) -> Self {
        Self {
            id,
            addr,
            router: axum::Router::new(),
            shutdown_timeout: Some(std::time::Duration::from_secs(30)),
            max_request_body_size: None,
        }
    }

    pub fn build(self) -> HttpServer {
        let mut server = HttpServer::new(self.addr);
        if let Some(timeout) = self.shutdown_timeout {
            server = server.shutdown_timeout(timeout);
        }
        if let Some(size) = self.max_request_body_size {
            server = server.max_request_body_size(size);
        }
        *server.router() = self.router;
        server
    }
}

/// HTTP server manager for managing multiple server instances
pub struct ServerManager {
    server_configs: Arc<RwLock<HashMap<ServerId, ServerConfig>>>,
    running: Arc<RwLock<HashMap<ServerId, tokio::task::JoinHandle<Result<()>>>>>,
}

impl ServerManager {
    /// Create a new server manager
    pub fn new() -> Self {
        Self {
            server_configs: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a server configuration
    pub async fn register(&self, config: ServerConfig) -> Result<()> {
        let id = config.id.clone();
        let mut configs = self.server_configs.write().await;
        configs.insert(id, config);
        Ok(())
    }

    /// Get a server configuration
    pub async fn get_config(&self, id: &str) -> Option<ServerConfig> {
        let configs = self.server_configs.read().await;
        configs.get(id).cloned()
    }

    /// Start a server instance
    pub async fn start(&self, id: &str) -> Result<()> {
        let config = {
            let configs = self.server_configs.read().await;
            configs.get(id).cloned()
        };

        if let Some(config) = config {
            let server = config.build();
            let handle = tokio::spawn(async move {
                server.serve().await
            });

            let mut running = self.running.write().await;
            running.insert(id.to_string(), handle);
            Ok(())
        } else {
            Err(rf_errors::RfError::Internal(format!("Server {} not found", id)))
        }
    }

    /// Stop a server instance
    pub async fn stop(&self, id: &str) -> Result<()> {
        let mut running = self.running.write().await;
        if let Some(handle) = running.remove(id) {
            handle.abort();
            Ok(())
        } else {
            Err(rf_errors::RfError::Internal(format!("Server {} is not running", id)))
        }
    }

    /// Start all registered servers
    pub async fn start_all(&self) -> Result<()> {
        let server_ids: Vec<String> = {
            let configs = self.server_configs.read().await;
            configs.keys().cloned().collect()
        };

        for id in server_ids {
            self.start(&id).await?;
        }

        Ok(())
    }

    /// Stop all running servers
    pub async fn stop_all(&self) -> Result<()> {
        let server_ids: Vec<String> = {
            let running = self.running.read().await;
            running.keys().cloned().collect()
        };

        for id in server_ids {
            self.stop(&id).await?;
        }

        Ok(())
    }

    /// Check if a server is running
    pub async fn is_running(&self, id: &str) -> bool {
        let running = self.running.read().await;
        running.contains_key(id)
    }

    /// Get list of all server IDs
    pub async fn list_servers(&self) -> Vec<ServerId> {
        let configs = self.server_configs.read().await;
        configs.keys().cloned().collect()
    }

    /// Get list of running server IDs
    pub async fn list_running(&self) -> Vec<ServerId> {
        let running = self.running.read().await;
        running.keys().cloned().collect()
    }

    /// Remove a server instance
    pub async fn remove(&self, id: &str) -> Result<()> {
        // Stop if running
        if self.is_running(id).await {
            self.stop(id).await?;
        }

        let mut configs = self.server_configs.write().await;
        configs.remove(id);
        Ok(())
    }
}

impl Default for ServerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global server manager instance
static GLOBAL_SERVER_MANAGER: once_cell::sync::LazyLock<ServerManager> = 
    once_cell::sync::LazyLock::new(|| ServerManager::new());

/// Get the global server manager
pub fn global_manager() -> &'static ServerManager {
    &GLOBAL_SERVER_MANAGER
}

