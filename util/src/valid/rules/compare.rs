//! # compare
//!
//! compare 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Comparison validation rules

use rf_errors::Result;

/// Validate equal
pub fn validate_eq(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("eq needs comparison value".to_string()));
    }
    if value != params[0] {
        Err(rf_errors::RfError::Validation(format!("Value must equal {}", params[0])))
    } else {
        Ok(())
    }
}

/// Validate not equal
pub fn validate_ne(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("ne needs comparison value".to_string()));
    }
    if value == params[0] {
        Err(rf_errors::RfError::Validation(format!("Value must not equal {}", params[0])))
    } else {
        Ok(())
    }
}

/// Validate greater than
pub fn validate_gt(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("gt needs comparison value".to_string()));
    }
    let val: f64 = value.parse().map_err(|_| rf_errors::RfError::Validation("Invalid number".to_string()))?;
    let cmp: f64 = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid comparison value".to_string()))?;
    if val <= cmp {
        Err(rf_errors::RfError::Validation(format!("Value must be greater than {}", cmp)))
    } else {
        Ok(())
    }
}

/// Validate greater than or equal
pub fn validate_gte(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("gte needs comparison value".to_string()));
    }
    let val: f64 = value.parse().map_err(|_| rf_errors::RfError::Validation("Invalid number".to_string()))?;
    let cmp: f64 = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid comparison value".to_string()))?;
    if val < cmp {
        Err(rf_errors::RfError::Validation(format!("Value must be greater than or equal to {}", cmp)))
    } else {
        Ok(())
    }
}

/// Validate less than
pub fn validate_lt(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("lt needs comparison value".to_string()));
    }
    let val: f64 = value.parse().map_err(|_| rf_errors::RfError::Validation("Invalid number".to_string()))?;
    let cmp: f64 = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid comparison value".to_string()))?;
    if val >= cmp {
        Err(rf_errors::RfError::Validation(format!("Value must be less than {}", cmp)))
    } else {
        Ok(())
    }
}

/// Validate less than or equal
pub fn validate_lte(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("lte needs comparison value".to_string()));
    }
    let val: f64 = value.parse().map_err(|_| rf_errors::RfError::Validation("Invalid number".to_string()))?;
    let cmp: f64 = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid comparison value".to_string()))?;
    if val > cmp {
        Err(rf_errors::RfError::Validation(format!("Value must be less than or equal to {}", cmp)))
    } else {
        Ok(())
    }
}

/// Validate between
pub fn validate_between(value: &str, params: &[String]) -> Result<()> {
    if params.len() < 2 {
        return Err(rf_errors::RfError::Validation("between needs min and max values".to_string()));
    }
    let val: f64 = value.parse().map_err(|_| rf_errors::RfError::Validation("Invalid number".to_string()))?;
    let min: f64 = params[0].parse().map_err(|_| rf_errors::RfError::Validation("Invalid min value".to_string()))?;
    let max: f64 = params[1].parse().map_err(|_| rf_errors::RfError::Validation("Invalid max value".to_string()))?;
    if val < min || val > max {
        Err(rf_errors::RfError::Validation(format!("Value must be between {} and {}", min, max)))
    } else {
        Ok(())
    }
}

/// Validate same (must be same as another field)
pub fn validate_same(_value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("same needs field name".to_string()));
    }
    // Simplified - would compare with other field
    Ok(())
}

/// Validate different (must be different from another field)
pub fn validate_different(_value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("different needs field name".to_string()));
    }
    // Simplified - would compare with other field
    Ok(())
}

