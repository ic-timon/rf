//! # adapter
//!
//! adapter 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Configuration adapter system

use rf_errors::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration adapter trait
pub trait ConfigAdapter: Send + Sync {
    /// Get configuration value
    fn get(&self, key: &str) -> Result<Option<String>>;
    
    /// Set configuration value
    fn set(&self, key: &str, value: &str) -> Result<()>;
    
    /// Get all configuration
    fn all(&self) -> Result<HashMap<String, String>>;
}

/// File-based configuration adapter
pub struct FileConfigAdapter {
    _path: String,
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl FileConfigAdapter {
    /// Create a new file adapter
    pub fn new(path: &str) -> Result<Self> {
        let data = if std::path::Path::new(path).exists() {
            let _content = std::fs::read_to_string(path)
                .map_err(rf_errors::RfError::Io)?;
            // Simplified - would parse actual config format
            Arc::new(RwLock::new(HashMap::new()))
        } else {
            Arc::new(RwLock::new(HashMap::new()))
        };
        Ok(Self {
            _path: path.to_string(),
            data,
        })
    }
}

impl ConfigAdapter for FileConfigAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        let data = futures::executor::block_on(self.data.read());
        Ok(data.get(key).cloned())
    }
    
    fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut data = futures::executor::block_on(self.data.write());
        data.insert(key.to_string(), value.to_string());
        // Simplified - would write to file
        Ok(())
    }
    
    fn all(&self) -> Result<HashMap<String, String>> {
        let data = futures::executor::block_on(self.data.read());
        Ok(data.clone())
    }
}

/// Environment variable configuration adapter
pub struct EnvConfigAdapter;

impl EnvConfigAdapter {
    /// Create a new environment adapter
    pub fn new() -> Self {
        Self
    }
}

impl ConfigAdapter for EnvConfigAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        Ok(std::env::var(key).ok())
    }
    
    fn set(&self, _key: &str, _value: &str) -> Result<()> {
        Err(rf_errors::RfError::Config("Cannot set environment variables".to_string()))
    }
    
    fn all(&self) -> Result<HashMap<String, String>> {
        Ok(std::env::vars().collect())
    }
}

impl Default for EnvConfigAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory-based configuration adapter
pub struct MemoryConfigAdapter {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl MemoryConfigAdapter {
    /// Create a new memory adapter
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl ConfigAdapter for MemoryConfigAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        let data = futures::executor::block_on(self.data.read());
        Ok(data.get(key).cloned())
    }
    
    fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut data = futures::executor::block_on(self.data.write());
        data.insert(key.to_string(), value.to_string());
        Ok(())
    }
    
    fn all(&self) -> Result<HashMap<String, String>> {
        let data = futures::executor::block_on(self.data.read());
        Ok(data.clone())
    }
}

impl Default for MemoryConfigAdapter {
    fn default() -> Self {
        Self::new()
    }
}

