//! # uint
//!
//! uint 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Unsigned integer type conversions

use rf_errors::Result;

/// Convert value to u8
pub fn u8<T: TryInto<u8>>(value: T) -> Result<u8> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to u8".to_string()))
}

/// Convert value to u16
pub fn u16<T: TryInto<u16>>(value: T) -> Result<u16> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to u16".to_string()))
}

/// Convert value to u32
pub fn u32<T: TryInto<u32>>(value: T) -> Result<u32> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to u32".to_string()))
}

/// Convert value to u64
pub fn u64<T: TryInto<u64>>(value: T) -> Result<u64> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to u64".to_string()))
}

/// Convert value to usize
pub fn usize<T: TryInto<usize>>(value: T) -> Result<usize> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to usize".to_string()))
}

/// Convert string to u64
pub fn u64_from_str(s: &str) -> Result<u64> {
    s.parse::<u64>()
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to parse u64 from string: {}", e)))
}

/// Convert string to u32
pub fn u32_from_str(s: &str) -> Result<u32> {
    s.parse::<u32>()
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to parse u32 from string: {}", e)))
}

