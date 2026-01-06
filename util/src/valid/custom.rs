//! # custom
//!
//! custom 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Custom validation rules

use rf_errors::Result;
use std::collections::HashMap;
use std::sync::Arc;

/// Custom validation rule function type
pub type CustomRuleFn = Arc<dyn Fn(&str, &[String]) -> Result<()> + Send + Sync>;

/// Custom rule registry
pub struct CustomRuleRegistry {
    rules: HashMap<String, CustomRuleFn>,
}

impl CustomRuleRegistry {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Register a custom rule
    pub fn register<F>(&mut self, name: &str, rule: F)
    where
        F: Fn(&str, &[String]) -> Result<()> + Send + Sync + 'static,
    {
        self.rules.insert(name.to_string(), Arc::new(rule));
    }

    /// Get a custom rule
    pub fn get(&self, name: &str) -> Option<&CustomRuleFn> {
        self.rules.get(name)
    }
}

impl Default for CustomRuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global custom rule registry
static CUSTOM_RULES: once_cell::sync::Lazy<std::sync::Mutex<CustomRuleRegistry>> =
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(CustomRuleRegistry::new()));

/// Register a custom validation rule globally
pub fn register_custom_rule<F>(name: &str, rule: F)
where
    F: Fn(&str, &[String]) -> Result<()> + Send + Sync + 'static,
{
    let mut registry = CUSTOM_RULES.lock().unwrap();
    registry.register(name, rule);
}

/// Get a custom rule
pub fn get_custom_rule(name: &str) -> Option<CustomRuleFn> {
    let registry = CUSTOM_RULES.lock().unwrap();
    registry.get(name).cloned()
}

