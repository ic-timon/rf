//! # apollo
//!
//! apollo 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Apollo configuration center adapter

use super::ConfigCenterAdapter;
use rf_errors::{Result, RfError};
use std::collections::HashMap;
use std::sync::Arc;
use reqwest::Client;
use tokio::sync::RwLock;

// Note: ApolloAdapter needs to be Clone for the watch implementation
impl Clone for ApolloAdapter {
    fn clone(&self) -> Self {
        Self {
            client: Client::new(),
            base_url: self.base_url.clone(),
            app_id: self.app_id.clone(),
            cluster: self.cluster.clone(),
            namespace: self.namespace.clone(),
            notification_id: Arc::new(RwLock::new(None)),
        }
    }
}

/// Apollo configuration adapter
pub struct ApolloAdapter {
    client: Client,
    base_url: String,
    app_id: String,
    cluster: String,
    namespace: String,
    notification_id: Arc<RwLock<Option<i64>>>,
}

impl ApolloAdapter {
    /// Create a new Apollo adapter
    pub fn new(base_url: &str, app_id: &str, cluster: &str, namespace: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            app_id: app_id.to_string(),
            cluster: cluster.to_string(),
            namespace: namespace.to_string(),
            notification_id: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Fetch notifications using long polling
    async fn fetch_notifications(&self, timeout: u64) -> Result<Vec<String>> {
        let notification_id = {
            let id = self.notification_id.read().await;
            id.unwrap_or(0)
        };
        
        let url = format!(
            "{}/notifications/v2?appId={}&cluster={}&namespaceName={}&notificationId={}",
            self.base_url, self.app_id, self.cluster, self.namespace, notification_id
        );
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout + 5))
            .build()
            .map_err(|e| RfError::Config(format!("Failed to create HTTP client: {}", e)))?;
        
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| RfError::Config(format!("Apollo notification request failed: {}", e)))?;
        
        if response.status() == 304 {
            // No changes
            return Ok(Vec::new());
        }
        
        if response.status().is_success() {
            let notifications: Vec<serde_json::Value> = response
                .json()
                .await
                .map_err(|e| RfError::Config(format!("Failed to parse Apollo notifications: {}", e)))?;
            
            let mut changed_namespaces = Vec::new();
            let mut max_notification_id = notification_id;
            
            for notification in notifications {
                if let Some(ns) = notification.get("namespaceName").and_then(|n| n.as_str()) {
                    if ns == self.namespace {
                        changed_namespaces.push(ns.to_string());
                    }
                }
                if let Some(id) = notification.get("notificationId").and_then(|n| n.as_i64()) {
                    if id > max_notification_id {
                        max_notification_id = id;
                    }
                }
            }
            
            // Update notification ID
            {
                let mut id = self.notification_id.write().await;
                *id = Some(max_notification_id);
            }
            
            return Ok(changed_namespaces);
        }
        
        Ok(Vec::new())
    }
}

impl ConfigCenterAdapter for ApolloAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        // Simplified Apollo API call
        let url = format!(
            "{}/configs/{}/{}/{}?key={}",
            self.base_url, self.app_id, self.cluster, self.namespace, key
        );
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("Apollo request failed: {}", e)))?;
        
        if response.status().is_success() {
            let json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Config(format!("Failed to parse Apollo response: {}", e)))?;
            
            if let Some(value) = json.get("configurations").and_then(|c| c.get(key)) {
                return Ok(value.as_str().map(|s| s.to_string()));
            }
        }
        
        Ok(None)
    }
    
    fn set(&self, key: &str, value: &str) -> Result<()> {
        // Apollo typically requires admin API for setting values
        // This is a simplified implementation
        let url = format!(
            "{}/configs/{}/{}/{}",
            self.base_url, self.app_id, self.cluster, self.namespace
        );
        
        let mut map = HashMap::new();
        map.insert(key.to_string(), value.to_string());
        
        let _response = futures::executor::block_on(
            self.client.put(&url).json(&map).send()
        )
        .map_err(|e| RfError::Config(format!("Apollo set failed: {}", e)))?;
        
        Ok(())
    }
    
    fn all(&self) -> Result<HashMap<String, String>> {
        let url = format!(
            "{}/configs/{}/{}/{}",
            self.base_url, self.app_id, self.cluster, self.namespace
        );
        
        let response = futures::executor::block_on(
            self.client.get(&url).send()
        )
        .map_err(|e| RfError::Config(format!("Apollo request failed: {}", e)))?;
        
        if response.status().is_success() {
            let json: serde_json::Value = futures::executor::block_on(response.json())
                .map_err(|e| RfError::Config(format!("Failed to parse Apollo response: {}", e)))?;
            
            if let Some(configs) = json.get("configurations").and_then(|c| c.as_object()) {
                let mut result = HashMap::new();
                for (k, v) in configs {
                    if let Some(s) = v.as_str() {
                        result.insert(k.clone(), s.to_string());
                    }
                }
                return Ok(result);
            }
        }
        
        Ok(HashMap::new())
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
                // Long polling with 60 second timeout
                match adapter.fetch_notifications(60).await {
                    Ok(changed_namespaces) => {
                        if !changed_namespaces.is_empty() {
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
                                    tracing::warn!("Failed to fetch Apollo config after notification: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Apollo notification polling failed: {}", e);
                        // Wait before retrying
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        });
        
        Ok(())
    }
}

