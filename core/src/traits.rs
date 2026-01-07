//! # traits
//!
//! traits 模块 - RF 框架的核心 trait 定义
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF 框架的核心 trait 定义
//!
//! 该模块定义了 RF 框架中使用的基础 trait，这些 trait 为框架中的类型
//! 提供了统一的行为接口，包括字符串转换、克隆、比较和哈希功能。
//!
//! # Trait 列表
//!
//! - [`ToString`]: 将对象转换为字符串表示
//! - [`Clone`]: 标记 trait，表示对象可以被克隆
//! - [`Compare`]: 标记 trait，表示对象可以进行比较
//! - [`Hash`]: 标记 trait，表示对象可以被哈希
//!
//! # 设计理念
//!
//! 这些 trait 大多是对标准库 trait 的重新导出或标记，目的是：
//! 1. 提供统一的命名空间
//! 2. 简化导入路径
//! 3. 为框架提供一致的行为接口
//!
//! # 使用示例
//!
//! ```rust
//! use rf_core::traits::ToString;
//!
//! // 为自定义类型实现 ToString trait
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! impl ToString for Person {
//!     fn to_string(&self) -> String {
//!         format!("{} ({}岁)", self.name, self.age)
//!     }
//! }
//!
//! let person = Person {
//!     name: "张三".to_string(),
//!     age: 25,
//! };
//!
//! println!("{}", person.to_string()); // 输出: 张三 (25岁)
//! ```

//! // ToString trait - 可转换为字符串的对象
//! //
//! // 该 trait 定义了将对象转换为字符串表示的能力。
//! // 与标准库的 `std::string::ToString` 不同，这是框架自定义的版本，
//! // 允许为类型提供更灵活的字符串转换实现。
//! //
//! // # 必需方法
//! //
//! // - [`to_string`](#tymethod.to_string): 将对象转换为字符串
//! //
//! // # 使用场景
//! //
//! // - 日志输出
//! // - 调试信息显示
//! // - 数据序列化
//! // - 用户界面显示
//! //
//! // # 实现示例
//! //
//! // ## 基本实现
//! //
//! // ```rust
//! // use rf_core::traits::ToString;
//! //
//! // struct Point {
//! //     x: i32,
//! //     y: i32,
//! // }
//! //
//! // impl ToString for Point {
//! //     fn to_string(&self) -> String {
//! //         format!("({}, {})", self.x, self.y)
//! //     }
//! // }
//! //
//! // let point = Point { x: 10, y: 20 };
//! // assert_eq!(point.to_string(), "(10, 20)");
//! // ```
//! //
//! // ## 为枚举实现
//! //
//! // ```rust
//! // use rf_core::traits::ToString;
//! //
//! // enum Status {
//! //     Active,
//! //     Inactive,
//! //     Pending,
//! // }
//! //
//! // impl ToString for Status {
//! //     fn to_string(&self) -> String {
//! //         match self {
//! //             Status::Active => "激活".to_string(),
//! //             Status::Inactive => "未激活".to_string(),
//! //             Status::Pending => "待处理".to_string(),
//! //         }
//! //     }
//! // }
//! //
//! // let status = Status::Active;
//! // assert_eq!(status.to_string(), "激活");
//! // ```
//! //
//! // # 注意事项
//! //
//! // - 该 trait 与标准库的 `std::string::ToString` 是独立的
//! // - 实现时应考虑字符串的可读性和格式一致性
//! // - 对于复杂对象，建议包含关键的标识信息
pub trait ToString {
    /// 将对象转换为字符串表示
    ///
    /// 该方法应该返回对象的字符串表示形式。字符串应该：
    /// - 清晰地描述对象的状态
    /// - 适合在日志或用户界面中显示
    /// - 遵循一致的格式约定
    ///
    /// # 返回值
    ///
    /// 返回对象的字符串表示形式。
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_core::traits::ToString;
    ///
    /// struct User {
    ///     id: u32,
    ///     username: String,
    /// }
    ///
    /// impl ToString for User {
    ///     fn to_string(&self) -> String {
    ///         format!("User[{}]@{}", self.id, self.username)
    ///     }
    /// }
    ///
    /// let user = User {
    ///     id: 123,
    ///     username: "alice".to_string(),
    /// };
    ///
    /// assert_eq!(user.to_string(), "User[123]@alice");
    /// ```
    fn to_string(&self) -> String;
}

/// Clone trait - 可克隆对象的标记 trait
///
/// 这是一个标记 trait，继承自标准库的 `std::clone::Clone`。
/// 它表示实现了该 trait 的类型可以通过克隆创建自身的深拷贝。
///
/// # 特性
///
/// - 这是一个标记 trait，没有额外的方法
/// - 继承自 `std::clone::Clone`
/// - 可以通过标准库的 `clone()` 方法创建副本
///
/// # 使用场景
///
/// - 需要保留对象副本时
/// - 在多所有权场景下使用
/// - 避免借用检查器限制
///
/// # 实现示例
///
/// ```rust
/// use rf_core::traits::Clone;
///
/// #[derive(Clone)]
/// struct Config {
///     host: String,
///     port: u16,
/// }
///
/// impl Clone for Config {} // 自动实现，通常使用 #[derive(Clone)]
///
/// let config1 = Config {
///     host: "localhost".to_string(),
///     port: 8080,
/// };
///
/// let config2 = config1.clone(); // 创建深拷贝
/// ```
///
/// # 注意事项
///
/// - 大多数情况下应使用 `#[derive(Clone)]` 自动派生
/// - 包含资源的类型需要手动实现克隆逻辑
/// - 克隆操作可能涉及深拷贝，性能开销需要考虑
pub trait Clone: std::clone::Clone {}

/// Compare trait - 可比较对象的标记 trait
///
/// 这是一个标记 trait，组合了标准库的 `PartialEq` 和 `Eq` trait。
/// 它表示实现了该 trait 的类型可以进行相等性比较。
///
/// # 特性
///
/// - 继承自 `PartialEq` 和 `Eq`
/// - 支持 `==` 和 `!=` 操作符
/// - 表示值的相等性比较（不仅是部分相等）
///
/// # 使用场景
///
/// - 需要比较两个对象是否相等
/// - 在集合中查找元素
/// - 去重操作
/// - 测试和验证
///
/// # 实现示例
///
/// ```rust
/// use rf_core::traits::Compare;
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct UserId(u32);
///
/// impl Compare for UserId {} // 标记 trait，自动实现
///
/// let id1 = UserId(123);
/// let id2 = UserId(123);
/// let id3 = UserId(456);
///
/// assert_eq!(id1, id2); // 相等
/// assert_ne!(id1, id3); // 不相等
/// ```
///
/// # PartialEq vs Eq
///
/// - `PartialEq`: 部分相等，可能存在无法比较的情况（如浮点数的 NaN）
/// - `Eq`: 完全相等，是 `PartialEq` 的子 trait，要求相等关系是等价关系
///   - 自反性: `x == x`
///   - 对称性: `x == y` 意味着 `y == x`
///   - 传递性: `x == y` 且 `y == z` 意味着 `x == z`
///
/// # 注意事项
///
/// - 实现时应确保相等关系的语义合理
/// - 浮点数类型通常只实现 `PartialEq` 而不实现 `Eq`
/// - 包含浮点数的结构体也要注意这个问题
pub trait Compare: PartialEq + Eq {}

/// Hash trait - 可哈希对象的标记 trait
///
/// 这是一个标记 trait，继承自标准库的 `std::hash::Hash`。
/// 它表示实现了该 trait 的类型可以被哈希，并可用作哈希表（如 HashMap）的键。
///
/// # 特性
///
/// - 继承自 `std::hash::Hash`
/// - 可以计算对象的哈希值
/// - 通常需要与 `Eq` trait 一起实现
///
/// # 使用场景
///
/// - 作为 HashMap 或 HashSet 的键
/// - 缓存实现
/// - 数据去重
/// - 快速查找数据结构
///
/// # 实现示例
///
/// ```rust
/// use std::collections::HashMap;
/// use std::hash::{Hash, Hasher};
///
/// #[derive(PartialEq, Eq)]
/// struct Key {
///     namespace: String,
///     id: u32,
/// }
///
/// impl Hash for Key {
///     fn hash<H: Hasher>(&self, state: &mut H) {
///         self.namespace.hash(state);
///         self.id.hash(state);
///     }
/// }
///
/// // 可以用作 HashMap 的键
/// let mut map: HashMap<Key, String> = HashMap::new();
/// let key = Key {
///     namespace: "user".to_string(),
///     id: 123,
/// };
/// map.insert(key, "数据".to_string());
/// ```
///
/// # Hash + Eq 关系
///
/// 正确的实现应该满足：如果两个对象相等（根据 `Eq`），它们的哈希值必须相同。
///
/// ```rust
/// use std::hash::Hash;
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::Hasher;
///
/// fn get_hash<T: Hash>(item: &T) -> u64 {
///     let mut hasher = DefaultHasher::new();
///     item.hash(&mut hasher);
///     hasher.finish()
/// }
///
/// // 如果 a == b，则 get_hash(&a) == get_hash(&b)
/// ```
///
/// # 注意事项
///
/// - 通常应该使用 `#[derive(Hash)]` 自动派生
/// - 实现 `Hash` 时通常也应该实现 `Eq`
/// - 哈希函数应该尽可能均匀分布，避免冲突
/// - 不要在哈希计算中包含可变字段
pub trait Hash: std::hash::Hash {}

