//! # json
//!
//! json 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! JSON validation rules

use rf_errors::Result;

/// Validate JSON
pub fn validate_json(value: &str, _params: &[String]) -> Result<()> {
    serde_json::from_str::<serde_json::Value>(value)
        .map_err(|_| rf_errors::RfError::Validation("Value must be valid JSON".to_string()))?;
    Ok(())
}

