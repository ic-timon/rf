//! # zookeeper
//!
//! zookeeper 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! ZooKeeper service registry adapter

use super::{ServiceRegistry, ServiceInstance};
use rf_errors::{Result, RfError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// ZooKeeper client wrapper
/// Note: This is a simplified implementation. In production, you'd use a proper ZooKeeper client library
pub struct ZooKeeperClient {
    _connection_string: String,
    base_path: String,
    // In a real implementation, this would be a ZooKeeper client handle
    // For now, we'll use a placeholder that would need to be replaced with actual ZooKeeper client
}

impl ZooKeeperClient {
    /// Create a new ZooKeeper client
    pub fn new(connection_string: &str, base_path: &str) -> Self {
        Self {
            _connection_string: connection_string.to_string(),
            base_path: base_path.to_string(),
        }
    }

    /// Create a node path for a service
    fn service_path(&self, service_name: &str) -> String {
        format!("{}/{}", self.base_path, service_name)
    }

    /// Create a node path for a service instance
    fn instance_path(&self, service_name: &str, instance_id: &str) -> String {
        format!("{}/{}/{}", self.base_path, service_name, instance_id)
    }
}

/// ZooKeeper service registry
pub struct ZooKeeperRegistry {
    client: Arc<ZooKeeperClient>,
    instances: Arc<RwLock<HashMap<String, ServiceInstance>>>, // service_id -> instance
}

impl ZooKeeperRegistry {
    /// Create a new ZooKeeper registry
    pub fn new(connection_string: &str, base_path: &str) -> Self {
        Self {
            client: Arc::new(ZooKeeperClient::new(connection_string, base_path)),
            instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Serialize service instance to JSON
    fn serialize_instance(instance: &ServiceInstance) -> Result<String> {
        serde_json::to_string(instance)
            .map_err(|e| RfError::Internal(format!("Failed to serialize instance: {}", e)))
    }

    /// Deserialize service instance from JSON
    #[allow(dead_code)]
    fn deserialize_instance(data: &str) -> Result<ServiceInstance> {
        serde_json::from_str(data)
            .map_err(|e| RfError::Internal(format!("Failed to deserialize instance: {}", e)))
    }

    /// Create base path if it doesn't exist (simplified - would use actual ZooKeeper API)
    async fn ensure_base_path(&self) -> Result<()> {
        // In a real implementation, this would create the path in ZooKeeper
        // For now, this is a placeholder
        Ok(())
    }

    /// Create service path if it doesn't exist
    async fn ensure_service_path(&self, _service_name: &str) -> Result<()> {
        // In a real implementation, this would create the service path in ZooKeeper
        Ok(())
    }
}

impl ServiceRegistry for ZooKeeperRegistry {
    fn register(&self, instance: &ServiceInstance) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RfError::Internal(format!("Failed to create runtime: {}", e)))?;
        
        rt.block_on(async {
            // Ensure base path exists
            self.ensure_base_path().await?;
            self.ensure_service_path(&instance.name).await?;
            
            // Serialize instance data
            let _data = Self::serialize_instance(instance)?;
            let path = self.client.instance_path(&instance.name, &instance.id);
            
            // In a real implementation, this would:
            // 1. Create an ephemeral node at the path
            // 2. Store the instance data as the node value
            // 3. Set up a watcher to handle connection loss
            
            // For now, we'll just store it in memory
            let mut instances = self.instances.write().await;
            instances.insert(instance.id.clone(), instance.clone());
            
            tracing::info!("Registered service instance {} at {}", instance.id, path);
            Ok(())
        })
    }
    
    fn deregister(&self, service_id: &str) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RfError::Internal(format!("Failed to create runtime: {}", e)))?;
        
        rt.block_on(async {
            // In a real implementation, this would delete the ephemeral node
            let mut instances = self.instances.write().await;
            if let Some(instance) = instances.remove(service_id) {
                let path = self.client.instance_path(&instance.name, service_id);
                tracing::info!("Deregistered service instance {} at {}", service_id, path);
            }
            Ok(())
        })
    }
    
    fn discover(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RfError::Internal(format!("Failed to create runtime: {}", e)))?;
        
        rt.block_on(async {
            let _service_path = self.client.service_path(service_name);
            
            // In a real implementation, this would:
            // 1. List all children of the service path
            // 2. Read the data from each child node
            // 3. Deserialize and return the instances
            
            // For now, we'll filter from memory
            let instances = self.instances.read().await;
            let result: Vec<ServiceInstance> = instances.values()
                .filter(|instance| instance.name == service_name)
                .cloned()
                .collect();
            
            Ok(result)
        })
    }
    
    fn list_services(&self) -> Result<Vec<String>> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RfError::Internal(format!("Failed to create runtime: {}", e)))?;
        
        rt.block_on(async {
            // In a real implementation, this would list all children of the base path
            let instances = self.instances.read().await;
            let mut services: std::collections::HashSet<String> = std::collections::HashSet::new();
            for instance in instances.values() {
                services.insert(instance.name.clone());
            }
            Ok(services.into_iter().collect())
        })
    }
    
    fn watch<F>(&self, service_name: &str, callback: F) -> Result<()>
    where
        F: Fn(Vec<ServiceInstance>) -> Result<()> + Send + Sync + 'static,
    {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RfError::Internal(format!("Failed to create runtime: {}", e)))?;
        
        let client = self.client.clone();
        let instances = self.instances.clone();
        let callback = Arc::new(callback);
        let service_name = service_name.to_string();
        
        rt.spawn(async move {
            let mut last_instances: Vec<ServiceInstance> = Vec::new();
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
            
            loop {
                interval.tick().await;
                
                // Create a temporary registry for discovery
                let temp_registry = ZooKeeperRegistry {
                    client: client.clone(),
                    instances: instances.clone(),
                };
                match temp_registry.discover(&service_name) {
                    Ok(new_instances) => {
                        // Check if instances changed (compare by length and IDs)
                        let changed = new_instances.len() != last_instances.len() ||
                            new_instances.iter().any(|i| !last_instances.iter().any(|li| li.id == i.id));
                        if changed {
                            if let Err(e) = callback(new_instances.clone()) {
                                tracing::warn!("Watch callback error: {}", e);
                            }
                            last_instances = new_instances;
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to discover services: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }
}

// Note: ZooKeeperRegistry needs to be Clone for the watch implementation
// However, ServiceRegistry trait doesn't require Clone, so we'll use Arc instead

