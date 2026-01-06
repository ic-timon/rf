//! # validation
//!
//! validation 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Configuration validation

use rf_errors::{Result, RfError};
use std::collections::HashMap;

/// Configuration validation rule
pub trait ValidationRule: Send + Sync {
    /// Validate a configuration value
    fn validate(&self, key: &str, value: &str) -> Result<()>;
}

/// Required validation rule
pub struct RequiredRule;

impl ValidationRule for RequiredRule {
    fn validate(&self, key: &str, value: &str) -> Result<()> {
        if value.is_empty() {
            return Err(RfError::Config(format!("Configuration key '{}' is required", key)));
        }
        Ok(())
    }
}

/// Type validation rule
pub enum TypeRule {
    String,
    Integer,
    Float,
    Boolean,
    Url,
    Email,
}

impl ValidationRule for TypeRule {
    fn validate(&self, key: &str, value: &str) -> Result<()> {
        match self {
            TypeRule::String => Ok(()),
            TypeRule::Integer => {
                value.parse::<i64>()
                    .map_err(|_| RfError::Config(format!("Configuration key '{}' must be an integer", key)))?;
                Ok(())
            }
            TypeRule::Float => {
                value.parse::<f64>()
                    .map_err(|_| RfError::Config(format!("Configuration key '{}' must be a float", key)))?;
                Ok(())
            }
            TypeRule::Boolean => {
                value.parse::<bool>()
                    .map_err(|_| RfError::Config(format!("Configuration key '{}' must be a boolean", key)))?;
                Ok(())
            }
            TypeRule::Url => {
                url::Url::parse(value)
                    .map_err(|_| RfError::Config(format!("Configuration key '{}' must be a valid URL", key)))?;
                Ok(())
            }
            TypeRule::Email => {
                if !value.contains('@') || !value.contains('.') {
                    return Err(RfError::Config(format!("Configuration key '{}' must be a valid email", key)));
                }
                Ok(())
            }
        }
    }
}

/// Range validation rule
pub struct RangeRule {
    min: Option<i64>,
    max: Option<i64>,
}

impl RangeRule {
    pub fn new(min: Option<i64>, max: Option<i64>) -> Self {
        Self { min, max }
    }
}

impl ValidationRule for RangeRule {
    fn validate(&self, key: &str, value: &str) -> Result<()> {
        let num = value.parse::<i64>()
            .map_err(|_| RfError::Config(format!("Configuration key '{}' must be a number for range validation", key)))?;
        
        if let Some(min) = self.min {
            if num < min {
                return Err(RfError::Config(format!("Configuration key '{}' must be >= {}", key, min)));
            }
        }
        
        if let Some(max) = self.max {
            if num > max {
                return Err(RfError::Config(format!("Configuration key '{}' must be <= {}", key, max)));
            }
        }
        
        Ok(())
    }
}

/// Length validation rule
pub struct LengthRule {
    min: Option<usize>,
    max: Option<usize>,
}

impl LengthRule {
    pub fn new(min: Option<usize>, max: Option<usize>) -> Self {
        Self { min, max }
    }
}

impl ValidationRule for LengthRule {
    fn validate(&self, key: &str, value: &str) -> Result<()> {
        let len = value.len();
        
        if let Some(min) = self.min {
            if len < min {
                return Err(RfError::Config(format!("Configuration key '{}' length must be >= {}", key, min)));
            }
        }
        
        if let Some(max) = self.max {
            if len > max {
                return Err(RfError::Config(format!("Configuration key '{}' length must be <= {}", key, max)));
            }
        }
        
        Ok(())
    }
}

/// Configuration validator
pub struct ConfigValidator {
    rules: HashMap<String, Vec<Box<dyn ValidationRule>>>,
}

impl ConfigValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Add a validation rule for a key
    pub fn rule(mut self, key: &str, rule: Box<dyn ValidationRule>) -> Self {
        self.rules.entry(key.to_string())
            .or_default()
            .push(rule);
        self
    }

    /// Validate a configuration value
    pub fn validate(&self, key: &str, value: &str) -> Result<()> {
        if let Some(rules) = self.rules.get(key) {
            for rule in rules {
                rule.validate(key, value)?;
            }
        }
        Ok(())
    }

    /// Validate all configuration
    pub fn validate_all(&self, config: &HashMap<String, String>) -> Result<()> {
        for (key, value) in config {
            self.validate(key, value)?;
        }
        Ok(())
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

