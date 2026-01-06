//! # time
//!
//! time 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Time handling

use chrono::{DateTime, Local, NaiveDateTime, Utc};
use rf_errors::Result;

/// Get current time in UTC
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// Get current time in local timezone
pub fn now_local() -> DateTime<Local> {
    Local::now()
}

/// Format time as string
pub fn format(dt: &DateTime<Utc>, format: &str) -> String {
    dt.format(format).to_string()
}

/// Parse time from string
pub fn parse(s: &str, format: &str) -> Result<DateTime<Utc>> {
    let dt = NaiveDateTime::parse_from_str(s, format)
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to parse time: {}", e)))?;
    Ok(DateTime::from_naive_utc_and_offset(dt, Utc))
}

/// Get timestamp in seconds
pub fn timestamp(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp()
}

/// Get timestamp in milliseconds
pub fn timestamp_millis(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp_millis()
}

