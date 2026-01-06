//! # consul
//!
//! consul 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Consul configuration center adapter

use super::ConfigCenterAdapter;
use rf_errors::{Result, RfError};
use std::collections::HashMap;
use std::sync::Arc;
use reqwest::Client;
use tokio::sync::RwLock;

// Note: ConsulAdapter needs to be Clone for the watch implementation
impl Clone for ConsulAdapter {
    fn clone(&self) -> Self {
        Self {
            client: Client::new(),
            base_url: self.base_url.clone(),
            prefix: self.prefix.clone(),
            wait_index: Arc::new(RwLock::new(0)),
        }
    }
}

/// Consul configuration adapter
pub struct ConsulAdapter {
    client: Client,
    base_url: String,
    prefix: String,
    wait_index: Arc<RwLock<u64>>,
}

impl ConsulAdapter {
    /// Create a new Consul adapter
    pub fn new(base_url: &str, prefix: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            prefix: prefix.to_string(),
            wait_index: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Perform blocking query to watch for changes
    async fn blocking_query(&self, wait_index: u64, wait_time: u64) -> Result<(u64, HashMap<String, String>)> {
        let url = format!(
            "{}/v1/kv/{}/?recurse&wait={}s&index={}",
            self.base_url, self.prefix, wait_time, wait_index
        );
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(wait_time + 10))
            .build()
            .map_err(|e| RfError::Config(format!("Failed to create HTTP client: {}", e)))?;
        
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| RfError::Config(format!("Consul blocking query failed: {}", e)))?;
        
        let new_index = response
            .headers()
            .get("X-Consul-Index")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(wait_index);
        
        let mut result = HashMap::new();
        
        if response.status().is_success() {
            let json: Vec<serde_json::Value> = response
                .json()
                .await
                .map_err(|e| RfError::Config(format!("Failed to parse Consul response: {}", e)))?;
            
            use base64::{Engine as _, engine::general_purpose};
            for item in json {
                if let (Some(key_path), Some(value)) = (item.get("Key"), item.get("Value")) {
                    if let (Some(k), Some(v)) = (key_path.as_str(), value.as_str()) {
                        if let Ok(decoded_bytes) = general_purpose::STANDARD.decode(v) {
                            if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                                // Remove prefix from key
                                let key = k.strip_prefix(&format!("{}/", self.prefix))
                                    .unwrap_or(k)
                                    .to_string();
                                result.insert(key, decoded_str);
                            }
                        }
                    }
                }
            }
        }
        
        Ok((new_index, result))
    }
}

impl ConfigCenterAdapter for ConsulAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        let url = format!("{}/v1/kv/{}/{}", self.base_url, self.prefix, key);
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("Consul request failed: {}", e)))?;
        
        if response.status().is_success() {
            let json: Vec<serde_json::Value> = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Config(format!("Failed to parse Consul response: {}", e)))?;
            
            if let Some(item) = json.first() {
                if let Some(value) = item.get("Value") {
                    if let Some(decoded) = value.as_str() {
                        // Consul values are base64 encoded
                        use base64::{Engine as _, engine::general_purpose};
                        if let Ok(decoded_bytes) = general_purpose::STANDARD.decode(decoded) {
                            if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                                return Ok(Some(decoded_str));
                            }
                        }
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    fn set(&self, key: &str, value: &str) -> Result<()> {
        let url = format!("{}/v1/kv/{}/{}", self.base_url, self.prefix, key);
        
        let _response = futures::executor::block_on(
            self.client.put(&url).body(value.to_string()).send()
        )
        .map_err(|e| RfError::Config(format!("Consul set failed: {}", e)))?;
        
        Ok(())
    }
    
    fn all(&self) -> Result<HashMap<String, String>> {
        let url = format!("{}/v1/kv/{}/?recurse", self.base_url, self.prefix);
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("Consul request failed: {}", e)))?;
        
        let mut result = HashMap::new();
        
        if response.status().is_success() {
            let json: Vec<serde_json::Value> = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Config(format!("Failed to parse Consul response: {}", e)))?;
            
            use base64::{Engine as _, engine::general_purpose};
            for item in json {
                if let (Some(key_path), Some(value)) = (item.get("Key"), item.get("Value")) {
                    if let (Some(k), Some(v)) = (key_path.as_str(), value.as_str()) {
                        if let Ok(decoded_bytes) = general_purpose::STANDARD.decode(v) {
                            if let Ok(decoded_str) = String::from_utf8(decoded_bytes) {
                                // Remove prefix from key
                                let key = k.strip_prefix(&format!("{}/", self.prefix))
                                    .unwrap_or(k)
                                    .to_string();
                                result.insert(key, decoded_str);
                            }
                        }
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
            let mut wait_index = {
                let index = adapter.wait_index.read().await;
                *index
            };
            
            loop {
                match adapter.blocking_query(wait_index, 60).await {
                    Ok((new_index, config)) => {
                        if new_index != wait_index {
                            // Configuration changed
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
                            wait_index = new_index;
                            
                            // Update wait_index
                            {
                                let mut index = adapter.wait_index.write().await;
                                *index = new_index;
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Consul blocking query failed: {}", e);
                        // Wait before retrying
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        });
        
        Ok(())
    }
}

