//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Service registry and discovery
//!
//! Provides service registry adapters for:
//! - Consul
//! - etcd
//! - Nacos
//! - File-based registry

pub mod consul;
pub mod etcd;
pub mod nacos;
pub mod file;
pub mod zookeeper;

pub use consul::*;
pub use etcd::*;
pub use nacos::*;
pub use file::*;
pub use zookeeper::*;

use rf_errors::Result;
use std::collections::HashMap;
use std::net::SocketAddr;

/// Service instance information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub address: SocketAddr,
    pub metadata: HashMap<String, String>,
    pub health: ServiceHealth,
}

/// Service health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ServiceHealth {
    Healthy,
    Unhealthy,
    Unknown,
}

/// Service registry trait
pub trait ServiceRegistry: Send + Sync {
    /// Register a service instance
    fn register(&self, instance: &ServiceInstance) -> Result<()>;
    
    /// Deregister a service instance
    fn deregister(&self, service_id: &str) -> Result<()>;
    
    /// Discover service instances by name
    fn discover(&self, service_name: &str) -> Result<Vec<ServiceInstance>>;
    
    /// Get all registered services
    fn list_services(&self) -> Result<Vec<String>>;
    
    /// Watch for service changes
    fn watch<F>(&self, service_name: &str, callback: F) -> Result<()>
    where
        F: Fn(Vec<ServiceInstance>) -> Result<()> + Send + Sync + 'static;
}

