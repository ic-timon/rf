//! # hash
//!
//! hash 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # 哈希算法模块
//!
//! 提供多种哈希算法，用于数据完整性校验、去重和快速查找。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{xxhash, hash};
//!
//! // 计算 XXHash 哈希值
//! let data = b"Hello, World!";
//! let hash_value = xxhash(data);
//! println!("XXHash: {}", hash_value);
//!
//! // 计算任意类型的哈希值
//! let value = "some data";
//! let hash_value = hash(&value);
//! println!("Hash: {}", hash_value);
//! ```
//!
//! ## 哈希算法说明
//!
//! ### XXHash
//! - 极快的非加密哈希算法
//! - 适合数据完整性校验和去重
//! - 哈希值长度：64 位
//! - 速度非常快，接近内存限制
//! - 不适合加密用途
//!
//! ### 通用 Hash
//! - 使用 Rust 标准库的默认哈希器
//! - 适用于 HashMap 和 HashSet
//! - 哈希值长度：64 位
//! - 算法可能与平台相关

use std::hash::{Hash, Hasher};

/// 使用 XXHash 算法计算数据的哈希值
///
/// XXHash 是一种极快的非加密哈希算法，适用于数据完整性校验和去重。
///
/// # 参数
///
/// * `data` - 要计算哈希值的字节数据
///
/// # 返回值
///
/// 返回 64 位哈希值
///
/// # 性能
///
/// XXHash 是目前最快的非加密哈希算法之一，速度接近内存限制。
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xxhash;
///
/// let data = b"Hello, World!";
/// let hash_value = xxhash(data);
/// println!("XXHash: {}", hash_value);
/// ```
pub fn xxhash(data: &[u8]) -> u64 {
    use twox_hash::XxHash64;
    let mut hasher = XxHash64::default();
    hasher.write(data);
    hasher.finish()
}

/// 使用 Rust 默认哈希器计算任意类型的哈希值
///
/// 此函数使用 Rust 标准库的默认哈希器（默认是 SipHash）计算哈希值。
///
/// # 类型参数
///
/// * `T` - 要计算哈希值的类型，必须实现了 `Hash` trait
///
/// # 参数
///
/// * `value` - 要计算哈希值的值引用
///
/// # 返回值
///
/// 返回 64 位哈希值
///
/// # 注意
///
/// - 默认哈希算法可能与平台相关
/// - 相同的数据在同一运行中总是产生相同的哈希值
/// - 适用于 HashMap 和 HashSet
///
/// # 示例
///
/// ```rust
/// use rf_encoding::hash;
///
/// let value = "some data";
/// let hash_value = hash(&value);
/// println!("Hash: {}", hash_value);
/// ```
pub fn hash<T: Hash + ?Sized>(value: &T) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

/// 计算字符串的哈希值（便捷函数）
///
/// # 参数
///
/// * `s` - 要计算哈希值的字符串
///
/// # 返回值
///
/// 返回 64 位哈希值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::hash_string;
///
/// let hash_value = hash_string("hello");
/// println!("Hash: {}", hash_value);
/// ```
pub fn hash_string(s: &str) -> u64 {
    hash(s)
}

/// 计算多个字节数据的组合哈希值
///
/// 将多个数据块的哈希值组合成一个哈希值。
///
/// # 参数
///
/// * `data_slices` - 多个字节数据切片
///
/// # 返回值
///
/// 返回 64 位组合哈希值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xxhash_combine;
///
/// let part1 = b"Hello, ";
/// let part2 = b"World!";
/// let hash_value = xxhash_combine(&[part1, part2]);
/// println!("Combined Hash: {}", hash_value);
/// ```
pub fn xxhash_combine(data_slices: &[&[u8]]) -> u64 {
    use twox_hash::XxHash64;
    let mut hasher = XxHash64::default();
    for slice in data_slices {
        hasher.write(slice);
    }
    hasher.finish()
}

/// 哈希值结构体
///
/// 用于表示和操作哈希值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HashValue {
    /// 64 位哈希值
    pub value: u64,
}

impl HashValue {
    /// 创建新的哈希值
    ///
    /// # 参数
    ///
    /// * `value` - 64 位哈希值
    ///
    /// # 返回值
    ///
    /// 返回哈希值结构体
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// 使用 XXHash 计算哈希值
    ///
    /// # 参数
    ///
    /// * `data` - 要计算哈希值的字节数据
    ///
    /// # 返回值
    ///
    /// 返回哈希值结构体
    pub fn xxhash(data: &[u8]) -> Self {
        Self::new(xxhash(data))
    }

    /// 使用默认哈希器计算哈希值
    ///
    /// # 类型参数
    ///
    /// * `T` - 要计算哈希值的类型，必须实现了 `Hash` trait
    ///
    /// # 参数
    ///
    /// * `value` - 要计算哈希值的值引用
    ///
    /// # 返回值
    ///
    /// 返回哈希值结构体
    pub fn hash<T: Hash + ?Sized>(value: &T) -> Self {
        Self::new(hash(value))
    }

    /// 获取哈希值的十六进制字符串表示
    ///
    /// # 返回值
    ///
    /// 返回哈希值的十六进制字符串
    pub fn to_hex(self) -> String {
        format!("{:016x}", self.value)
    }

    /// 从十六进制字符串解析哈希值
    ///
    /// # 参数
    ///
    /// * `hex` - 十六进制字符串
    ///
    /// # 返回值
    ///
    /// 返回解析后的哈希值结构体，解析失败时返回 None
    pub fn from_hex(hex: &str) -> Option<Self> {
        u64::from_str_radix(hex, 16).ok().map(Self::new)
    }
}
