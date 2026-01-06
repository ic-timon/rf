//! # conditional
//!
//! conditional 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Conditional validation rules

use rf_errors::Result;

/// Validate required_if_all (required if all specified fields have values)
pub fn validate_required_if_all(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("required_if_all needs field names".to_string()));
    }
    // Simplified - would check all fields have values
    if value.is_empty() {
        Err(rf_errors::RfError::Validation("Field is required".to_string()))
    } else {
        Ok(())
    }
}

/// Validate required_unless (required unless another field has value)
pub fn validate_required_unless(value: &str, params: &[String]) -> Result<()> {
    if params.len() < 2 {
        return Err(rf_errors::RfError::Validation("required_unless needs field and value".to_string()));
    }
    // Simplified - would check other field value
    if value.is_empty() {
        Err(rf_errors::RfError::Validation("Field is required".to_string()))
    } else {
        Ok(())
    }
}

/// Validate required_with_all (required if all specified fields exist)
pub fn validate_required_with_all(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("required_with_all needs field names".to_string()));
    }
    // Simplified - would check all fields exist
    if value.is_empty() {
        Err(rf_errors::RfError::Validation("Field is required".to_string()))
    } else {
        Ok(())
    }
}

/// Validate required_without_all (required if none of specified fields exist)
pub fn validate_required_without_all(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("required_without_all needs field names".to_string()));
    }
    // Simplified - would check none of fields exist
    if value.is_empty() {
        Err(rf_errors::RfError::Validation("Field is required".to_string()))
    } else {
        Ok(())
    }
}

