//! # map
//!
//! map 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Map conversions

use rf_errors::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Convert map to struct
pub fn map_to_struct<T: for<'de> Deserialize<'de>>(
    map: &HashMap<String, serde_json::Value>
) -> Result<T> {
    let json = serde_json::to_value(map)
        .map_err(|e| rf_errors::RfError::Serialization(format!("Map to JSON failed: {}", e)))?;
    serde_json::from_value(json)
        .map_err(|e| rf_errors::RfError::Serialization(format!("JSON to struct failed: {}", e)))
}

/// Convert struct to map
pub fn struct_to_map<T: Serialize>(value: &T) -> Result<HashMap<String, serde_json::Value>> {
    let json = serde_json::to_value(value)
        .map_err(|e| rf_errors::RfError::Serialization(format!("Struct to JSON failed: {}", e)))?;
    json.as_object()
        .ok_or_else(|| rf_errors::RfError::Internal("Not an object".to_string()))
        .map(|obj| {
            let mut map = HashMap::new();
            for (k, v) in obj {
                map.insert(k.clone(), v.clone());
            }
            map
        })
}

/// Convert map to map with different value types
pub fn map_to_map<K: std::hash::Hash + Eq + for<'de> Deserialize<'de>, V: for<'de> Deserialize<'de>>(
    map: &HashMap<String, serde_json::Value>
) -> Result<HashMap<K, V>> {
    let mut result = HashMap::new();
    for (k, v) in map {
        let key: K = serde_json::from_str(k)
            .map_err(|e| rf_errors::RfError::Serialization(format!("Key conversion failed: {}", e)))?;
        let value: V = serde_json::from_value(v.clone())
            .map_err(|e| rf_errors::RfError::Serialization(format!("Value conversion failed: {}", e)))?;
        result.insert(key, value);
    }
    Ok(result)
}

