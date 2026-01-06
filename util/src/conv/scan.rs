//! # scan
//!
//! scan 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Scan functionality for automatic type inference and conversion

use rf_errors::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Scan options
pub struct ScanOptions {
    pub continue_on_error: bool,
    pub param_key_to_attr_map: Option<HashMap<String, String>>,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            continue_on_error: true,
            param_key_to_attr_map: None,
        }
    }
}

/// Scan value to pointer (automatic type inference)
pub fn scan<T: for<'de> Deserialize<'de>>(
    src: &impl Serialize,
    dst: &mut T,
    options: Option<ScanOptions>,
) -> Result<()> {
    let opts = options.unwrap_or_default();
    
    // Convert source to JSON
    let json = serde_json::to_value(src)
        .map_err(|e| rf_errors::RfError::Serialization(format!("Serialization failed: {}", e)))?;
    
    // Apply key mapping if provided
    let json = if let Some(ref map) = opts.param_key_to_attr_map {
        if let Some(obj) = json.as_object() {
            let mut new_obj = serde_json::Map::new();
            for (k, v) in obj {
                let new_key = map.get(k).cloned().unwrap_or_else(|| k.clone());
                new_obj.insert(new_key, v.clone());
            }
            serde_json::Value::Object(new_obj)
        } else {
            json
        }
    } else {
        json
    };
    
    // Deserialize to target type
    *dst = serde_json::from_value(json)
        .map_err(|e| rf_errors::RfError::Serialization(format!("Deserialization failed: {}", e)))?;
    
    Ok(())
}

