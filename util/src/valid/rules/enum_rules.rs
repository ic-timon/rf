//! # enum_rules
//!
//! enum_rules 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Enum validation rules

use rf_errors::Result;

/// Validate in (value must be in list)
pub fn validate_in(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("in needs list of values".to_string()));
    }
    if !params.contains(&value.to_string()) {
        Err(rf_errors::RfError::Validation(format!("Value must be one of: {}", params.join(", "))))
    } else {
        Ok(())
    }
}

/// Validate not_in (value must not be in list)
pub fn validate_not_in(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("not_in needs list of values".to_string()));
    }
    if params.contains(&value.to_string()) {
        Err(rf_errors::RfError::Validation(format!("Value must not be one of: {}", params.join(", "))))
    } else {
        Ok(())
    }
}

