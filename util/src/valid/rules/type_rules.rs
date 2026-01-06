//! # type_rules
//!
//! type_rules 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Type validation rules

use rf_errors::Result;

/// Validate integer
pub fn validate_integer(value: &str, _params: &[String]) -> Result<()> {
    value.parse::<i64>()
        .map_err(|_| rf_errors::RfError::Validation("Value must be an integer".to_string()))?;
    Ok(())
}

/// Validate float
pub fn validate_float(value: &str, _params: &[String]) -> Result<()> {
    value.parse::<f64>()
        .map_err(|_| rf_errors::RfError::Validation("Value must be a float".to_string()))?;
    Ok(())
}

/// Validate boolean
pub fn validate_boolean(value: &str, _params: &[String]) -> Result<()> {
    match value.to_lowercase().as_str() {
        "true" | "false" | "1" | "0" | "yes" | "no" | "on" | "off" => Ok(()),
        _ => Err(rf_errors::RfError::Validation("Value must be a boolean".to_string())),
    }
}

/// Validate date
pub fn validate_date(value: &str, _params: &[String]) -> Result<()> {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| rf_errors::RfError::Validation("Invalid date format (expected YYYY-MM-DD)".to_string()))?;
    Ok(())
}

/// Validate datetime
pub fn validate_datetime(value: &str, _params: &[String]) -> Result<()> {
    chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| rf_errors::RfError::Validation("Invalid datetime format (expected YYYY-MM-DD HH:MM:SS)".to_string()))?;
    Ok(())
}

/// Validate date format
pub fn validate_date_format(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("date_format needs format string".to_string()));
    }
    chrono::NaiveDate::parse_from_str(value, &params[0])
        .map_err(|_| rf_errors::RfError::Validation(format!("Invalid date format (expected {})", params[0])))?;
    Ok(())
}

