//! # conv
//!
//! conv 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Type conversion utilities

pub mod conv {
    pub mod basic;
    pub mod int;
    pub mod uint;
    pub mod float;
    pub mod time;
    pub mod slice;
    pub mod map;
    pub mod struct_conv;
    pub mod scan;
    pub mod converter;
    
    pub use basic::*;
    pub use int::*;
    pub use uint::*;
    pub use float::*;
    pub use time::*;
    pub use slice::*;
    pub use map::*;
    pub use struct_conv::*;
    pub use scan::*;
    pub use converter::*;
}

pub use conv::*;

// Re-export common functions for backward compatibility
pub use conv::string as to_string;
pub use conv::i64 as to_i64;
pub use conv::f64 as to_f64;
pub use conv::bool as to_bool;

use rf_errors::Result;
use serde::{Deserialize, Serialize};

/// Convert via JSON serialization (backward compatibility)
pub fn convert<T: Serialize, U: for<'de> Deserialize<'de>>(value: T) -> Result<U> {
    let json = serde_json::to_value(value)
        .map_err(|e| rf_errors::RfError::Serialization(format!("Conversion error: {}", e)))?;
    serde_json::from_value(json)
        .map_err(|e| rf_errors::RfError::Serialization(format!("Conversion error: {}", e)))
}
