//! # rand
//!
//! rand 模块 - 随机数生成工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 随机数生成工具模块
//!
//! 本模块提供了各种随机数的生成功能，包括：
//! - 随机整数
//! - 随机浮点数
//! - 随机字符串
//!
//! 基于 rand crate 实现。

use rand::Rng;

/// 生成随机 i32 整数
///
/// 生成一个 i32 类型范围内的随机整数。
///
/// # 返回值
/// 返回一个随机的 i32 整数
///
/// # 示例
/// ```ignore
/// use rf_util::rand;
///
/// let num = rand::int();
/// println!("Random i32: {}", num);
/// ```
pub fn int() -> i32 {
    rand::thread_rng().gen()
}

/// 生成指定范围内的随机 i32 整数
///
/// # 参数
/// - `min`: 最小值（包含）
/// - `max`: 最大值（包含）
///
/// # 返回值
/// 返回一个在 [min, max] 范围内的随机 i32 整数
///
/// # 示例
/// ```ignore
/// use rf_util::rand;
///
/// let num = rand::int_range(1, 100);
/// println!("Random number between 1 and 100: {}", num);
/// ```
pub fn int_range(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

/// 生成随机 f64 浮点数
///
/// 生成一个 [0.0, 1.0) 范围内的随机浮点数。
///
/// # 返回值
/// 返回一个随机的 f64 浮点数
///
/// # 示例
/// ```ignore
/// use rf_util::rand;
///
/// let num = rand::float();
/// println!("Random f64: {}", num);
/// ```
pub fn float() -> f64 {
    rand::thread_rng().gen()
}

/// 生成指定范围内的随机 f64 浮点数
///
/// # 参数
/// - `min`: 最小值（包含）
/// - `max`: 最大值（包含）
///
/// # 返回值
/// 返回一个在 [min, max] 范围内的随机 f64 浮点数
///
/// # 示例
/// ```ignore
/// use rf_util::rand;
///
/// let num = rand::float_range(0.0, 100.0);
/// println!("Random number between 0.0 and 100.0: {}", num);
/// ```
pub fn float_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..=max)
}

/// 生成随机字母数字字符串
///
/// 生成一个包含大写字母、小写字母和数字的随机字符串。
///
/// # 参数
/// - `len`: 字符串的长度
///
/// # 返回值
/// 返回一个指定长度的随机字母数字字符串
///
/// # 示例
/// ```ignore
/// use rf_util::rand;
///
/// let s = rand::string(10);
/// println!("Random string: {}", s);
/// // 可能的输出: "aB3xY7kL2m"
/// ```
pub fn string(len: usize) -> String {
    use rand::distributions::Alphanumeric;
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

