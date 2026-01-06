//! # guid
//!
//! guid 模块 - UUID/GUID 生成和解析工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! UUID/GUID 工具模块
//!
//! 本模块提供了 UUID（通用唯一标识符）的生成和解析功能。
//! 基于 uuid crate 实现，支持 UUID v4 标准。

use uuid::Uuid;

/// 生成一个新的 UUID v4
///
/// UUID v4 是基于随机数生成的 UUID，具有极高的唯一性。
/// 生成的 UUID 格式为：`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`
///
/// # 返回值
/// 返回包含连字符的标准 UUID 字符串（36 个字符）
///
/// # 示例
/// ```ignore
/// use rf_util::guid;
///
/// let uuid = guid::new();
/// println!("Generated UUID: {}", uuid);
/// // 输出示例: 550e8400-e29b-41d4-a716-446655440000
/// ```
pub fn new() -> String {
    Uuid::new_v4().to_string()
}

/// 生成一个不带连字符的简化 UUID
///
/// 与 `new()` 函数类似，但移除了 UUID 中的连字符分隔符。
/// 适用于需要紧凑格式的场景。
///
/// # 返回值
/// 返回不包含连字符的 UUID 字符串（32 个十六进制字符）
///
/// # 示例
/// ```ignore
/// use rf_util::guid;
///
/// let uuid = guid::new_simple();
/// println!("Generated UUID: {}", uuid);
/// // 输出示例: 550e8400e29b41d4a716446655440000
/// ```
pub fn new_simple() -> String {
    Uuid::new_v4().simple().to_string()
}

/// 从字符串解析 UUID
///
/// # 参数
/// - `s`: UUID 字符串，可以是带连字符或不带连字符的格式
///
/// # 返回值
/// - `Ok(Uuid)`: 解析成功，返回 Uuid 对象
/// - `Err(uuid::Error)`: 解析失败，返回错误信息
///
/// # 示例
/// ```ignore
/// use rf_util::guid;
///
/// // 解析带连字符的 UUID
/// let uuid1 = guid::parse("550e8400-e29b-41d4-a716-446655440000");
/// assert!(uuid1.is_ok());
///
/// // 解析不带连字符的 UUID
/// let uuid2 = guid::parse("550e8400e29b41d4a716446655440000");
/// assert!(uuid2.is_ok());
///
/// // 解析无效的 UUID
/// let uuid3 = guid::parse("invalid-uuid");
/// assert!(uuid3.is_err());
/// ```
pub fn parse(s: &str) -> Result<Uuid, uuid::Error> {
    Uuid::parse_str(s)
}

