//! # nacos
//!
//! nacos 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Nacos configuration center adapter

use super::ConfigCenterAdapter;
use rf_errors::{Result, RfError};
use std::collections::HashMap;
use std::sync::Arc;
use reqwest::Client;
use tokio::sync::RwLock;

// Note: NacosAdapter needs to be Clone for the watch implementation
impl Clone for NacosAdapter {
    fn clone(&self) -> Self {
        Self {
            client: Client::new(),
            base_url: self.base_url.clone(),
            namespace: self.namespace.clone(),
            group: self.group.clone(),
            data_id: self.data_id.clone(),
            content_md5: Arc::new(RwLock::new(None)),
        }
    }
}

/// Nacos configuration adapter
pub struct NacosAdapter {
    client: Client,
    base_url: String,
    namespace: String,
    group: String,
    data_id: String,
    content_md5: Arc<RwLock<Option<String>>>,
}

impl NacosAdapter {
    /// Create a new Nacos adapter
    pub fn new(base_url: &str, namespace: &str, group: &str, data_id: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            namespace: namespace.to_string(),
            group: group.to_string(),
            data_id: data_id.to_string(),
            content_md5: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Long poll for configuration changes
    async fn long_poll(&self, timeout: u64) -> Result<bool> {
        let content_md5 = {
            let md5 = self.content_md5.read().await;
            md5.clone()
        };
        
        let url = format!(
            "{}/nacos/v1/cs/configs/listener",
            self.base_url
        );
        
        let timeout_str = timeout.to_string();
        let mut params: Vec<(&str, &str)> = vec![
            ("dataId", self.data_id.as_str()),
            ("group", self.group.as_str()),
            ("tenant", self.namespace.as_str()),
            ("timeOut", &timeout_str),
        ];
        
        if let Some(ref md5) = content_md5 {
            params.push(("contentMD5", md5));
        }
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout + 10))
            .build()
            .map_err(|e| RfError::Config(format!("Failed to create HTTP client: {}", e)))?;
        
        let response = client
            .post(&url)
            .form(&params)
            .send()
            .await
            .map_err(|e| RfError::Config(format!("Nacos long poll failed: {}", e)))?;
        
        if response.status().is_success() {
            let content = response
                .text()
                .await
                .map_err(|e| RfError::Config(format!("Failed to read Nacos response: {}", e)))?;
            
            // If content is not empty, configuration changed
            if !content.is_empty() {
                // Update MD5 - use simple hash for now
                // In production, would use proper MD5 hashing
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                content.hash(&mut hasher);
                let md5_hash = format!("{:x}", hasher.finish());
                
                let mut md5_guard = self.content_md5.write().await;
                *md5_guard = Some(md5_hash);
                
                return Ok(true);
            }
        }
        
        Ok(false)
    }
}

impl ConfigCenterAdapter for NacosAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        // Nacos typically stores configs as properties or YAML
        // This is a simplified implementation
        let url = format!(
            "{}/nacos/v1/cs/configs?dataId={}&group={}&tenant={}",
            self.base_url, self.data_id, self.group, self.namespace
        );
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("Nacos request failed: {}", e)))?;
        
        if response.status().is_success() {
            let content = futures::executor::block_on(response.text())
                .map_err(|e| RfError::Config(format!("Failed to read Nacos response: {}", e)))?;
            
            // Parse properties format (simplified)
            for line in content.lines() {
                if let Some((k, v)) = line.split_once('=') {
                    if k.trim() == key {
                        return Ok(Some(v.trim().to_string()));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    fn set(&self, key: &str, value: &str) -> Result<()> {
        // Nacos set requires admin API
        // This is a simplified implementation
        let url = format!(
            "{}/nacos/v1/cs/configs?dataId={}&group={}&tenant={}",
            self.base_url, self.data_id, self.group, self.namespace
        );
        
        // In real implementation, would need to get existing content and update
        let content = format!("{}={}", key, value);
        
        let _response = futures::executor::block_on(
            self.client.post(&url).body(content).send()
        )
        .map_err(|e| RfError::Config(format!("Nacos set failed: {}", e)))?;
        
        Ok(())
    }
    
    fn all(&self) -> Result<HashMap<String, String>> {
        let url = format!(
            "{}/nacos/v1/cs/configs?dataId={}&group={}&tenant={}",
            self.base_url, self.data_id, self.group, self.namespace
        );
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("Nacos request failed: {}", e)))?;
        
        let mut result = HashMap::new();
        
        if response.status().is_success() {
            let content = futures::executor::block_on(response.text())
                .map_err(|e| RfError::Config(format!("Failed to read Nacos response: {}", e)))?;
            
            // Parse properties format
            for line in content.lines() {
                if let Some((k, v)) = line.split_once('=') {
                    result.insert(k.trim().to_string(), v.trim().to_string());
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
            
            loop {
                // Long polling with 30 second timeout
                match adapter.long_poll(30).await {
                    Ok(changed) => {
                        if changed {
                            // Fetch updated configuration
                            match adapter.all() {
                                Ok(config) => {
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
                                    tracing::warn!("Failed to fetch Nacos config after change: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Nacos long polling failed: {}", e);
                        // Wait before retrying
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        });
        
        Ok(())
    }
}

