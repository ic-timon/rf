//! # polaris
//!
//! polaris 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Polaris configuration center adapter

use super::ConfigCenterAdapter;
use rf_errors::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use reqwest::Client;

/// Polaris configuration center adapter
pub struct PolarisAdapter {
    client: Client,
    server_url: String,
    namespace: String,
    file_group: String,
    file_name: String,
    cached_config: Arc<RwLock<HashMap<String, String>>>,
}

impl PolarisAdapter {
    /// Create a new Polaris adapter
    pub fn new(server_url: &str, namespace: &str, file_group: &str, file_name: &str) -> Self {
        Self {
            client: Client::new(),
            server_url: server_url.to_string(),
            namespace: namespace.to_string(),
            file_group: file_group.to_string(),
            file_name: file_name.to_string(),
            cached_config: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Build Polaris API URL for configuration
    fn build_url(&self, endpoint: &str) -> String {
        format!("{}/config/v1/{}/{}", self.server_url, endpoint, self.file_name)
    }

    /// Fetch configuration from Polaris
    async fn fetch_config(&self) -> Result<HashMap<String, String>> {
        let url = self.build_url(&format!("namespaces/{}/filegroups/{}/files", 
            self.namespace, self.file_group));
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to fetch from Polaris: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(rf_errors::RfError::Config(format!(
                "Polaris API returned error: {}", response.status()
            )));
        }

        let text = response.text().await
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to read response: {}", e)))?;
        
        // Parse configuration (Polaris typically returns JSON or properties format)
        // This is a simplified parser - full implementation would handle different formats
        let mut config = HashMap::new();
        
        // Try to parse as JSON first
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(obj) = json.as_object() {
                for (key, value) in obj {
                    if let Some(val_str) = value.as_str() {
                        config.insert(key.clone(), val_str.to_string());
                    } else {
                        config.insert(key.clone(), value.to_string());
                    }
                }
            }
        } else {
            // Fallback to properties format
            for line in text.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some(pos) = line.find('=') {
                    let key = line[..pos].trim().to_string();
                    let value = line[pos+1..].trim().to_string();
                    config.insert(key, value);
                }
            }
        }
        
        Ok(config)
    }

    /// Update configuration in Polaris
    async fn update_config(&self, key: &str, value: &str) -> Result<()> {
        // First, fetch current config
        let mut config = self.fetch_config().await?;
        config.insert(key.to_string(), value.to_string());
        
        // Convert back to properties format
        let mut content = String::new();
        for (k, v) in &config {
            content.push_str(&format!("{}={}\n", k, v));
        }
        
        let url = self.build_url(&format!("namespaces/{}/filegroups/{}/files", 
            self.namespace, self.file_group));
        
        let response = self.client
            .put(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "namespace": self.namespace,
                "file_group": self.file_group,
                "file_name": self.file_name,
                "content": content,
            }))
            .send()
            .await
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to update Polaris: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(rf_errors::RfError::Config(format!(
                "Polaris API returned error: {}", response.status()
            )));
        }
        
        // Update cache
        let mut cached = self.cached_config.write().await;
        cached.insert(key.to_string(), value.to_string());
        
        Ok(())
    }
}

impl ConfigCenterAdapter for PolarisAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        // Use blocking runtime for sync interface
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to create runtime: {}", e)))?;
        
        rt.block_on(async {
            // Check cache first
            {
                let cached = self.cached_config.read().await;
                if let Some(value) = cached.get(key) {
                    return Ok(Some(value.clone()));
                }
            }
            
            // Fetch from Polaris if not in cache
            let config = self.fetch_config().await?;
            let value = config.get(key).cloned();
            
            // Update cache
            if let Some(ref val) = value {
                let mut cached = self.cached_config.write().await;
                cached.insert(key.to_string(), val.clone());
            }
            
            Ok(value)
        })
    }
    
    fn set(&self, key: &str, value: &str) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to create runtime: {}", e)))?;
        
        rt.block_on(self.update_config(key, value))
    }
    
    fn all(&self) -> Result<HashMap<String, String>> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to create runtime: {}", e)))?;
        
        rt.block_on(async {
            let config = self.fetch_config().await?;
            
            // Update cache
            let mut cached = self.cached_config.write().await;
            *cached = config.clone();
            
            Ok(config)
        })
    }
    
    fn watch<F>(&self, callback: F) -> Result<()>
    where
        F: Fn(&str, &str) -> Result<()> + Send + Sync + 'static,
    {
        let adapter = Arc::new(self.clone());
        let callback = Arc::new(callback);
        
        tokio::spawn(async move {
            let mut last_config = HashMap::new();
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
            let mut consecutive_errors = 0;
            let max_errors = 5;
            
            loop {
                interval.tick().await;
                
                match adapter.fetch_config().await {
                    Ok(config) => {
                        consecutive_errors = 0; // Reset error counter on success
                        
                        // Check for changes
                        for (key, value) in &config {
                            if let Some(old_value) = last_config.get(key) {
                                if old_value != value {
                                    if let Err(e) = callback(key, value) {
                                        tracing::warn!("Polaris watch callback failed for key {}: {}", key, e);
                                    }
                                }
                            } else {
                                // New key
                                if let Err(e) = callback(key, value) {
                                    tracing::warn!("Polaris watch callback failed for new key {}: {}", key, e);
                                }
                            }
                        }
                        
                        // Check for deleted keys
                        let deleted_keys: Vec<String> = last_config.keys()
                            .filter(|k| !config.contains_key(*k))
                            .cloned()
                            .collect();
                        for key in deleted_keys {
                            if let Err(e) = callback(&key, "") {
                                tracing::warn!("Polaris watch callback failed for deleted key {}: {}", key, e);
                            }
                        }
                        
                        last_config = config;
                    }
                    Err(e) => {
                        consecutive_errors += 1;
                        tracing::warn!("Failed to fetch Polaris config (error {}/{}): {}", 
                            consecutive_errors, max_errors, e);
                        
                        // If too many consecutive errors, increase interval
                        if consecutive_errors >= max_errors {
                            tracing::error!("Too many consecutive errors, backing off...");
                            interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
                            consecutive_errors = 0; // Reset after backing off
                        }
                    }
                }
            }
        });
        
        Ok(())
    }
}

// Note: PolarisAdapter needs to be Clone for the watch implementation
// This is a simplified version - in production, you'd use Arc<Mutex<>> or similar
impl Clone for PolarisAdapter {
    fn clone(&self) -> Self {
        Self {
            client: Client::new(),
            server_url: self.server_url.clone(),
            namespace: self.namespace.clone(),
            file_group: self.file_group.clone(),
            file_name: self.file_name.clone(),
            cached_config: self.cached_config.clone(),
        }
    }
}

