//! # special
//!
//! special 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Special validation rules (phone, ID, etc.)

use regex::Regex;
use rf_errors::Result;

/// Validate Chinese phone number
pub fn validate_phone(value: &str, _params: &[String]) -> Result<()> {
    let phone_regex = Regex::new(r"^1[3-9]\d{9}$")
        .unwrap();
    if !phone_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid phone number".to_string()))
    } else {
        Ok(())
    }
}

/// Validate phone number (loose)
pub fn validate_phone_loose(value: &str, _params: &[String]) -> Result<()> {
    let phone_regex = Regex::new(r"^1\d{10}$")
        .unwrap();
    if !phone_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid phone number".to_string()))
    } else {
        Ok(())
    }
}

/// Validate telephone number
pub fn validate_telephone(value: &str, _params: &[String]) -> Result<()> {
    let tel_regex = Regex::new(r"^0\d{2,3}-?\d{7,8}$")
        .unwrap();
    if !tel_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid telephone number".to_string()))
    } else {
        Ok(())
    }
}

/// Validate passport number
pub fn validate_passport(value: &str, _params: &[String]) -> Result<()> {
    // Simplified passport validation
    if value.len() < 6 || value.len() > 9 {
        Err(rf_errors::RfError::Validation("Invalid passport number".to_string()))
    } else {
        Ok(())
    }
}

/// Validate Chinese resident ID
pub fn validate_resident_id(value: &str, _params: &[String]) -> Result<()> {
    let id_regex = Regex::new(r"^[1-9]\d{5}(18|19|20)\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])\d{3}[\dXx]$")
        .unwrap();
    if !id_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid resident ID".to_string()))
    } else {
        Ok(())
    }
}

/// Validate bank card number
pub fn validate_bank_card(value: &str, _params: &[String]) -> Result<()> {
    // Luhn algorithm check
    let digits: Vec<u32> = value.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    if digits.len() < 13 || digits.len() > 19 {
        return Err(rf_errors::RfError::Validation("Invalid bank card number length".to_string()));
    }
    // Simplified - would implement full Luhn algorithm
    Ok(())
}

/// Validate QQ number
pub fn validate_qq(value: &str, _params: &[String]) -> Result<()> {
    let qq_regex = Regex::new(r"^[1-9]\d{4,10}$")
        .unwrap();
    if !qq_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid QQ number".to_string()))
    } else {
        Ok(())
    }
}

/// Validate postcode
pub fn validate_postcode(value: &str, _params: &[String]) -> Result<()> {
    let postcode_regex = Regex::new(r"^\d{6}$")
        .unwrap();
    if !postcode_regex.is_match(value) {
        Err(rf_errors::RfError::Validation("Invalid postcode".to_string()))
    } else {
        Ok(())
    }
}

/// Validate password (basic)
pub fn validate_password(value: &str, _params: &[String]) -> Result<()> {
    if value.len() < 6 {
        Err(rf_errors::RfError::Validation("Password must be at least 6 characters".to_string()))
    } else {
        Ok(())
    }
}

/// Validate password (medium strength)
pub fn validate_password2(value: &str, _params: &[String]) -> Result<()> {
    if value.len() < 8 {
        return Err(rf_errors::RfError::Validation("Password must be at least 8 characters".to_string()));
    }
    let has_upper = value.chars().any(|c| c.is_uppercase());
    let has_lower = value.chars().any(|c| c.is_lowercase());
    let has_digit = value.chars().any(|c| c.is_ascii_digit());
    if !has_upper || !has_lower || !has_digit {
        Err(rf_errors::RfError::Validation("Password must contain uppercase, lowercase and digit".to_string()))
    } else {
        Ok(())
    }
}

/// Validate password (strong)
pub fn validate_password3(value: &str, _params: &[String]) -> Result<()> {
    if value.len() < 10 {
        return Err(rf_errors::RfError::Validation("Password must be at least 10 characters".to_string()));
    }
    let has_upper = value.chars().any(|c| c.is_uppercase());
    let has_lower = value.chars().any(|c| c.is_lowercase());
    let has_digit = value.chars().any(|c| c.is_ascii_digit());
    let has_special = value.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
    if !has_upper || !has_lower || !has_digit || !has_special {
        Err(rf_errors::RfError::Validation("Password must contain uppercase, lowercase, digit and special character".to_string()))
    } else {
        Ok(())
    }
}

