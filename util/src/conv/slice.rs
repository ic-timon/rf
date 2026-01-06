//! # slice
//!
//! slice 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Slice conversions

use rf_errors::Result;

/// Convert slice to Vec<i64>
pub fn slice_i64<T: Copy + TryInto<i64>>(values: &[T]) -> Result<Vec<i64>> {
    values.iter()
        .map(|&v| v.try_into().map_err(|_| rf_errors::RfError::Internal("Conversion failed".to_string())))
        .collect()
}

/// Convert slice to Vec<u64>
pub fn slice_u64<T: Copy + TryInto<u64>>(values: &[T]) -> Result<Vec<u64>> {
    values.iter()
        .map(|&v| v.try_into().map_err(|_| rf_errors::RfError::Internal("Conversion failed".to_string())))
        .collect()
}

/// Convert slice to Vec<f64>
pub fn slice_f64<T: Copy + TryInto<f64>>(values: &[T]) -> Result<Vec<f64>> {
    values.iter()
        .map(|&v| v.try_into().map_err(|_| rf_errors::RfError::Internal("Conversion failed".to_string())))
        .collect()
}

/// Convert slice to Vec<String>
pub fn slice_string<T: ToString>(values: &[T]) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}

/// Convert slice to Vec<bool>
pub fn slice_bool<T: Copy + TryInto<bool>>(values: &[T]) -> Result<Vec<bool>> {
    values.iter()
        .map(|&v| v.try_into().map_err(|_| rf_errors::RfError::Internal("Conversion failed".to_string())))
        .collect()
}
