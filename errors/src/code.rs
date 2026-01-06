//! # code
//!
//! code 模块 - RF 框架错误码定义模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 错误码定义模块
//!
//! 本模块定义了 RF 框架中使用的所有错误码常量。
//!
//! ## 错误码设计规范
//!
//! 错误码采用 32 位整数表示，遵循以下分段规则：
//!
//! - **0**：操作成功
//! - **400-499**：客户端错误（参数错误、未授权、禁止访问等）
//! - **500-599**：服务器内部错误
//! - **1000-1999**：数据库相关错误
//! - **2000-2999**：网络相关错误
//! - **3000-3999**：配置相关错误
//!
//! ## 使用示例
//!
//! ```rust
//! use errors::codes;
//!
//! fn check_result(code: i32) {
//!     match code {
//!         codes::OK => println!("操作成功"),
//!         codes::INVALID_PARAMETER => println!("参数错误"),
//!         codes::NOT_FOUND => println!("资源未找到"),
//!         codes::INTERNAL_ERROR => println!("内部错误"),
//!         _ => println!("未知错误码: {}", code),
//!     }
//! }
//! ```

/// 错误码类型定义
///
/// 使用 i32 类型表示错误码，支持广泛的错误码范围
pub type Code = i32;

/// 通用错误码常量定义
///
/// 本模块包含了 RF 框架中所有标准错误码的常量定义。
/// 所有错误码均遵循 HTTP 状态码规范，并扩展了自定义错误码段。
pub mod codes {
    use super::Code;

    /// 操作成功
    ///
    /// 表示操作已成功完成，无任何错误。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::OK, 0);
    /// ```
    pub const OK: Code = 0;

    /// 内部服务器错误
    ///
    /// 表示服务器在处理请求时遇到了意外情况，无法完成请求。
    /// 通常用于捕获未预期的错误或异常情况。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::INTERNAL_ERROR, 500);
    /// ```
    pub const INTERNAL_ERROR: Code = 500;

    /// 无效参数错误
    ///
    /// 表示客户端提供的请求参数不符合要求，包括：
    /// - 参数类型错误
    /// - 参数缺失
    /// - 参数格式错误
    /// - 参数值超出允许范围
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::INVALID_PARAMETER, 400);
    /// ```
    pub const INVALID_PARAMETER: Code = 400;

    /// 资源未找到错误
    ///
    /// 表示请求的资源不存在。常见场景包括：
    /// - 查询的数据库记录不存在
    /// - 请求的文件不存在
    /// - 请求的 API 端点不存在
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::NOT_FOUND, 404);
    /// ```
    pub const NOT_FOUND: Code = 404;

    /// 未授权错误
    ///
    /// 表示请求缺少有效的身份认证信息。
    /// 客户端需要进行身份验证才能访问请求的资源。
    ///
    /// # 常见场景
    ///
    /// - 缺少认证令牌
    /// - 认证令牌过期
    /// - 认证令牌格式错误
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::UNAUTHORIZED, 401);
    /// ```
    pub const UNAUTHORIZED: Code = 401;

    /// 禁止访问错误
    ///
    /// 表示客户端虽然已经通过身份认证，但没有权限访问请求的资源。
    /// 与 UNAUTHORIZED 不同，此错误表示服务器已识别客户端身份，但拒绝访问。
    ///
    /// # 常见场景
    ///
    /// - 用户权限不足
    /// - 资源访问被限制
    /// - 超出访问配额
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::FORBIDDEN, 403);
    /// ```
    pub const FORBIDDEN: Code = 403;

    /// 请求超时错误
    ///
    /// 表示服务器在等待客户端发送请求时超时，或服务器在处理请求时超时。
    ///
    /// # 常见场景
    ///
    /// - 网络延迟过高
    /// - 服务器负载过重
    /// - 资源锁定等待超时
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::TIMEOUT, 408);
    /// ```
    pub const TIMEOUT: Code = 408;

    /// 数据库错误
    ///
    /// 表示数据库操作过程中发生的错误。
    ///
    /// # 常见场景
    ///
    /// - 数据库连接失败
    /// - SQL 语法错误
    /// - 约束冲突
    /// - 事务失败
    /// - 死锁发生
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::DATABASE_ERROR, 1000);
    /// ```
    pub const DATABASE_ERROR: Code = 1000;

    /// 网络错误
    ///
    /// 表示网络操作过程中发生的错误。
    ///
    /// # 常见场景
    ///
    /// - 网络连接失败
    /// - DNS 解析失败
    /// - 连接被重置
    /// - 网络超时
    /// - 数据传输错误
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::NETWORK_ERROR, 2000);
    /// ```
    pub const NETWORK_ERROR: Code = 2000;

    /// 配置错误
    ///
    /// 表示系统配置不正确或缺失。
    ///
    /// # 常见场景
    ///
    /// - 配置文件缺失
    /// - 配置参数格式错误
    /// - 必需的配置项未设置
    /// - 配置值不符合要求
    ///
    /// # 示例
    ///
    /// ```rust
    /// use errors::codes;
    ///
    /// assert_eq!(codes::CONFIG_ERROR, 3000);
    /// ```
    pub const CONFIG_ERROR: Code = 3000;
}

