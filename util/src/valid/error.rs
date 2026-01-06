//! # error
//!
//! error 模块 - 验证错误处理工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 验证错误处理模块
//!
//! 本模块提供了验证错误的数据结构和处理功能，包括：
//! - ValidationError: 单个验证错误
//! - ValidationErrors: 验证错误集合
//! - 错误消息的默认值

use std::collections::HashMap;

/// 验证错误结构体
///
/// 表示单个字段验证失败的信息。
///
/// # 字段说明
/// - `field`: 验证失败的字段名
/// - `rule`: 失败的验证规则名称
/// - `message`: 错误消息
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub rule: String,
    pub message: String,
}

impl ValidationError {
    /// 创建一个新的验证错误
    ///
    /// # 参数
    /// - `field`: 验证失败的字段名
    /// - `rule`: 失败的验证规则名称
    /// - `message`: 错误消息
    ///
    /// # 返回值
    /// 返回一个新的 ValidationError 实例
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::valid::error::ValidationError;
    ///
    /// let error = ValidationError::new("email", "required", "Email field is required");
    /// ```
    pub fn new(field: &str, rule: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            rule: rule.to_string(),
            message: message.to_string(),
        }
    }
}

/// 验证错误集合
///
/// 用于收集和管理多个验证错误。
///
/// # 字段说明
/// - `errors`: 按字段名分组的验证错误集合
#[derive(Debug)]
pub struct ValidationErrors {
    errors: HashMap<String, Vec<ValidationError>>,
}

impl ValidationErrors {
    /// 创建一个新的验证错误集合
    ///
    /// # 返回值
    /// 返回一个空的 ValidationErrors 实例
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::valid::error::ValidationErrors;
    ///
    /// let errors = ValidationErrors::new();
    /// ```
    pub fn new() -> Self {
        Self {
            errors: HashMap::new(),
        }
    }

    /// 添加一个验证错误
    ///
    /// # 参数
    /// - `error`: 要添加的验证错误
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::valid::error::{ValidationErrors, ValidationError};
    ///
    /// let mut errors = ValidationErrors::new();
    /// let error = ValidationError::new("email", "required", "Email is required");
    /// errors.add(error);
    /// ```
    pub fn add(&mut self, error: ValidationError) {
        self.errors.entry(error.field.clone())
            .or_default()
            .push(error);
    }

    /// 检查是否有任何错误
    ///
    /// # 返回值
    /// - `true`: 没有错误
    /// - `false`: 有错误
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::valid::error::ValidationErrors;
    ///
    /// let errors = ValidationErrors::new();
    /// assert!(errors.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// 获取所有错误
    ///
    /// # 返回值
    /// 返回包含所有错误的 HashMap 引用
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::valid::error::ValidationErrors;
    ///
    /// let errors = ValidationErrors::new();
    /// let all_errors = errors.all();
    /// ```
    pub fn all(&self) -> &HashMap<String, Vec<ValidationError>> {
        &self.errors
    }

    /// 获取指定字段的错误
    ///
    /// # 参数
    /// - `field`: 字段名
    ///
    /// # 返回值
    /// - `Some(&Vec<ValidationError>)`: 该字段的错误列表
    /// - `None`: 该字段没有错误
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::valid::error::ValidationErrors;
    ///
    /// let errors = ValidationErrors::new();
    /// if let Some(field_errors) = errors.get("email") {
    ///     for error in field_errors {
    ///         println!("Error: {}", error.message);
    ///     }
    /// }
    /// ```
    pub fn get(&self, field: &str) -> Option<&Vec<ValidationError>> {
        self.errors.get(field)
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

/// 自定义错误消息映射类型
///
/// 用于为不同的验证规则定义自定义错误消息。
/// 键是验证规则名称，值是错误消息模板。
pub type ErrorMessageMap = HashMap<String, String>;

/// 获取验证规则的默认错误消息
///
/// # 参数
/// - `rule`: 验证规则名称
///
/// # 返回值
/// 返回该规则的默认错误消息（静态字符串）
///
/// # 支持的规则
/// - "required": "Field is required"
/// - "email": "Invalid email format"
/// - "url": "Invalid URL format"
/// - "ip": "Invalid IP address"
/// - "phone": "Invalid phone number"
/// - 其他: "Validation failed"
///
/// # 示例
/// ```ignore
/// use rf_util::valid::error::default_error_message;
///
/// let msg = default_error_message("required");
/// println!("{}", msg); // 输出: "Field is required"
/// ```
pub fn default_error_message(rule: &str) -> &'static str {
    match rule {
        "required" => "Field is required",
        "email" => "Invalid email format",
        "url" => "Invalid URL format",
        "ip" => "Invalid IP address",
        "phone" => "Invalid phone number",
        _ => "Validation failed",
    }
}

