//! # format
//!
//! format 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Format validation rules

use regex::Regex;
use rf_errors::Result;

/// Validate regex pattern
pub fn validate_regex(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("regex needs pattern".to_string()));
    }
    let re = Regex::new(&params[0])
        .map_err(|e| rf_errors::RfError::Validation(format!("Invalid regex pattern: {}", e)))?;
    if !re.is_match(value) {
        Err(rf_errors::RfError::Validation("Value does not match pattern".to_string()))
    } else {
        Ok(())
    }
}

/// Validate not regex pattern
pub fn validate_not_regex(value: &str, params: &[String]) -> Result<()> {
    if params.is_empty() {
        return Err(rf_errors::RfError::Validation("not_regex needs pattern".to_string()));
    }
    let re = Regex::new(&params[0])
        .map_err(|e| rf_errors::RfError::Validation(format!("Invalid regex pattern: {}", e)))?;
    if re.is_match(value) {
        Err(rf_errors::RfError::Validation("Value matches pattern".to_string()))
    } else {
        Ok(())
    }
}

/// Validate email format
pub fn validate_email(value: &str, _params: &[String]) -> Result<()> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .unwrap();
    if !email_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid email format".to_string()))
    } else {
        Ok(())
    }
}

/// Validate URL format
pub fn validate_url(value: &str, _params: &[String]) -> Result<()> {
    let url_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$")
        .unwrap();
    if !url_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid URL format".to_string()))
    } else {
        Ok(())
    }
}

/// Validate IP address (IPv4 or IPv6)
pub fn validate_ip(value: &str, _params: &[String]) -> Result<()> {
    if validate_ipv4(value, &[]).is_ok() || validate_ipv6(value, &[]).is_ok() {
        Ok(())
    } else {
        Err(rf_errors::RfError::Validation("Invalid IP address".to_string()))
    }
}

/// Validate IPv4 address
pub fn validate_ipv4(value: &str, _params: &[String]) -> Result<()> {
    let ipv4_regex = Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$")
        .unwrap();
    if !ipv4_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid IPv4 address".to_string()))
    } else {
        Ok(())
    }
}

/// Validate IPv6 address
pub fn validate_ipv6(value: &str, _params: &[String]) -> Result<()> {
    // Simplified IPv6 validation
    let ipv6_regex = Regex::new(r"^([0-9a-fA-F]{0,4}:){7}[0-9a-fA-F]{0,4}$")
        .unwrap();
    if !ipv6_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid IPv6 address".to_string()))
    } else {
        Ok(())
    }
}

/// Validate MAC address
pub fn validate_mac(value: &str, _params: &[String]) -> Result<()> {
    let mac_regex = Regex::new(r"^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$")
        .unwrap();
    if !mac_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid MAC address".to_string()))
    } else {
        Ok(())
    }
}

