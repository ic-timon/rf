//! # i18n
//!
//! i18n 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Validation error message internationalization

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// Error message template
#[derive(Debug, Clone)]
pub struct ErrorTemplate {
    pub key: String,
    pub messages: HashMap<String, String>, // locale -> message
}

/// I18n manager for validation errors
pub struct ValidationI18n {
    templates: Arc<RwLock<HashMap<String, ErrorTemplate>>>,
    default_locale: String,
}

impl ValidationI18n {
    /// Create a new I18n manager
    pub fn new(default_locale: &str) -> Self {
        Self {
            templates: Arc::new(RwLock::new(HashMap::new())),
            default_locale: default_locale.to_string(),
        }
    }

    /// Register an error message template
    pub fn register_template(&self, key: String, locale: String, message: String) {
        let mut templates = self.templates.write();
        let template = templates.entry(key.clone()).or_insert_with(|| ErrorTemplate {
            key: key.clone(),
            messages: HashMap::new(),
        });
        template.messages.insert(locale, message);
    }

    /// Get error message for a rule
    pub fn get_message(&self, rule_name: &str, locale: Option<&str>, params: &[String]) -> String {
        let templates = self.templates.read();
        let locale = locale.unwrap_or(&self.default_locale);
        
        if let Some(template) = templates.get(rule_name) {
            // Try to get message for the locale
            if let Some(message) = template.messages.get(locale) {
                return format_message(message, params);
            }
            // Fallback to default locale
            if let Some(message) = template.messages.get(&self.default_locale) {
                return format_message(message, params);
            }
            // Fallback to first available message
            if let Some((_, message)) = template.messages.iter().next() {
                return format_message(message, params);
            }
        }
        
        // Default message
        format!("Validation failed for rule: {}", rule_name)
    }

    /// Set default locale
    pub fn set_default_locale(&mut self, locale: String) {
        self.default_locale = locale;
    }

    /// Get default locale
    pub fn default_locale(&self) -> &str {
        &self.default_locale
    }
}

/// Format message with parameters
fn format_message(template: &str, params: &[String]) -> String {
    let mut result = template.to_string();
    for param in params.iter() {
        // Replace first {} with the parameter
        if let Some(pos) = result.find("{}") {
            result.replace_range(pos..pos+2, param);
        }
    }
    result
}

/// Helper to register default messages
pub fn register_default_messages(i18n: &ValidationI18n) {
    let messages = vec![
        ("required", "en", "The {} field is required"),
        ("required", "zh", "{} 字段是必填的"),
        ("email", "en", "The {} field must be a valid email address"),
        ("email", "zh", "{} 字段必须是有效的邮箱地址"),
        ("min_length", "en", "The {} field must be at least {} characters"),
        ("min_length", "zh", "{} 字段至少需要 {} 个字符"),
        ("max_length", "en", "The {} field must not exceed {} characters"),
        ("max_length", "zh", "{} 字段不能超过 {} 个字符"),
        ("between", "en", "The {} field must be between {} and {}"),
        ("between", "zh", "{} 字段必须在 {} 和 {} 之间"),
    ];

    for (rule, locale, message) in messages {
        i18n.register_template(rule.to_string(), locale.to_string(), message.to_string());
    }
}

impl Default for ValidationI18n {
    fn default() -> Self {
        Self::new("en")
    }
}

