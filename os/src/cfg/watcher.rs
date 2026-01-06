//! # watcher
//!
//! watcher 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Configuration file watcher

use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use rf_errors::Result;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use std::collections::HashMap;

/// Configuration change event
#[derive(Debug, Clone)]
pub struct ConfigChangeEvent {
    pub path: String,
    pub event_type: ConfigChangeType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Configuration change type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigChangeType {
    Modified,
    Created,
    Deleted,
    Renamed,
}

/// Configuration change callback
pub type ConfigChangeCallback = Arc<dyn Fn(ConfigChangeEvent) -> Result<()> + Send + Sync>;

/// Configuration watcher with multiple callbacks
pub struct ConfigWatcher {
    watcher: RecommendedWatcher,
    callbacks: Vec<ConfigChangeCallback>,
    path_callbacks: HashMap<String, Vec<ConfigChangeCallback>>, // Path-specific callbacks
}

impl ConfigWatcher {
    /// Create a new config watcher
    pub fn new() -> Result<Self> {
        let callbacks: Vec<ConfigChangeCallback> = Vec::new();
        let path_callbacks: HashMap<String, Vec<ConfigChangeCallback>> = HashMap::new();
        
        let (tx, mut rx) = mpsc::channel::<ConfigChangeEvent>(100);
        let callbacks_clone = Arc::new(tokio::sync::RwLock::new(callbacks));
        let path_callbacks_clone = Arc::new(tokio::sync::RwLock::new(path_callbacks));
        
        let watcher = notify::recommended_watcher(move |event: notify::Result<Event>| {
            if let Ok(event) = event {
                let change_type = match event.kind {
                    EventKind::Create(_) => ConfigChangeType::Created,
                    EventKind::Modify(_) => ConfigChangeType::Modified,
                    EventKind::Remove(_) => ConfigChangeType::Deleted,
                    EventKind::Any | EventKind::Access(_) | EventKind::Other => return, // Skip other events
                };
                
                for path in event.paths {
                    if let Some(path_str) = path.to_str() {
                        let change_event = ConfigChangeEvent {
                            path: path_str.to_string(),
                            event_type: change_type,
                            timestamp: chrono::Utc::now(),
                        };
                        let _ = tx.try_send(change_event);
                    }
                }
            }
        })
        .map_err(|e| rf_errors::RfError::Config(format!("Failed to create watcher: {}", e)))?;
        
        // Spawn async task to handle events
        let callbacks_task = callbacks_clone.clone();
        let path_callbacks_task = path_callbacks_clone.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                // Call global callbacks
                let callbacks = callbacks_task.read().await;
                for callback in callbacks.iter() {
                    let _ = callback(event.clone());
                }
                
                // Call path-specific callbacks
                let path_callbacks = path_callbacks_task.read().await;
                if let Some(callbacks) = path_callbacks.get(&event.path) {
                    for callback in callbacks.iter() {
                        let _ = callback(event.clone());
                    }
                }
            }
        });
        
        Ok(Self {
            watcher,
            callbacks: Vec::new(),
            path_callbacks: HashMap::new(),
        })
    }

    /// Register a global change callback
    pub fn register_callback<F>(&mut self, callback: F)
    where
        F: Fn(ConfigChangeEvent) -> Result<()> + Send + Sync + 'static,
    {
        self.callbacks.push(Arc::new(callback));
    }

    /// Register a path-specific change callback
    pub fn register_path_callback<F>(&mut self, path: String, callback: F)
    where
        F: Fn(ConfigChangeEvent) -> Result<()> + Send + Sync + 'static,
    {
        self.path_callbacks.entry(path)
            .or_default()
            .push(Arc::new(callback));
    }

    /// Get callback count
    pub fn callback_count(&self) -> usize {
        self.callbacks.len()
    }

    /// Get path-specific callback count
    pub fn path_callback_count(&self, path: &str) -> usize {
        self.path_callbacks.get(path).map(|v| v.len()).unwrap_or(0)
    }
    
    /// Watch a configuration file
    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to watch file: {}", e)))?;
        Ok(())
    }
    
    /// Unwatch a configuration file
    pub fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.watcher.unwatch(path.as_ref())
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to unwatch file: {}", e)))?;
        Ok(())
    }

    /// Clear all callbacks
    pub fn clear_callbacks(&mut self) {
        self.callbacks.clear();
        self.path_callbacks.clear();
    }

    /// Remove callbacks for a specific path
    pub fn remove_path_callbacks(&mut self, path: &str) {
        self.path_callbacks.remove(path);
    }
}

impl Default for ConfigWatcher {
    fn default() -> Self {
        Self::new().expect("Failed to create default config watcher")
    }
}
