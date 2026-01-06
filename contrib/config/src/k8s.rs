//! # k8s
//!
//! k8s 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Kubernetes ConfigMap adapter

use super::ConfigCenterAdapter;
use rf_errors::{Result, RfError};
use std::collections::HashMap;
use std::sync::Arc;
use reqwest::Client;
use tokio::sync::RwLock;
use serde_json;

/// Kubernetes ConfigMap adapter
pub struct K8sConfigMapAdapter {
    client: Client,
    api_server: String,
    namespace: String,
    name: String,
    resource_version: Arc<RwLock<Option<String>>>,
}

impl K8sConfigMapAdapter {
    /// Create a new K8s ConfigMap adapter
    /// 
    /// # Parameters
    /// - `api_server`: Kubernetes API server URL (e.g., "https://kubernetes.default.svc")
    /// - `namespace`: Namespace where the ConfigMap is located
    /// - `name`: Name of the ConfigMap
    pub fn new(api_server: &str, namespace: &str, name: &str) -> Self {
        Self {
            client: Client::new(),
            api_server: api_server.to_string(),
            namespace: namespace.to_string(),
            name: name.to_string(),
            resource_version: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Watch ConfigMap using Kubernetes Watch API
    async fn watch_configmap(&self, resource_version: Option<&str>, timeout: u64) -> Result<(Option<String>, HashMap<String, String>)> {
        let url = format!(
            "{}/api/v1/namespaces/{}/configmaps/{}",
            self.api_server, self.namespace, self.name
        );
        
        let mut request = self.client.get(&url);
        
        // Add watch parameters
        if let Some(rv) = resource_version {
            request = request.query(&[("watch", "true"), ("resourceVersion", rv), ("timeoutSeconds", &timeout.to_string())]);
        } else {
            request = request.query(&[("watch", "true"), ("timeoutSeconds", &timeout.to_string())]);
        }
        
        // Note: In production, you would need to handle authentication (service account token, etc.)
        // This is a simplified implementation
        
        let response = request
            .send()
            .await
            .map_err(|e| RfError::Config(format!("K8s watch request failed: {}", e)))?;
        
        let mut result = HashMap::new();
        let mut new_resource_version = resource_version.map(|s| s.to_string());
        
        if response.status().is_success() {
            let json: serde_json::Value = response
                .json()
                .await
                .map_err(|e| RfError::Config(format!("Failed to parse K8s response: {}", e)))?;
            
            // Extract resource version
            if let Some(rv) = json.get("metadata").and_then(|m| m.get("resourceVersion")).and_then(|v| v.as_str()) {
                new_resource_version = Some(rv.to_string());
            }
            
            // Extract data from ConfigMap
            if let Some(data) = json.get("data").and_then(|d| d.as_object()) {
                for (key, value) in data {
                    if let Some(v) = value.as_str() {
                        result.insert(key.clone(), v.to_string());
                    }
                }
            }
        }
        
        Ok((new_resource_version, result))
    }
}

impl ConfigCenterAdapter for K8sConfigMapAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        let url = format!(
            "{}/api/v1/namespaces/{}/configmaps/{}",
            self.api_server, self.namespace, self.name
        );
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("K8s request failed: {}", e)))?;
        
        if response.status().is_success() {
            let json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Config(format!("Failed to parse K8s response: {}", e)))?;
            
            if let Some(data) = json.get("data").and_then(|d| d.get(key)) {
                if let Some(value) = data.as_str() {
                    return Ok(Some(value.to_string()));
                }
            }
        }
        
        Ok(None)
    }
    
    fn set(&self, key: &str, value: &str) -> Result<()> {
        // First get current ConfigMap
        let url = format!(
            "{}/api/v1/namespaces/{}/configmaps/{}",
            self.api_server, self.namespace, self.name
        );
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("K8s get failed: {}", e)))?;
        
        if response.status().is_success() {
            let mut json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Config(format!("Failed to parse K8s response: {}", e)))?;
            
            // Update data field
            if let Some(data) = json.get_mut("data") {
                if let Some(data_obj) = data.as_object_mut() {
                    data_obj.insert(key.to_string(), serde_json::Value::String(value.to_string()));
                }
            }
            
            // Update ConfigMap
            let _response = futures::executor::block_on(
                self.client.put(&url).json(&json).send()
            )
            .map_err(|e| RfError::Config(format!("K8s update failed: {}", e)))?;
        }
        
        Ok(())
    }
    
    fn all(&self) -> Result<HashMap<String, String>> {
        let url = format!(
            "{}/api/v1/namespaces/{}/configmaps/{}",
            self.api_server, self.namespace, self.name
        );
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("K8s request failed: {}", e)))?;
        
        let mut result = HashMap::new();
        
        if response.status().is_success() {
            let json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Config(format!("Failed to parse K8s response: {}", e)))?;
            
            if let Some(data) = json.get("data").and_then(|d| d.as_object()) {
                for (key, value) in data {
                    if let Some(v) = value.as_str() {
                        result.insert(key.clone(), v.to_string());
                    }
                }
            }
        }
        
        Ok(result)
    }
    
    fn watch<F>(&self, callback: F) -> Result<()>
    where
        F: Fn(&str, &str) -> Result<()> + Send + Sync + 'static,
    {
        let adapter = Arc::new(self.clone());
        let callback = Arc::new(callback);
        
        tokio::spawn(async move {
            let mut last_config = HashMap::new();
            let mut resource_version = {
                let rv = adapter.resource_version.read().await;
                rv.clone()
            };
            
            loop {
                let rv_str = resource_version.as_deref();
                match adapter.watch_configmap(rv_str, 60).await {
                    Ok((new_rv, config)) => {
                        // Update resource version
                        if let Some(ref rv) = new_rv {
                            resource_version = Some(rv.clone());
                            let mut rv_guard = adapter.resource_version.write().await;
                            *rv_guard = Some(rv.clone());
                        }
                        
                        // Check for changes
                        for (key, value) in &config {
                            if let Some(old_value) = last_config.get(key) {
                                if old_value != value {
                                    let _ = callback(key, value);
                                }
                            } else {
                                // New key
                                let _ = callback(key, value);
                            }
                        }
                        
                        // Check for deleted keys
                        let deleted_keys: Vec<String> = last_config.keys()
                            .filter(|k| !config.contains_key(*k))
                            .cloned()
                            .collect();
                        for key in deleted_keys {
                            let _ = callback(&key, "");
                        }
                        
                        last_config = config;
                    }
                    Err(e) => {
                        tracing::warn!("K8s watch failed: {}", e);
                        // Wait before retrying
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        });
        
        Ok(())
    }
}

// Note: K8sConfigMapAdapter needs to be Clone for the watch implementation
impl Clone for K8sConfigMapAdapter {
    fn clone(&self) -> Self {
        Self {
            client: Client::new(),
            api_server: self.api_server.clone(),
            namespace: self.namespace.clone(),
            name: self.name.clone(),
            resource_version: Arc::new(RwLock::new(None)),
        }
    }
}

