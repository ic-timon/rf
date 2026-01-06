//! # consul
//!
//! consul 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Consul service registry adapter

use super::{ServiceInstance, ServiceRegistry, ServiceHealth};
use rf_errors::{Result, RfError};
use reqwest::Client;
use std::collections::HashMap;
use std::net::SocketAddr;

/// Consul service registry
pub struct ConsulRegistry {
    client: Client,
    base_url: String,
}

impl ConsulRegistry {
    /// Create a new Consul registry
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }
}

impl ServiceRegistry for ConsulRegistry {
    fn register(&self, instance: &ServiceInstance) -> Result<()> {
        let url = format!("{}/v1/agent/service/register", self.base_url);
        
        let payload = serde_json::json!({
            "ID": instance.id,
            "Name": instance.name,
            "Address": instance.address.ip().to_string(),
            "Port": instance.address.port(),
            "Tags": instance.metadata.keys().collect::<Vec<_>>(),
            "Meta": instance.metadata,
            "Check": {
                "HTTP": format!("http://{}:{}/health", instance.address.ip(), instance.address.port()),
                "Interval": "10s"
            }
        });
        
        let _response = futures::executor::block_on(
            self.client.put(&url).json(&payload).send()
        )
        .map_err(|e| RfError::Internal(format!("Consul register failed: {}", e)))?;
        
        Ok(())
    }
    
    fn deregister(&self, service_id: &str) -> Result<()> {
        let url = format!("{}/v1/agent/service/deregister/{}", self.base_url, service_id);
        
        let _response = futures::executor::block_on(
            self.client.put(&url).send()
        )
        .map_err(|e| RfError::Internal(format!("Consul deregister failed: {}", e)))?;
        
        Ok(())
    }
    
    fn discover(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        let url = format!("{}/v1/health/service/{}?passing=true", self.base_url, service_name);
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Internal(format!("Consul discover failed: {}", e)))?;
        
        let mut instances = Vec::new();
        
        if response.status().is_success() {
            let services: Vec<serde_json::Value> = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Internal(format!("Failed to parse Consul response: {}", e)))?;
            
            for service in services {
                if let (Some(id), Some(name), Some(service_data)) = (
                    service.get("Service").and_then(|s| s.get("ID")).and_then(|v| v.as_str()),
                    service.get("Service").and_then(|s| s.get("Service")).and_then(|v| v.as_str()),
                    service.get("Service"),
                ) {
                    let address = if let (Some(addr), Some(port)) = (
                        service_data.get("Address").and_then(|v| v.as_str()),
                        service_data.get("Port").and_then(|v| v.as_u64()),
                    ) {
                        format!("{}:{}", addr, port).parse::<SocketAddr>()
                            .map_err(|e| RfError::Internal(format!("Invalid address: {}", e)))?
                    } else {
                        continue;
                    };
                    
                    let mut metadata = HashMap::new();
                    if let Some(meta) = service_data.get("Meta").and_then(|m| m.as_object()) {
                        for (k, v) in meta {
                            if let Some(s) = v.as_str() {
                                metadata.insert(k.clone(), s.to_string());
                            }
                        }
                    }
                    
                    let health = if let Some(checks) = service.get("Checks").and_then(|c| c.as_array()) {
                        if checks.iter().any(|c| c.get("Status").and_then(|s| s.as_str()) == Some("passing")) {
                            ServiceHealth::Healthy
                        } else {
                            ServiceHealth::Unhealthy
                        }
                    } else {
                        ServiceHealth::Unknown
                    };
                    
                    instances.push(ServiceInstance {
                        id: id.to_string(),
                        name: name.to_string(),
                        address,
                        metadata,
                        health,
                    });
                }
            }
        }
        
        Ok(instances)
    }
    
    fn list_services(&self) -> Result<Vec<String>> {
        let url = format!("{}/v1/catalog/services", self.base_url);
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Internal(format!("Consul list services failed: {}", e)))?;
        
        if response.status().is_success() {
            let services: HashMap<String, Vec<String>> = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Internal(format!("Failed to parse Consul response: {}", e)))?;
            
            return Ok(services.keys().cloned().collect());
        }
        
        Ok(Vec::new())
    }
    
    fn watch<F>(&self, _service_name: &str, _callback: F) -> Result<()>
    where
        F: Fn(Vec<ServiceInstance>) -> Result<()> + Send + Sync + 'static,
    {
        // Consul watch implementation would use blocking query with index
        // This is a placeholder
        Ok(())
    }
}

