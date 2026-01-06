//! # length
//!
//! length 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Length validation rules

use rf_errors::Result;

/// Validate exact length
pub fn validate_length(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("length needs length value".to_string()));
    }
    let len: usize = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid length value".to_string()))?;
    if value.len() != len {
        Err(rf_errors::RfError::Validation(format!("Length must be {}", len)))
    } else {
        Ok(())
    }
}

/// Validate minimum length
pub fn validate_min_length(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("min_length needs min length value".to_string()));
    }
    let min: usize = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid min length value".to_string()))?;
    if value.len() < min {
        Err(rf_errors::RfError::Validation(format!("Length must be at least {}", min)))
    } else {
        Ok(())
    }
}

/// Validate maximum length
pub fn validate_max_length(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("max_length needs max length value".to_string()));
    }
    let max: usize = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid max length value".to_string()))?;
    if value.len() > max {
        Err(rf_errors::RfError::Validation(format!("Length must be at most {}", max)))
    } else {
        Ok(())
    }
}

/// Validate size (for numbers)
pub fn validate_size(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("size needs size value".to_string()));
    }
    let val: f64 = value.parse().map_err(|_| rf_errors::RfError::Validation("Invalid number".to_string()))?;
    let size: f64 = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid size value".to_string()))?;
    if val != size {
        Err(rf_errors::RfError::Validation(format!("Size must be {}", size)))
    } else {
        Ok(())
    }
}

