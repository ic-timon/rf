//! # time
//!
//! time 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Time type conversions

use chrono::{DateTime, NaiveDateTime, Utc};
use rf_errors::Result;

/// Convert value to DateTime<Utc>
pub fn datetime<T: Into<DateTime<Utc>>>(value: T) -> DateTime<Utc> {
    value.into()
}

/// Convert string to DateTime<Utc>
pub fn datetime_from_str(s: &str, format: Option<&str>) -> Result<DateTime<Utc>> {
    let format_str = format.unwrap_or("%Y-%m-%d %H:%M:%S");
    NaiveDateTime::parse_from_str(s, format_str)
        .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to parse datetime: {}", e)))
}

/// Convert to duration
pub fn duration<T: Into<chrono::Duration>>(value: T) -> chrono::Duration {
    value.into()
}

/// Convert seconds to duration
pub fn duration_from_secs(secs: i64) -> chrono::Duration {
    chrono::Duration::seconds(secs)
}

