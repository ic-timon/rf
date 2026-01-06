//! # structs
//!
//! structs 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Struct utilities

use serde::{Deserialize, Serialize};
use rf_errors::{Result, RfError};

/// Convert struct to map
pub fn to_map<T: Serialize>(value: &T) -> Result<serde_json::Map<String, serde_json::Value>> {
    let json = serde_json::to_value(value)
        .map_err(|e| RfError::Serialization(format!("Failed to serialize: {}", e)))?;
    json.as_object()
        .cloned()
        .ok_or_else(|| RfError::Serialization("Not an object".to_string()))
}

/// Convert map to struct
pub fn from_map<T: for<'de> Deserialize<'de>>(map: &serde_json::Map<String, serde_json::Value>) -> Result<T> {
    let json = serde_json::Value::Object(map.clone());
    serde_json::from_value(json)
        .map_err(|e| RfError::Serialization(format!("Failed to deserialize: {}", e)))
}
