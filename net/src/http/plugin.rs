//! # plugin
//!
//! plugin 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP server plugin system

use axum::Router;
use rf_errors::Result;
use std::sync::Arc;
use std::collections::HashMap;

/// Plugin lifecycle hooks
pub enum PluginHook {
    BeforeStart,
    AfterStart,
    BeforeStop,
    AfterStop,
    OnRequest,
    OnResponse,
}

/// Plugin trait
pub trait Plugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Initialize the plugin
    fn init(&mut self) -> Result<()>;
    
    /// Configure the plugin
    fn configure(&mut self, config: HashMap<String, String>) -> Result<()>;
    
    /// Apply plugin to router
    fn apply(&self, router: Router) -> Router;
    
    /// Handle lifecycle hook
    fn handle_hook(&self, hook: PluginHook) -> Result<()> {
        let _ = hook;
        Ok(())
    }
    
    /// Cleanup the plugin
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Plugin manager
pub struct PluginManager {
    plugins: HashMap<String, Arc<dyn Plugin>>,
    configs: HashMap<String, HashMap<String, String>>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            configs: HashMap::new(),
        }
    }

    /// Register a plugin
    pub fn register(&mut self, mut plugin: Box<dyn Plugin>) -> Result<()> {
        let name = plugin.name().to_string();
        plugin.init()?;
        
        // Apply configuration if available
        if let Some(config) = self.configs.get(&name) {
            plugin.configure(config.clone())?;
        }
        
        self.plugins.insert(name, Arc::new(plugin));
        Ok(())
    }

    /// Configure a plugin
    pub fn configure(&mut self, name: &str, config: HashMap<String, String>) -> Result<()> {
        self.configs.insert(name.to_string(), config.clone());
        
        // If plugin is already registered, we can't modify it
        // Configuration will be applied when plugin is registered
        // For now, just store the config
        Ok(())
    }

    /// Get a plugin
    pub fn get(&self, name: &str) -> Option<&Arc<dyn Plugin>> {
        self.plugins.get(name)
    }

    /// Apply all plugins to a router
    pub fn apply_all(&self, mut router: Router) -> Router {
        for plugin in self.plugins.values() {
            router = plugin.apply(router);
        }
        router
    }

    /// Call a lifecycle hook on all plugins
    pub fn call_hook(&self, hook: PluginHook) -> Result<()> {
        for plugin in self.plugins.values() {
            plugin.handle_hook(hook.clone())?;
        }
        Ok(())
    }

    /// Unregister a plugin
    pub fn unregister(&mut self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.remove(name) {
            // Note: We can't call cleanup on Arc<dyn Plugin> since Plugin::cleanup requires &mut self
            // This is a limitation - plugins should handle cleanup in their drop implementation
            let _ = plugin;
        }
        self.configs.remove(name);
        Ok(())
    }

    /// List all registered plugins
    pub fn list(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for PluginHook {
    fn clone(&self) -> Self {
        match self {
            PluginHook::BeforeStart => PluginHook::BeforeStart,
            PluginHook::AfterStart => PluginHook::AfterStart,
            PluginHook::BeforeStop => PluginHook::BeforeStop,
            PluginHook::AfterStop => PluginHook::AfterStop,
            PluginHook::OnRequest => PluginHook::OnRequest,
            PluginHook::OnResponse => PluginHook::OnResponse,
        }
    }
}

