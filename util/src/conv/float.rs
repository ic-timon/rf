//! # float
//!
//! float 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Float type conversions

use rf_errors::Result;

/// Convert value to f32
pub fn f32<T: TryInto<f32>>(value: T) -> Result<f32> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to f32".to_string()))
}

/// Convert value to f64
pub fn f64<T: TryInto<f64>>(value: T) -> Result<f64> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to f64".to_string()))
}

/// Convert string to f64
pub fn f64_from_str(s: &str) -> Result<f64> {
    s.parse::<f64>()
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to parse f64 from string: {}", e)))
}

/// Convert string to f32
pub fn f32_from_str(s: &str) -> Result<f32> {
    s.parse::<f32>()
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to parse f32 from string: {}", e)))
}

