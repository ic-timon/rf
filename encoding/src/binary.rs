//! # binary
//!
//! binary 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # 二进制编码/解码模块
//!
//! 提供基本数据类型与字节数组之间的转换功能，支持大端序和小端序。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{
//!     encode_i32_be, decode_i32_be,
//!     encode_i32_le, decode_i32_le
//! };
//!
//! // 大端序编码和解码
//! let value: i32 = 123456;
//! let bytes_be = encode_i32_be(value);
//! let decoded_be = decode_i32_be(&bytes_be);
//! assert_eq!(value, decoded_be);
//!
//! // 小端序编码和解码
//! let bytes_le = encode_i32_le(value);
//! let decoded_le = decode_i32_le(&bytes_le);
//! assert_eq!(value, decoded_le);
//! ```
//!
//! ## 字节序说明
//!
//! - **大端序（Big-Endian）**: 最高有效字节在前，网络字节序通常使用大端序
//! - **小端序（Little-Endian）**: 最低有效字节在前，Intel x86 架构使用小端序

use std::mem;

/// 将 i8 类型编码为字节数组
///
/// # 参数
///
/// * `value` - 要编码的 i8 值
///
/// # 返回值
///
/// 返回包含 1 个字节的数组
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_i8;
///
/// let bytes = encode_i8(42);
/// assert_eq!(bytes, [42]);
/// ```
pub fn encode_i8(value: i8) -> [u8; 1] {
    value.to_be_bytes()
}

/// 从字节数组解码为 i8 类型
///
/// # 参数
///
/// * `bytes` - 包含 1 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 i8 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_i8;
///
/// let value = decode_i8(&[42]);
/// assert_eq!(value, 42);
/// ```
pub fn decode_i8(bytes: &[u8]) -> i8 {
    bytes[0] as i8
}

/// 将 u8 类型编码为字节数组
///
/// # 参数
///
/// * `value` - 要编码的 u8 值
///
/// # 返回值
///
/// 返回包含 1 个字节的数组
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_u8;
///
/// let bytes = encode_u8(255);
/// assert_eq!(bytes, [255]);
/// ```
pub fn encode_u8(value: u8) -> [u8; 1] {
    [value]
}

/// 从字节数组解码为 u8 类型
///
/// # 参数
///
/// * `bytes` - 包含 1 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 u8 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_u8;
///
/// let value = decode_u8(&[255]);
/// assert_eq!(value, 255);
/// ```
pub fn decode_u8(bytes: &[u8]) -> u8 {
    bytes[0]
}

/// 将 i16 类型编码为大端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 i16 值
///
/// # 返回值
///
/// 返回包含 2 个字节的数组（大端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_i16_be;
///
/// let bytes = encode_i16_be(1000);
/// assert_eq!(bytes, [3, 232]);
/// ```
pub fn encode_i16_be(value: i16) -> [u8; 2] {
    value.to_be_bytes()
}

/// 将 i16 类型编码为小端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 i16 值
///
/// # 返回值
///
/// 返回包含 2 个字节的数组（小端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_i16_le;
///
/// let bytes = encode_i16_le(1000);
/// assert_eq!(bytes, [232, 3]);
/// ```
pub fn encode_i16_le(value: i16) -> [u8; 2] {
    value.to_le_bytes()
}

/// 从大端序字节数组解码为 i16 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 2 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 i16 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_i16_be;
///
/// let value = decode_i16_be(&[3, 232]);
/// assert_eq!(value, 1000);
/// ```
pub fn decode_i16_be(bytes: &[u8]) -> i16 {
    i16::from_be_bytes([bytes[0], bytes[1]])
}

/// 从小端序字节数组解码为 i16 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 2 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 i16 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_i16_le;
///
/// let value = decode_i16_le(&[232, 3]);
/// assert_eq!(value, 1000);
/// ```
pub fn decode_i16_le(bytes: &[u8]) -> i16 {
    i16::from_le_bytes([bytes[0], bytes[1]])
}

/// 将 u16 类型编码为大端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 u16 值
///
/// # 返回值
///
/// 返回包含 2 个字节的数组（大端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_u16_be;
///
/// let bytes = encode_u16_be(1000);
/// assert_eq!(bytes, [3, 232]);
/// ```
pub fn encode_u16_be(value: u16) -> [u8; 2] {
    value.to_be_bytes()
}

/// 将 u16 类型编码为小端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 u16 值
///
/// # 返回值
///
/// 返回包含 2 个字节的数组（小端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_u16_le;
///
/// let bytes = encode_u16_le(1000);
/// assert_eq!(bytes, [232, 3]);
/// ```
pub fn encode_u16_le(value: u16) -> [u8; 2] {
    value.to_le_bytes()
}

/// 从大端序字节数组解码为 u16 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 2 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 u16 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_u16_be;
///
/// let value = decode_u16_be(&[3, 232]);
/// assert_eq!(value, 1000);
/// ```
pub fn decode_u16_be(bytes: &[u8]) -> u16 {
    u16::from_be_bytes([bytes[0], bytes[1]])
}

/// 从小端序字节数组解码为 u16 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 2 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 u16 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_u16_le;
///
/// let value = decode_u16_le(&[232, 3]);
/// assert_eq!(value, 1000);
/// ```
pub fn decode_u16_le(bytes: &[u8]) -> u16 {
    u16::from_le_bytes([bytes[0], bytes[1]])
}

/// 将 i32 类型编码为大端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 i32 值
///
/// # 返回值
///
/// 返回包含 4 个字节的数组（大端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_i32_be;
///
/// let bytes = encode_i32_be(123456);
/// assert_eq!(bytes, [0, 1, 226, 64]);
/// ```
pub fn encode_i32_be(value: i32) -> [u8; 4] {
    value.to_be_bytes()
}

/// 将 i32 类型编码为小端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 i32 值
///
/// # 返回值
///
/// 返回包含 4 个字节的数组（小端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_i32_le;
///
/// let bytes = encode_i32_le(123456);
/// assert_eq!(bytes, [64, 226, 1, 0]);
/// ```
pub fn encode_i32_le(value: i32) -> [u8; 4] {
    value.to_le_bytes()
}

/// 从大端序字节数组解码为 i32 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 4 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 i32 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_i32_be;
///
/// let value = decode_i32_be(&[0, 1, 226, 64]);
/// assert_eq!(value, 123456);
/// ```
pub fn decode_i32_be(bytes: &[u8]) -> i32 {
    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// 从小端序字节数组解码为 i32 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 4 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 i32 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_i32_le;
///
/// let value = decode_i32_le(&[64, 226, 1, 0]);
/// assert_eq!(value, 123456);
/// ```
pub fn decode_i32_le(bytes: &[u8]) -> i32 {
    i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// 将 u32 类型编码为大端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 u32 值
///
/// # 返回值
///
/// 返回包含 4 个字节的数组（大端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_u32_be;
///
/// let bytes = encode_u32_be(123456);
/// assert_eq!(bytes, [0, 1, 226, 64]);
/// ```
pub fn encode_u32_be(value: u32) -> [u8; 4] {
    value.to_be_bytes()
}

/// 将 u32 类型编码为小端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 u32 值
///
/// # 返回值
///
/// 返回包含 4 个字节的数组（小端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_u32_le;
///
/// let bytes = encode_u32_le(123456);
/// assert_eq!(bytes, [64, 226, 1, 0]);
/// ```
pub fn encode_u32_le(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

/// 从大端序字节数组解码为 u32 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 4 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 u32 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_u32_be;
///
/// let value = decode_u32_be(&[0, 1, 226, 64]);
/// assert_eq!(value, 123456);
/// ```
pub fn decode_u32_be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// 从小端序字节数组解码为 u32 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 4 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 u32 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_u32_le;
///
/// let value = decode_u32_le(&[64, 226, 1, 0]);
/// assert_eq!(value, 123456);
/// ```
pub fn decode_u32_le(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// 将 i64 类型编码为大端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 i64 值
///
/// # 返回值
///
/// 返回包含 8 个字节的数组（大端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_i64_be;
///
/// let bytes = encode_i64_be(123456789);
/// ```
pub fn encode_i64_be(value: i64) -> [u8; 8] {
    value.to_be_bytes()
}

/// 将 i64 类型编码为小端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 i64 值
///
/// # 返回值
///
/// 返回包含 8 个字节的数组（小端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_i64_le;
///
/// let bytes = encode_i64_le(123456789);
/// ```
pub fn encode_i64_le(value: i64) -> [u8; 8] {
    value.to_le_bytes()
}

/// 从大端序字节数组解码为 i64 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 8 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 i64 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_i64_be;
///
/// let value = decode_i64_be(&[0, 0, 0, 0, 7, 91, 205, 21]);
/// assert_eq!(value, 123456789);
/// ```
pub fn decode_i64_be(bytes: &[u8]) -> i64 {
    i64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

/// 从小端序字节数组解码为 i64 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 8 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 i64 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_i64_le;
///
/// let value = decode_i64_le(&[21, 205, 91, 7, 0, 0, 0, 0]);
/// assert_eq!(value, 123456789);
/// ```
pub fn decode_i64_le(bytes: &[u8]) -> i64 {
    i64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

/// 将 u64 类型编码为大端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 u64 值
///
/// # 返回值
///
/// 返回包含 8 个字节的数组（大端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_u64_be;
///
/// let bytes = encode_u64_be(123456789);
/// ```
pub fn encode_u64_be(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

/// 将 u64 类型编码为小端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 u64 值
///
/// # 返回值
///
/// 返回包含 8 个字节的数组（小端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_u64_le;
///
/// let bytes = encode_u64_le(123456789);
/// ```
pub fn encode_u64_le(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}

/// 从大端序字节数组解码为 u64 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 8 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 u64 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_u64_be;
///
/// let value = decode_u64_be(&[0, 0, 0, 0, 7, 91, 205, 21]);
/// assert_eq!(value, 123456789);
/// ```
pub fn decode_u64_be(bytes: &[u8]) -> u64 {
    u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

/// 从小端序字节数组解码为 u64 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 8 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 u64 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_u64_le;
///
/// let value = decode_u64_le(&[21, 205, 91, 7, 0, 0, 0, 0]);
/// assert_eq!(value, 123456789);
/// ```
pub fn decode_u64_le(bytes: &[u8]) -> u64 {
    u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

/// 将 f32 类型编码为大端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 f32 值
///
/// # 返回值
///
/// 返回包含 4 个字节的数组（大端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_f32_be;
///
/// let bytes = encode_f32_be(3.14);
/// ```
pub fn encode_f32_be(value: f32) -> [u8; 4] {
    value.to_be_bytes()
}

/// 将 f32 类型编码为小端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 f32 值
///
/// # 返回值
///
/// 返回包含 4 个字节的数组（小端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_f32_le;
///
/// let bytes = encode_f32_le(3.14);
/// ```
pub fn encode_f32_le(value: f32) -> [u8; 4] {
    value.to_le_bytes()
}

/// 从大端序字节数组解码为 f32 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 4 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 f32 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_f32_be;
///
/// let bytes = encode_f32_be(3.14);
/// let value = decode_f32_be(&bytes);
/// assert!((value - 3.14).abs() < 0.001);
/// ```
pub fn decode_f32_be(bytes: &[u8]) -> f32 {
    f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// 从小端序字节数组解码为 f32 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 4 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 f32 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_f32_le;
///
/// let bytes = encode_f32_le(3.14);
/// let value = decode_f32_le(&bytes);
/// assert!((value - 3.14).abs() < 0.001);
/// ```
pub fn decode_f32_le(bytes: &[u8]) -> f32 {
    f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// 将 f64 类型编码为大端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 f64 值
///
/// # 返回值
///
/// 返回包含 8 个字节的数组（大端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_f64_be;
///
/// let bytes = encode_f64_be(3.14159265);
/// ```
pub fn encode_f64_be(value: f64) -> [u8; 8] {
    value.to_be_bytes()
}

/// 将 f64 类型编码为小端序字节数组
///
/// # 参数
///
/// * `value` - 要编码的 f64 值
///
/// # 返回值
///
/// 返回包含 8 个字节的数组（小端序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::encode_f64_le;
///
/// let bytes = encode_f64_le(3.14159265);
/// ```
pub fn encode_f64_le(value: f64) -> [u8; 8] {
    value.to_le_bytes()
}

/// 从大端序字节数组解码为 f64 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 8 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 f64 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_f64_be;
///
/// let bytes = encode_f64_be(3.14159265);
/// let value = decode_f64_be(&bytes);
/// assert!((value - 3.14159265).abs() < 0.000001);
/// ```
pub fn decode_f64_be(bytes: &[u8]) -> f64 {
    f64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

/// 从小端序字节数组解码为 f64 类型
///
/// # 参数
///
/// * `bytes` - 包含至少 8 个字节的数组
///
/// # 返回值
///
/// 返回解码后的 f64 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::decode_f64_le;
///
/// let bytes = encode_f64_le(3.14159265);
/// let value = decode_f64_le(&bytes);
/// assert!((value - 3.14159265).abs() < 0.000001);
/// ```
pub fn decode_f64_le(bytes: &[u8]) -> f64 {
    f64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
}

/// 将任意类型转换为字节数组
///
/// 此函数使用 Rust 的内存布局将任意类型转换为字节数组。
/// 注意：此函数不处理字节序，只是简单地复制内存。
///
/// # 类型参数
///
/// * `T` - 要转换的类型，必须满足 `Copy` trait
///
/// # 参数
///
/// * `value` - 要转换的值
///
/// # 返回值
///
/// 返回包含类型内存表示的字节数组
///
/// # 示例
///
/// ```rust
/// use rf_encoding::to_bytes;
///
/// let value: u32 = 123456;
/// let bytes = to_bytes(value);
/// ```
pub fn to_bytes<T: Copy>(value: T) -> Vec<u8> {
    unsafe {
        let ptr = &value as *const T as *const u8;
        let len = mem::size_of::<T>();
        Vec::from_raw_parts(ptr as *mut u8, len, len)
    }
}
