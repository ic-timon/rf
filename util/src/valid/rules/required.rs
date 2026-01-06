//! # required
//!
//! required 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Required validation rules

use rf_errors::Result;

/// Validate required field
pub fn validate_required(value: &str, _params: &[String]) -> Result<()> {
    if value.is_empty() {
        Err(rf_errors::RfError::Validation("Field is required".to_string()))
    } else {
        Ok(())
    }
}

/// Validate required_if (required if another field has value)
pub fn validate_required_if(value: &str, params: &[String]) -> Result<()> {
    if params.len() < 2 {
        return Err(rf_errors::RfError::Validation("required_if needs field and value".to_string()));
    }
    // Simplified - would check other field value
    if value.is_empty() {
        Err(rf_errors::RfError::Validation("Field is required".to_string()))
    } else {
        Ok(())
    }
}

/// Validate required_with (required if another field exists)
pub fn validate_required_with(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("required_with needs field name".to_string()));
    }
    // Simplified - would check other field exists
    if value.is_empty() {
        Err(rf_errors::RfError::Validation("Field is required".to_string()))
    } else {
        Ok(())
    }
}

/// Validate required_without (required if another field doesn't exist)
pub fn validate_required_without(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("required_without needs field name".to_string()));
    }
    // Simplified - would check other field doesn't exist
    if value.is_empty() {
        Err(rf_errors::RfError::Validation("Field is required".to_string()))
    } else {
        Ok(())
    }
}

