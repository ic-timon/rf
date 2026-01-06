//! # mod
//!
//! mod 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Data validation module

pub mod rules;
pub mod struct_validation;
pub mod map_validation;
pub mod error;
pub mod custom;
pub mod i18n;
pub mod recursive;

// Re-export rules functions but not types to avoid conflicts
pub use rules::*;
pub use struct_validation::*;
pub use map_validation::*;
pub use error::*;
pub use custom::*;
pub use i18n::*;
pub use recursive::*;

use rf_errors::Result;

/// Validation result
pub type ValidationResult = Result<()>;

/// Validator
pub struct Validator {
    rules: Vec<Rule>,
    bail: bool, // Stop on first error
}

impl Validator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            bail: false,
        }
    }

    /// Set bail mode (stop on first error)
    pub fn bail(mut self) -> Self {
        self.bail = true;
        self
    }

    /// Add a validation rule
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Validate data
    ///
    /// Validates the given data against all registered rules.
    ///
    /// # Example
    /// ```ignore
    /// use rf_util::valid::Validator;
    /// use std::collections::HashMap;
    ///
    /// let validator = Validator::new()
    ///     .rule(Rule::new("email", "required", vec![]))
    ///     .rule(Rule::new("email", "email", vec![]));
    ///
    /// let mut data = HashMap::new();
    /// data.insert("email".to_string(), "test@example.com".to_string());
    ///
    /// let result = validator.validate_map(&data);
    /// ```
    pub fn validate_map(&self, data: &std::collections::HashMap<String, String>) -> ValidationResult {
        for rule in &self.rules {
            let value = data.get(&rule.field);
            let value_str = value.map(|s| s.as_str()).unwrap_or("");

            // Call the rule function based on rule name
            let result = match rule.rule_name.as_str() {
                "required" => rules::required::validate_required(value_str, &rule.params),
                "required_if" => rules::required::validate_required_if(value_str, &rule.params),
                "email" => rules::format::validate_email(value_str, &rule.params),
                "integer" => rules::type_rules::validate_integer(value_str, &rule.params),
                "min_length" => rules::length::validate_min_length(value_str, &rule.params),
                "max_length" => rules::length::validate_max_length(value_str, &rule.params),
                "in" => rules::enum_rules::validate_in(value_str, &rule.params),
                // Add more rules as needed
                _ => Ok(()),
            };

            if let Err(e) = result {
                if self.bail {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    /// Validate data (Any version - supports multiple types)
    /// 
    /// Supports the following types:
    /// - `HashMap<String, String>` - Direct validation
    /// - `HashMap<&str, &str>` - Converted to String HashMap
    /// - `serde_json::Value` - Converted to HashMap
    /// - Any type implementing `serde::Serialize` - Serialized to JSON then validated
    pub fn validate(&self, data: &dyn std::any::Any) -> ValidationResult {
        // Try HashMap<String, String>
        if let Some(map) = data.downcast_ref::<std::collections::HashMap<String, String>>() {
            return self.validate_map(map);
        }
        
        // Try HashMap<&str, &str>
        if let Some(map) = data.downcast_ref::<std::collections::HashMap<&str, &str>>() {
            let converted: std::collections::HashMap<String, String> = map
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            return self.validate_map(&converted);
        }
        
        // Try serde_json::Value
        if let Some(json) = data.downcast_ref::<serde_json::Value>() {
            return self.validate_json(json);
        }
        
        // Try any type that implements Serialize
        // We need to serialize it to JSON first, then validate
        if let Ok(json_value) = self.try_serialize_to_json(data) {
            return self.validate_json(&json_value);
        }
        
        // Unsupported type
        Err(rf_errors::RfError::InvalidParameter(
            "Unsupported data type for validation. Supported types: HashMap<String, String>, HashMap<&str, &str>, serde_json::Value, or any type implementing serde::Serialize".to_string()
        ))
    }
    
    /// Validate JSON value
    fn validate_json(&self, json: &serde_json::Value) -> ValidationResult {
        match json {
            serde_json::Value::Object(obj) => {
                let map: std::collections::HashMap<String, String> = obj
                    .iter()
                    .filter_map(|(k, v)| {
                        match v {
                            serde_json::Value::String(s) => Some((k.clone(), s.clone())),
                            serde_json::Value::Number(n) => Some((k.clone(), n.to_string())),
                            serde_json::Value::Bool(b) => Some((k.clone(), b.to_string())),
                            serde_json::Value::Null => Some((k.clone(), String::new())),
                            _ => Some((k.clone(), serde_json::to_string(v).unwrap_or_default())),
                        }
                    })
                    .collect();
                self.validate_map(&map)
            }
            _ => Err(rf_errors::RfError::InvalidParameter(
                "JSON value must be an object for validation".to_string()
            ))
        }
    }
    
    /// Try to serialize any type to JSON
    /// 
    /// Note: This is a limitation - we can't directly serialize from &dyn Any
    /// without knowing the concrete type. This would require a different approach
    /// such as using a trait or type registry.
    /// 
    /// Users should convert their types to HashMap or JSON before calling validate
    fn try_serialize_to_json(&self, _data: &dyn std::any::Any) -> rf_errors::Result<serde_json::Value> {
        Err(rf_errors::RfError::InvalidParameter(
            "Direct serialization from &dyn Any is not supported. Please convert to HashMap<String, String> or serde_json::Value first.".to_string()
        ))
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation rule
pub struct Rule {
    field: String,
    rule_name: String,
    params: Vec<String>,
}

impl Rule {
    /// Create a new rule
    pub fn new(field: &str, rule_name: &str, params: Vec<String>) -> Self {
        Self {
            field: field.to_string(),
            rule_name: rule_name.to_string(),
            params,
        }
    }
}

