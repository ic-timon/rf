//! # file
//!
//! file 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! File-based service registry adapter

use super::{ServiceInstance, ServiceRegistry};
use rf_errors::{Result, RfError};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, RwLock};

/// File-based service registry
pub struct FileRegistry {
    path: String,
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
}

impl FileRegistry {
    /// Create a new file-based registry
    pub fn new(path: &str) -> Result<Self> {
        let services = if Path::new(path).exists() {
            let content = fs::read_to_string(path)
                .map_err(RfError::Io)?;
            let data: HashMap<String, Vec<ServiceInstance>> = serde_json::from_str(&content)
                .unwrap_or_default();
            Arc::new(RwLock::new(data))
        } else {
            // Create directory if needed
            if let Some(parent) = Path::new(path).parent() {
                fs::create_dir_all(parent)
                    .map_err(RfError::Io)?;
            }
            Arc::new(RwLock::new(HashMap::new()))
        };
        
        Ok(Self {
            path: path.to_string(),
            services,
        })
    }
    
    fn save(&self) -> Result<()> {
        let services = self.services.read()
            .map_err(|_| RfError::Internal("Failed to acquire read lock".to_string()))?;
        let content = serde_json::to_string_pretty(&*services)
            .map_err(|e| RfError::Internal(format!("Failed to serialize services: {}", e)))?;
        fs::write(&self.path, content)
            .map_err(RfError::Io)?;
        Ok(())
    }
}

impl ServiceRegistry for FileRegistry {
    fn register(&self, instance: &ServiceInstance) -> Result<()> {
        let mut services = self.services.write()
            .map_err(|_| RfError::Internal("Failed to acquire write lock".to_string()))?;
        
        let instances = services.entry(instance.name.clone()).or_insert_with(Vec::new);
        
        // Remove existing instance with same ID
        instances.retain(|i| i.id != instance.id);
        
        // Add new instance
        instances.push(instance.clone());
        
        drop(services);
        self.save()?;
        
        Ok(())
    }
    
    fn deregister(&self, service_id: &str) -> Result<()> {
        let mut services = self.services.write()
            .map_err(|_| RfError::Internal("Failed to acquire write lock".to_string()))?;
        
        for instances in services.values_mut() {
            instances.retain(|i| i.id != service_id);
        }
        
        drop(services);
        self.save()?;
        
        Ok(())
    }
    
    fn discover(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        let services = self.services.read()
            .map_err(|_| RfError::Internal("Failed to acquire read lock".to_string()))?;
        
        Ok(services.get(service_name).cloned().unwrap_or_default())
    }
    
    fn list_services(&self) -> Result<Vec<String>> {
        let services = self.services.read()
            .map_err(|_| RfError::Internal("Failed to acquire read lock".to_string()))?;
        
        Ok(services.keys().cloned().collect())
    }
    
    fn watch<F>(&self, _service_name: &str, _callback: F) -> Result<()>
    where
        F: Fn(Vec<ServiceInstance>) -> Result<()> + Send + Sync + 'static,
    {
        // File-based watch would use file system watcher
        // This is a placeholder
        Ok(())
    }
}

