//! # mode
//!
//! mode 模块 - 运行时模式管理工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 运行时模式管理工具模块
//!
//! 本模块提供了应用程序运行时模式的管理功能。
//! 通过环境变量 `RF_MODE` 来控制当前的运行模式。
//!
//! 支持的运行模式：
//! - Development（开发模式）
//! - Production（生产模式）
//! - Testing（测试模式）

use std::env;

/// 运行时模式枚举
///
/// 定义了应用程序的三种运行模式。
///
/// # 变体说明
/// - `Development`: 开发模式，通常启用详细的日志输出和调试功能
/// - `Production`: 生产模式，优化性能，关闭调试功能
/// - `Testing`: 测试模式，用于单元测试和集成测试
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// 开发模式
    Development,
    /// 生产模式
    Production,
    /// 测试模式
    Testing,
}

/// 获取当前的运行时模式
///
/// 从环境变量 `RF_MODE` 中读取并解析运行模式。
/// 如果环境变量未设置或值无效，默认返回开发模式。
///
/// # 环境变量
/// - `RF_MODE`: 运行模式
///   - "dev" 或 "development" -> Development
///   - "prod" 或 "production" -> Production
///   - "test" 或 "testing" -> Testing
///   - 其他值或未设置 -> Development（默认）
///
/// # 返回值
/// 返回当前的运行模式
///
/// # 示例
/// ```ignore
/// use rf_util::mode;
///
/// // 在 shell 中设置: export RF_MODE=production
/// let current_mode = mode::get();
/// println!("Current mode: {:?}", current_mode);
/// ```
pub fn get() -> Mode {
    match env::var("RF_MODE") {
        Ok(mode) => match mode.as_str() {
            "dev" | "development" => Mode::Development,
            "prod" | "production" => Mode::Production,
            "test" | "testing" => Mode::Testing,
            _ => Mode::Development,
        },
        Err(_) => Mode::Development,
    }
}

/// 检查是否为开发模式
///
/// # 返回值
/// - `true`: 当前是开发模式
/// - `false`: 当前不是开发模式
///
/// # 示例
/// ```ignore
/// use rf_util::mode;
///
/// if mode::is_dev() {
///     println!("Running in development mode");
/// }
/// ```
pub fn is_dev() -> bool {
    get() == Mode::Development
}

/// 检查是否为生产模式
///
/// # 返回值
/// - `true`: 当前是生产模式
/// - `false`: 当前不是生产模式
///
/// # 示例
/// ```ignore
/// use rf_util::mode;
///
/// if mode::is_prod() {
///     println!("Running in production mode");
/// }
/// ```
pub fn is_prod() -> bool {
    get() == Mode::Production
}

/// 检查是否为测试模式
///
/// # 返回值
/// - `true`: 当前是测试模式
/// - `false`: 当前不是测试模式
///
/// # 示例
/// ```ignore
/// use rf_util::mode;
///
/// if mode::is_test() {
///     println!("Running in testing mode");
/// }
/// ```
pub fn is_test() -> bool {
    get() == Mode::Testing
}

