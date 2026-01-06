//! # array
//!
//! array 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Array validation rules

use rf_errors::Result;

/// Validate array
pub fn validate_array(value: &str, _params: &[String]) -> Result<()> {
    // Try to parse as JSON array
    let json: serde_json::Value = serde_json::from_str(value)
        .map_err(|_| rf_errors::RfError::Validation("Value must be a valid JSON array".to_string()))?;
    if !json.is_array() {
        Err(rf_errors::RfError::Validation("Value must be an array".to_string()))
    } else {
        Ok(())
    }
}

