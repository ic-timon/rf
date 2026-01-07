//! # type 模块
//!
//! type 模块 - 原子类型包装器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 原子类型包装器
//!
//! 提供线程安全的基本数据类型包装器，使用原子操作和读写锁实现并发安全。
//! 所有类型都实现了 `Clone`，可以在线程之间安全共享。
//!
//! # 可用类型
//!
//! - [`Bool`]：线程安全的布尔值
//! - [`Int`]：线程安全的 i32 整数
//! - [`Int64`]：线程安全的 i64 整数
//! - [`Uint`]：线程安全的 u32 无符号整数
//! - [`Uint64`]：线程安全的 u64 无符号整数
//! - [`String`]：线程安全的字符串
//!
//! # 示例
//!
//! ```
//! use rf_container::r#type::{Bool, Int};
//! use rf_container::r#type::String as ThreadSafeString;
//! use std::sync::Arc;
//! use std::thread;
//!
//! // 创建原子整数
//! let counter = Arc::new(Int::new(0));
//! let counter_clone = Arc::clone(&counter);
//!
//! thread::spawn(move || {
//!     counter_clone.add(10);
//! }).join().unwrap();
//!
//! assert_eq!(counter.get(), 10);
//!
//! // 创建原子字符串
//! let text = ThreadSafeString::new("Hello".to_string());
//! text.set("World".to_string());
//! assert_eq!(text.get(), "World");
//! ```

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU32, AtomicU64};
use std::sync::Arc;

/// 线程安全的布尔值包装器
///
/// 使用 `AtomicBool` 实现的线程安全布尔类型，内部使用 `Arc` 实现引用计数共享。
///
/// # 字段
///
/// - `0`: 内部的 `Arc<AtomicBool>`，提供原子操作能力
///
/// # 示例
///
/// ```
/// use rf_container::r#type::Bool;
///
/// let flag = Bool::new(false);
/// println!("Initial: {}", flag.get()); // 输出: false
///
/// flag.set(true);
/// println!("After set: {}", flag.get()); // 输出: true
/// ```
#[derive(Debug, Clone)]
pub struct Bool(Arc<AtomicBool>);

impl Bool {
    /// 创建一个新的布尔值包装器
    ///
    /// # 参数
    ///
    /// * `value`: 初始布尔值
    ///
    /// # 返回值
    ///
    /// 返回一个 `Bool` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Bool;
    ///
    /// let flag = Bool::new(true);
    /// ```
    pub fn new(value: bool) -> Self {
        Self(Arc::new(AtomicBool::new(value)))
    }

    /// 获取当前布尔值
    ///
    /// # 返回值
    ///
    /// 返回当前的布尔值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Bool;
    ///
    /// let flag = Bool::new(true);
    /// assert_eq!(flag.get(), true);
    /// ```
    pub fn get(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 设置新的布尔值
    ///
    /// # 参数
    ///
    /// * `value`: 要设置的新布尔值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Bool;
    ///
    /// let flag = Bool::new(false);
    /// flag.set(true);
    /// assert_eq!(flag.get(), true);
    /// ```
    pub fn set(&self, value: bool) {
        self.0.store(value, std::sync::atomic::Ordering::Relaxed);
    }
}

/// 线程安全的 i32 整数包装器
///
/// 使用 `AtomicI32` 实现的线程安全 32 位有符号整数类型。
/// 支持原子加法操作，适合用作计数器。
///
/// # 字段
///
/// - `0`: 内部的 `Arc<AtomicI32>`，提供原子操作能力
///
/// # 示例
///
/// ```
/// use rf_container::r#type::Int;
///
/// let counter = Int::new(0);
/// counter.add(5);
/// println!("Counter: {}", counter.get()); // 输出: 5
/// ```
#[derive(Debug, Clone)]
pub struct Int(Arc<AtomicI32>);

impl Int {
    /// 创建一个新的 i32 整数包装器
    ///
    /// # 参数
    ///
    /// * `value`: 初始 i32 值
    ///
    /// # 返回值
    ///
    /// 返回一个 `Int` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Int;
    ///
    /// let num = Int::new(42);
    /// ```
    pub fn new(value: i32) -> Self {
        Self(Arc::new(AtomicI32::new(value)))
    }

    /// 获取当前整数值
    ///
    /// # 返回值
    ///
    /// 返回当前的 i32 值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Int;
    ///
    /// let num = Int::new(42);
    /// assert_eq!(num.get(), 42);
    /// ```
    pub fn get(&self) -> i32 {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 设置新的整数值
    ///
    /// # 参数
    ///
    /// * `value`: 要设置的新 i32 值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Int;
    ///
    /// let num = Int::new(42);
    /// num.set(100);
    /// assert_eq!(num.get(), 100);
    /// ```
    pub fn set(&self, value: i32) {
        self.0.store(value, std::sync::atomic::Ordering::Relaxed);
    }

    /// 原子地增加整数值并返回新值
    ///
    /// # 参数
    ///
    /// * `delta`: 要增加的值（可以是负数）
    ///
    /// # 返回值
    ///
    /// 返回增加后的新值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Int;
    ///
    /// let counter = Int::new(10);
    /// let new_value = counter.add(5);
    /// assert_eq!(new_value, 15);
    /// assert_eq!(counter.get(), 15);
    /// ```
    pub fn add(&self, delta: i32) -> i32 {
        self.0.fetch_add(delta, std::sync::atomic::Ordering::Relaxed) + delta
    }
}

/// 线程安全的 i64 整数包装器
///
/// 使用 `AtomicI64` 实现的线程安全 64 位有符号整数类型。
/// 适用于需要更大范围的计数场景。
///
/// # 字段
///
/// - `0`: 内部的 `Arc<AtomicI64>`，提供原子操作能力
///
/// # 示例
///
/// ```
/// use rf_container::r#type::Int64;
///
/// let counter = Int64::new(0);
/// counter.add(1_000_000);
/// println!("Counter: {}", counter.get()); // 输出: 1000000
/// ```
#[derive(Debug, Clone)]
pub struct Int64(Arc<AtomicI64>);

impl Int64 {
    /// 创建一个新的 i64 整数包装器
    ///
    /// # 参数
    ///
    /// * `value`: 初始 i64 值
    ///
    /// # 返回值
    ///
    /// 返回一个 `Int64` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Int64;
    ///
    /// let num = Int64::new(1_000_000);
    /// ```
    pub fn new(value: i64) -> Self {
        Self(Arc::new(AtomicI64::new(value)))
    }

    /// 获取当前整数值
    ///
    /// # 返回值
    ///
    /// 返回当前的 i64 值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Int64;
    ///
    /// let num = Int64::new(1_000_000);
    /// assert_eq!(num.get(), 1_000_000);
    /// ```
    pub fn get(&self) -> i64 {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 设置新的整数值
    ///
    /// # 参数
    ///
    /// * `value`: 要设置的新 i64 值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Int64;
    ///
    /// let num = Int64::new(1_000_000);
    /// num.set(2_000_000);
    /// assert_eq!(num.get(), 2_000_000);
    /// ```
    pub fn set(&self, value: i64) {
        self.0.store(value, std::sync::atomic::Ordering::Relaxed);
    }

    /// 原子地增加整数值并返回新值
    ///
    /// # 参数
    ///
    /// * `delta`: 要增加的值（可以是负数）
    ///
    /// # 返回值
    ///
    /// 返回增加后的新值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Int64;
    ///
    /// let counter = Int64::new(1_000_000);
    /// let new_value = counter.add(500_000);
    /// assert_eq!(new_value, 1_500_000);
    /// ```
    pub fn add(&self, delta: i64) -> i64 {
        self.0.fetch_add(delta, std::sync::atomic::Ordering::Relaxed) + delta
    }
}

/// 线程安全的 u32 无符号整数包装器
///
/// 使用 `AtomicU32` 实现的线程安全 32 位无符号整数类型。
///
/// # 字段
///
/// - `0`: 内部的 `Arc<AtomicU32>`，提供原子操作能力
///
/// # 示例
///
/// ```
/// use rf_container::r#type::Uint;
///
/// let counter = Uint::new(0);
/// counter.add(10);
/// println!("Counter: {}", counter.get()); // 输出: 10
/// ```
#[derive(Debug, Clone)]
pub struct Uint(Arc<AtomicU32>);

impl Uint {
    /// 创建一个新的 u32 无符号整数包装器
    ///
    /// # 参数
    ///
    /// * `value`: 初始 u32 值
    ///
    /// # 返回值
    ///
    /// 返回一个 `Uint` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Uint;
    ///
    /// let num = Uint::new(100);
    /// ```
    pub fn new(value: u32) -> Self {
        Self(Arc::new(AtomicU32::new(value)))
    }

    /// 获取当前整数值
    ///
    /// # 返回值
    ///
    /// 返回当前的 u32 值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Uint;
    ///
    /// let num = Uint::new(100);
    /// assert_eq!(num.get(), 100);
    /// ```
    pub fn get(&self) -> u32 {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 设置新的整数值
    ///
    /// # 参数
    ///
    /// * `value`: 要设置的新 u32 值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Uint;
    ///
    /// let num = Uint::new(100);
    /// num.set(200);
    /// assert_eq!(num.get(), 200);
    /// ```
    pub fn set(&self, value: u32) {
        self.0.store(value, std::sync::atomic::Ordering::Relaxed);
    }

    /// 原子地增加整数值并返回新值
    ///
    /// # 参数
    ///
    /// * `delta`: 要增加的值
    ///
    /// # 返回值
    ///
    /// 返回增加后的新值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Uint;
    ///
    /// let counter = Uint::new(100);
    /// let new_value = counter.add(50);
    /// assert_eq!(new_value, 150);
    /// ```
    pub fn add(&self, delta: u32) -> u32 {
        self.0.fetch_add(delta, std::sync::atomic::Ordering::Relaxed) + delta
    }
}

/// 线程安全的 u64 无符号整数包装器
///
/// 使用 `AtomicU64` 实现的线程安全 64 位无符号整数类型。
///
/// # 字段
///
/// - `0`: 内部的 `Arc<AtomicU64>`，提供原子操作能力
///
/// # 示例
///
/// ```
/// use rf_container::r#type::Uint64;
///
/// let counter = Uint64::new(0);
/// counter.add(1_000_000);
/// println!("Counter: {}", counter.get()); // 输出: 1000000
/// ```
#[derive(Debug, Clone)]
pub struct Uint64(Arc<AtomicU64>);

impl Uint64 {
    /// 创建一个新的 u64 无符号整数包装器
    ///
    /// # 参数
    ///
    /// * `value`: 初始 u64 值
    ///
    /// # 返回值
    ///
    /// 返回一个 `Uint64` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Uint64;
    ///
    /// let num = Uint64::new(1_000_000);
    /// ```
    pub fn new(value: u64) -> Self {
        Self(Arc::new(AtomicU64::new(value)))
    }

    /// 获取当前整数值
    ///
    /// # 返回值
    ///
    /// 返回当前的 u64 值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Uint64;
    ///
    /// let num = Uint64::new(1_000_000);
    /// assert_eq!(num.get(), 1_000_000);
    /// ```
    pub fn get(&self) -> u64 {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// 设置新的整数值
    ///
    /// # 参数
    ///
    /// * `value`: 要设置的新 u64 值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Uint64;
    ///
    /// let num = Uint64::new(1_000_000);
    /// num.set(2_000_000);
    /// assert_eq!(num.get(), 2_000_000);
    /// ```
    pub fn set(&self, value: u64) {
        self.0.store(value, std::sync::atomic::Ordering::Relaxed);
    }

    /// 原子地增加整数值并返回新值
    ///
    /// # 参数
    ///
    /// * `delta`: 要增加的值
    ///
    /// # 返回值
    ///
    /// 返回增加后的新值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::Uint64;
    ///
    /// let counter = Uint64::new(1_000_000);
    /// let new_value = counter.add(500_000);
    /// assert_eq!(new_value, 1_500_000);
    /// ```
    pub fn add(&self, delta: u64) -> u64 {
        self.0.fetch_add(delta, std::sync::atomic::Ordering::Relaxed) + delta
    }
}

/// 线程安全的字符串包装器
///
/// 使用 `RwLock` 实现的线程安全字符串类型，内部使用 `Arc` 实现引用计数共享。
/// 适用于多线程环境下需要共享和修改字符串的场景。
///
/// # 字段
///
/// - `0`: 内部的 `Arc<RwLock<String>>`，提供读写锁能力
///
/// # 示例
///
/// ```
/// use rf_container::r#type::String;
///
/// let text = String::new("Hello".to_string());
/// println!("{}", text.get()); // 输出: Hello
///
/// text.set("World".to_string());
/// println!("{}", text.get()); // 输出: World
/// ```
#[derive(Debug, Clone)]
pub struct String(Arc<parking_lot::RwLock<std::string::String>>);

impl String {
    /// 创建一个新的字符串包装器
    ///
    /// # 参数
    ///
    /// * `value`: 初始字符串值
    ///
    /// # 返回值
    ///
    /// 返回一个 `String` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::String;
    ///
    /// let text = String::new("Hello, World!".to_string());
    /// ```
    pub fn new(value: std::string::String) -> Self {
        Self(Arc::new(parking_lot::RwLock::new(value)))
    }

    /// 获取当前字符串值的克隆
    ///
    /// # 返回值
    ///
    /// 返回当前字符串的副本
    ///
    /// # 注意
    ///
    /// 此方法会克隆整个字符串，对于大字符串可能有性能影响。
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::String;
    ///
    /// let text = String::new("Hello".to_string());
    /// let content = text.get();
    /// assert_eq!(content, "Hello");
    /// ```
    pub fn get(&self) -> std::string::String {
        self.0.read().clone()
    }

    /// 设置新的字符串值
    ///
    /// # 参数
    ///
    /// * `value`: 要设置的新字符串值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::r#type::String;
    ///
    /// let text = String::new("Hello".to_string());
    /// text.set("World".to_string());
    /// assert_eq!(text.get(), "World");
    /// ```
    pub fn set(&self, value: std::string::String) {
        *self.0.write() = value;
    }
}
