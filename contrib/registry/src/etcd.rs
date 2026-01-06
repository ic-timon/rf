//! # etcd
//!
//! etcd 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! etcd service registry adapter

use super::{ServiceInstance, ServiceRegistry};
use rf_errors::{Result, RfError};
use reqwest::Client;

/// etcd service registry
pub struct EtcdRegistry {
    client: Client,
    base_url: String,
    prefix: String,
}

impl EtcdRegistry {
    /// Create a new etcd registry
    pub fn new(base_url: &str, prefix: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            prefix: prefix.to_string(),
        }
    }
}

impl ServiceRegistry for EtcdRegistry {
    fn register(&self, instance: &ServiceInstance) -> Result<()> {
        let key = format!("{}/{}/{}", self.prefix, instance.name, instance.id);
        let value = serde_json::to_string(instance)
            .map_err(|e| RfError::Internal(format!("Failed to serialize instance: {}", e)))?;
        
        let url = format!("{}/v3/kv/put", self.base_url);
        
        use base64::{Engine as _, engine::general_purpose};
        let payload = serde_json::json!({
            "key": general_purpose::STANDARD.encode(key.as_bytes()),
            "value": general_purpose::STANDARD.encode(value.as_bytes()),
        });
        
        let _response = futures::executor::block_on(
            self.client.post(&url).json(&payload).send()
        )
        .map_err(|e| RfError::Internal(format!("etcd register failed: {}", e)))?;
        
        Ok(())
    }
    
    fn deregister(&self, service_id: &str) -> Result<()> {
        // In etcd, we need to know the service name to construct the key
        // This is a simplified implementation
        let key = format!("{}/{}", self.prefix, service_id);
        
        let url = format!("{}/v3/kv/deleterange", self.base_url);
        
        use base64::{Engine as _, engine::general_purpose};
        let payload = serde_json::json!({
            "key": general_purpose::STANDARD.encode(key.as_bytes()),
        });
        
        let _response = futures::executor::block_on(
            self.client.post(&url).json(&payload).send()
        )
        .map_err(|e| RfError::Internal(format!("etcd deregister failed: {}", e)))?;
        
        Ok(())
    }
    
    fn discover(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        let key = format!("{}/{}", self.prefix, service_name);
        
        let url = format!("{}/v3/kv/range", self.base_url);
        
        use base64::{Engine as _, engine::general_purpose};
        let payload = serde_json::json!({
            "key": general_purpose::STANDARD.encode(key.as_bytes()),
            "range_end": general_purpose::STANDARD.encode(format!("{}/{}", key, "\0").as_bytes()),
        });
        
        let response = futures::executor::block_on(
            self.client.post(&url).json(&payload).send()
        )
        .map_err(|e| RfError::Internal(format!("etcd discover failed: {}", e)))?;
        
        let instances = Vec::new();
        
        if response.status().is_success() {
            let _json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Internal(format!("Failed to parse etcd response: {}", e)))?;
            // Simplified - in full implementation would parse kvs and extract instances
        }
        
        Ok(instances)
    }
    
    fn list_services(&self) -> Result<Vec<String>> {
        let url = format!("{}/v3/kv/range", self.base_url);
        
        use base64::{Engine as _, engine::general_purpose};
        let payload = serde_json::json!({
            "key": general_purpose::STANDARD.encode(self.prefix.as_bytes()),
            "range_end": general_purpose::STANDARD.encode(format!("{}/{}", self.prefix, "\0").as_bytes()),
        });
        
        let response = futures::executor::block_on(
            self.client.post(&url).json(&payload).send()
        )
        .map_err(|e| RfError::Internal(format!("etcd list services failed: {}", e)))?;
        
        let services = std::collections::HashSet::new();
        
        if response.status().is_success() {
            let _json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Internal(format!("Failed to parse etcd response: {}", e)))?;
            // Simplified - in full implementation would parse kvs and extract service names
        }
        
        Ok(services.into_iter().collect())
    }
    
    fn watch<F>(&self, _service_name: &str, _callback: F) -> Result<()>
    where
        F: Fn(Vec<ServiceInstance>) -> Result<()> + Send + Sync + 'static,
    {
        // etcd watch implementation would use watch API
        // This is a placeholder
        Ok(())
    }
}

