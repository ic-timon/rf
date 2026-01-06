//! # error
//!
//! error 模块 - RF 框架错误类型定义模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 错误类型和处理模块
//!
//! 本模块定义了 RF 框架中使用的所有错误类型，并提供错误处理机制。
//!
//! ## 主要功能
//!
//! - **错误类型定义**：使用枚举定义所有可能的错误情况
//! - **错误码映射**：为每种错误类型关联对应的错误码
//! - **自动转换**：支持从标准库错误类型自动转换
//! - **错误信息**：提供详细的错误描述信息
//!
//! ## 使用示例
//!
//! ### 基本使用
//!
//! ```rust
//! use errors::{RfError, Result};
//!
//! fn read_file(path: &str) -> Result<String> {
//!     std::fs::read_to_string(path)
//!         .map_err(|e| RfError::NotFound(format!("文件未找到: {}", path)))?;
//!     Ok("成功".to_string())
//! }
//! ```
//!
//! ### 错误处理
//!
//! ```rust
//! use errors::{RfError, Result};
//!
//! fn handle_error() -> Result<()> {
//!     Err(RfError::InvalidParameter("用户名不能为空".to_string()))
//! }
//!
//! match handle_error() {
//!     Ok(_) => println!("操作成功"),
//!     Err(e) => {
//!         eprintln!("错误码: {}", e.code());
//!         eprintln!("错误信息: {}", e.message());
//!     }
//! }
//! ```
//!
//! ### 错误传播
//!
//! ```rust
//! use errors::{RfError, Result};
//!
//! fn inner_function() -> Result<String> {
//!     Err(RfError::Database("数据库连接失败".to_string()))
//! }
//!
//! fn outer_function() -> Result<String> {
//!     inner_function()?;  // 使用 ? 运算符传播错误
//!     Ok("成功".to_string())
//! }
//! ```

use thiserror::Error;

use crate::code::Code;

/// RF 框架错误类型
///
/// 本枚举定义了 RF 框架中所有可能发生的错误类型。
/// 每个错误变体都包含一个描述性的错误消息。
///
/// 错误类型通过 `thiserror` crate 实现，自动提供：
/// - `Display` trait：错误信息格式化输出
/// - `Error` trait：标准错误接口
/// - `Debug` trait：错误调试信息
///
/// # 错误变体说明
///
/// - `Internal`：内部服务器错误
/// - `InvalidParameter`：参数验证失败
/// - `NotFound`：资源未找到
/// - `Unauthorized`：未授权访问
/// - `Forbidden`：权限不足
/// - `Timeout`：操作超时
/// - `Database`：数据库操作错误
/// - `Network`：网络通信错误
/// - `Config`：配置错误
/// - `Io`：IO 操作错误（自动从 `std::io::Error` 转换）
/// - `Serialization`：序列化/反序列化错误
/// - `Validation`：数据验证错误
/// - `Custom`：自定义错误
///
/// # 示例
///
/// ```rust
/// use errors::RfError;
///
/// // 创建错误
/// let error = RfError::NotFound("用户不存在".to_string());
/// println!("{}", error);  // 输出: Not found: 用户不存在
///
/// // 从 IO 错误自动转换
/// let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "文件未找到");
/// let rf_error: RfError = io_error.into();
/// ```
#[derive(Error, Debug)]
pub enum RfError {
    /// 内部错误
    ///
    /// 表示服务器在处理请求时遇到了意外情况。
    /// 通常用于捕获未预期的错误或异常情况。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Internal("服务暂时不可用".to_string());
    /// ```
    #[error("Internal error: {0}")]
    Internal(String),

    /// 无效参数错误
    ///
    /// 表示客户端提供的请求参数不符合要求。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明哪个参数无效以及原因
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::InvalidParameter("用户ID必须大于0".to_string());
    /// ```
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// 资源未找到错误
    ///
    /// 表示请求的资源不存在。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明哪个资源未找到
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::NotFound("用户ID: 12345 不存在".to_string());
    /// ```
    #[error("Not found: {0}")]
    NotFound(String),

    /// 未授权错误
    ///
    /// 表示请求缺少有效的身份认证信息。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明认证失败的原因
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Unauthorized("缺少认证令牌".to_string());
    /// ```
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 禁止访问错误
    ///
    /// 表示客户端虽然已通过身份认证，但没有权限访问请求的资源。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明为什么访问被拒绝
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Forbidden("需要管理员权限".to_string());
    /// ```
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 超时错误
    ///
    /// 表示操作在规定时间内未能完成。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明哪个操作超时
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Timeout("数据库查询超时".to_string());
    /// ```
    #[error("Timeout: {0}")]
    Timeout(String),

    /// 数据库错误
    ///
    /// 表示数据库操作过程中发生的错误。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明具体的数据库错误
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Database("连接数据库失败".to_string());
    /// ```
    #[error("Database error: {0}")]
    Database(String),

    /// 网络错误
    ///
    /// 表示网络操作过程中发生的错误。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明具体的网络错误
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Network("无法连接到服务器".to_string());
    /// ```
    #[error("Network error: {0}")]
    Network(String),

    /// 配置错误
    ///
    /// 表示系统配置不正确或缺失。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明哪个配置项有问题
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Config("数据库配置缺失".to_string());
    /// ```
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO 错误
    ///
    /// 表示输入/输出操作过程中发生的错误。
    ///
    /// 注意：此变体可以从 `std::io::Error` 自动转换，
    /// 因此使用 `?` 运算符时会自动将 IO 错误转换为 `RfError`。
    ///
    /// # 参数
    ///
    /// - `0`：原始的 IO 错误对象
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    /// use std::fs;
    ///
    /// // 自动转换
    /// let content: Result<String, RfError> = fs::read_to_string("file.txt")?;
    /// ```
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// 序列化错误
    ///
    /// 表示数据序列化或反序列化过程中发生的错误。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明序列化失败的原因
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Serialization("JSON 格式错误".to_string());
    /// ```
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// 验证错误
    ///
    /// 表示数据验证失败。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息，应说明哪个验证规则未通过
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Validation("邮箱格式不正确".to_string());
    /// ```
    #[error("Validation error: {0}")]
    Validation(String),

    /// 自定义错误
    ///
    /// 用于处理未包含在上述标准错误类型中的特殊情况。
    ///
    /// # 参数
    ///
    /// - `0`：错误描述信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::Custom("自定义业务逻辑错误".to_string());
    /// ```
    #[error("Custom error: {0}")]
    Custom(String),
}

impl RfError {
    /// 获取错误码
    ///
    /// 根据错误类型返回对应的错误码。
    /// 错误码遵循 HTTP 状态码规范和自定义错误码段。
    ///
    /// # 返回值
    ///
    /// 返回与当前错误类型对应的错误码：
    ///
    /// - `Internal` → 500 (内部错误)
    /// - `InvalidParameter` → 400 (参数错误)
    /// - `NotFound` → 404 (未找到)
    /// - `Unauthorized` → 401 (未授权)
    /// - `Forbidden` → 403 (禁止访问)
    /// - `Timeout` → 408 (超时)
    /// - `Database` → 1000 (数据库错误)
    /// - `Network` → 2000 (网络错误)
    /// - `Config` → 3000 (配置错误)
    /// - `Io` → 500 (映射为内部错误)
    /// - `Serialization` → 500 (映射为内部错误)
    /// - `Validation` → 400 (映射为参数错误)
    /// - `Custom` → 500 (映射为内部错误)
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::NotFound("资源未找到".to_string());
    /// assert_eq!(error.code(), 404);
    ///
    /// let error = RfError::Database("连接失败".to_string());
    /// assert_eq!(error.code(), 1000);
    /// ```
    pub fn code(&self) -> Code {
        use crate::code::codes;
        match self {
            Self::Internal(_) => codes::INTERNAL_ERROR,
            Self::InvalidParameter(_) => codes::INVALID_PARAMETER,
            Self::NotFound(_) => codes::NOT_FOUND,
            Self::Unauthorized(_) => codes::UNAUTHORIZED,
            Self::Forbidden(_) => codes::FORBIDDEN,
            Self::Timeout(_) => codes::TIMEOUT,
            Self::Database(_) => codes::DATABASE_ERROR,
            Self::Network(_) => codes::NETWORK_ERROR,
            Self::Config(_) => codes::CONFIG_ERROR,
            Self::Io(_) => codes::INTERNAL_ERROR,
            Self::Serialization(_) => codes::INTERNAL_ERROR,
            Self::Validation(_) => codes::INVALID_PARAMETER,
            Self::Custom(_) => codes::INTERNAL_ERROR,
        }
    }

    /// 获取错误消息
    ///
    /// 返回错误的完整描述信息，包括错误类型和具体消息。
    ///
    /// # 返回值
    ///
    /// 返回格式化后的错误消息字符串。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::RfError;
    ///
    /// let error = RfError::NotFound("用户ID: 123".to_string());
    /// let message = error.message();
    /// assert_eq!(message, "Not found: 用户ID: 123");
    ///
    /// // 也可以直接使用 Display trait
    /// println!("{}", error);  // 输出: Not found: 用户ID: 123
    /// ```
    pub fn message(&self) -> String {
        format!("{}", self)
    }
}

/// Result 类型别名
///
/// 为 `RfError` 定义的结果类型别名，简化返回类型的书写。
///
/// # 类型参数
///
/// - `T`：成功时的返回值类型
///
/// # 示例
///
/// ```rust
/// use errors::{RfError, Result};
///
/// fn get_user(id: i32) -> Result<String> {
///     if id > 0 {
///         Ok(format!("用户{}", id))
///     } else {
///         Err(RfError::InvalidParameter("ID必须大于0".to_string()))
///     }
/// }
///
/// // 使用
/// match get_user(1) {
///     Ok(user) => println!("{}", user),
///     Err(e) => eprintln!("错误: {}", e),
/// }
/// ```
pub type Result<T> = std::result::Result<T, RfError>;

