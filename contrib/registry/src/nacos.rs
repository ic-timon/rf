//! # nacos
//!
//! nacos 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Nacos service registry adapter

use super::{ServiceInstance, ServiceRegistry, ServiceHealth};
use rf_errors::{Result, RfError};
use reqwest::Client;
use std::collections::HashMap;
use std::net::SocketAddr;

/// Nacos service registry
pub struct NacosRegistry {
    client: Client,
    base_url: String,
    namespace: String,
}

impl NacosRegistry {
    /// Create a new Nacos registry
    pub fn new(base_url: &str, namespace: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            namespace: namespace.to_string(),
        }
    }
}

impl ServiceRegistry for NacosRegistry {
    fn register(&self, instance: &ServiceInstance) -> Result<()> {
        let url = format!("{}/nacos/v1/ns/instance", self.base_url);
        
        let params = [
            ("serviceName", instance.name.as_str()),
            ("ip", &instance.address.ip().to_string()),
            ("port", &instance.address.port().to_string()),
            ("namespaceId", &self.namespace),
            ("metadata", &serde_json::to_string(&instance.metadata).unwrap_or_default()),
        ];
        
        let _response = futures::executor::block_on(
            self.client.post(&url).form(&params).send()
        )
        .map_err(|e| RfError::Internal(format!("Nacos register failed: {}", e)))?;
        
        Ok(())
    }
    
    fn deregister(&self, _service_id: &str) -> Result<()> {
        // Nacos deregister requires service name and IP:port
        // This is a simplified implementation
        let url = format!("{}/nacos/v1/ns/instance", self.base_url);
        
        // Parse service_id to extract name and address
        // In real implementation, would need to store this mapping
        let _response = futures::executor::block_on(
            self.client.delete(&url).send()
        )
        .map_err(|e| RfError::Internal(format!("Nacos deregister failed: {}", e)))?;
        
        Ok(())
    }
    
    fn discover(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        let url = format!("{}/nacos/v1/ns/instance/list", self.base_url);
        
        let params = [
            ("serviceName", service_name),
            ("namespaceId", &self.namespace),
        ];
        
        let response = futures::executor::block_on(
            self.client.get(&url).query(&params).send()
        )
        .map_err(|e| RfError::Internal(format!("Nacos discover failed: {}", e)))?;
        
        let mut instances = Vec::new();
        
        if response.status().is_success() {
            let json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Internal(format!("Failed to parse Nacos response: {}", e)))?;
            
            if let Some(hosts) = json.get("hosts").and_then(|v| v.as_array()) {
                for host in hosts {
                    if let (Some(ip), Some(port), Some(instance_id)) = (
                        host.get("ip").and_then(|v| v.as_str()),
                        host.get("port").and_then(|v| v.as_u64()),
                        host.get("instanceId").and_then(|v| v.as_str()),
                    ) {
                        let address = format!("{}:{}", ip, port).parse::<SocketAddr>()
                            .map_err(|e| RfError::Internal(format!("Invalid address: {}", e)))?;
                        
                        let mut metadata = HashMap::new();
                        if let Some(meta) = host.get("metadata").and_then(|m| m.as_object()) {
                            for (k, v) in meta {
                                if let Some(s) = v.as_str() {
                                    metadata.insert(k.clone(), s.to_string());
                                }
                            }
                        }
                        
                        let health = if let Some(healthy) = host.get("healthy").and_then(|v| v.as_bool()) {
                            if healthy {
                                ServiceHealth::Healthy
                            } else {
                                ServiceHealth::Unhealthy
                            }
                        } else {
                            ServiceHealth::Unknown
                        };
                        
                        instances.push(ServiceInstance {
                            id: instance_id.to_string(),
                            name: service_name.to_string(),
                            address,
                            metadata,
                            health,
                        });
                    }
                }
            }
        }
        
        Ok(instances)
    }
    
    fn list_services(&self) -> Result<Vec<String>> {
        let url = format!("{}/nacos/v1/ns/service/list", self.base_url);
        
        let params = [("namespaceId", self.namespace.as_str())];
        
        let response = futures::executor::block_on(
            self.client.get(&url).query(&params).send()
        )
        .map_err(|e| RfError::Internal(format!("Nacos list services failed: {}", e)))?;
        
        if response.status().is_success() {
            let json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Internal(format!("Failed to parse Nacos response: {}", e)))?;
            
            if let Some(services) = json.get("doms").and_then(|v| v.as_array()) {
                return Ok(services.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect());
            }
        }
        
        Ok(Vec::new())
    }
    
    fn watch<F>(&self, _service_name: &str, _callback: F) -> Result<()>
    where
        F: Fn(Vec<ServiceInstance>) -> Result<()> + Send + Sync + 'static,
    {
        // Nacos watch implementation would use long polling
        // This is a placeholder
        Ok(())
    }
}

