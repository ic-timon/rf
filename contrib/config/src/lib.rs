//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Configuration center adapters
//!
//! Provides adapters for various configuration centers:
//! - Apollo
//! - Consul
//! - Nacos
//! - Kubernetes ConfigMap

pub mod apollo;
pub mod consul;
pub mod nacos;
pub mod k8s;
pub mod polaris;

pub use apollo::*;
pub use consul::*;
pub use nacos::*;
pub use k8s::*;
pub use polaris::*;

use rf_errors::Result;
use std::collections::HashMap;

/// Configuration center adapter trait
pub trait ConfigCenterAdapter: Send + Sync {
    /// Get configuration value
    fn get(&self, key: &str) -> Result<Option<String>>;
    
    /// Set configuration value
    fn set(&self, key: &str, value: &str) -> Result<()>;
    
    /// Get all configuration
    fn all(&self) -> Result<HashMap<String, String>>;
    
    /// Watch for configuration changes
    fn watch<F>(&self, callback: F) -> Result<()>
    where
        F: Fn(&str, &str) -> Result<()> + Send + Sync + 'static;
}

