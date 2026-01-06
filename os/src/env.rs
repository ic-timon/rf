//! # env
//!
//! env 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Environment variable operations

use std::env;

/// Get an environment variable
pub fn get(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// Get an environment variable with default value
pub fn get_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Set an environment variable
pub fn set(key: &str, value: &str) {
    env::set_var(key, value);
}

/// Remove an environment variable
pub fn remove(key: &str) {
    env::remove_var(key);
}

/// Check if an environment variable exists
pub fn has(key: &str) -> bool {
    env::var(key).is_ok()
}

/// Get all environment variables
pub fn all() -> Vec<(String, String)> {
    env::vars().collect()
}

