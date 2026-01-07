//! # pool 模块
//!
//! pool 模块 - 对象池容器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 对象池容器
//!
//! 提供对象池管理器 (`ObjectPoolManager`) 和对象池 (`ObjectPool`)。
//!
//! # 警告
//!
//! **此模块的 `ObjectPool` 是一个占位符实现，不提供真正的对象池功能。**
//!
//! 如果需要生产环境的对象池功能，建议使用：
//! - [`deadpool`](https://crates.io/crates/deadpool) - 数据库连接池
//! - [`object-pool`](https://crates.io/crates/object-pool) - 通用对象池
//! - [`r2d2`](https://crates.io/crates/r2d2) - 资源池
//!
//! 此占位符总是返回 `T::default()`，不会维护或重用对象池中的对象。

use std::marker::PhantomData;

/// 通用对象池管理器
///
/// 这是一个占位符实现，仅用作类型标记，不提供实际的对象池管理功能。
///
/// # 类型参数
///
/// * `T`: 对象类型，必须实现 `Default`、`Send`、`Sync`
pub struct ObjectPoolManager<T> {
    _phantom: PhantomData<T>,
}

impl<T: Default + Send + Sync + 'static> ObjectPoolManager<T> {
    /// 创建一个新的对象池管理器
    ///
    /// # 返回值
    ///
    /// 返回一个 `ObjectPoolManager` 实例
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T: Default + Send + Sync + 'static> Default for ObjectPoolManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 通用对象池
///
/// # 警告：占位符实现
///
/// **这是一个占位符实现，不提供真正的对象池功能。**
///
/// 对于生产环境，强烈建议使用：
/// - [`deadpool`](https://crates.io/crates/deadpool) - 功能完善的连接池
/// - [`object-pool`](https://crates.io/crates/object-pool) - 通用对象池
/// - [`r2d2`](https://crates.io/crates/r2d2) - 资源池
///
/// # 占位符行为
///
/// - `new()` 方法：`max_size` 参数被忽略
/// - `get()` 方法：总是返回 `T::default()`，不维护对象池
/// - 不缓存或重用任何对象
///
/// # 示例
///
/// ```ignore
/// # use rf_container::pool::ObjectPool;
/// #
/// # let pool = ObjectPool::<String>::new(10);
/// # let obj = pool.get().await; // 返回 String::default()，而不是从池中获取
/// ```
///
/// # 为什么不使用此实现
///
/// 1. **性能**：每次都创建新对象，没有池化优势
/// 2. **资源浪费**：不限制对象创建数量
/// 3. **无状态管理**：不跟踪或重用对象
/// 
/// # Deprecated
/// 
/// **此类型已弃用。** 这是一个占位符实现，不提供真正的对象池功能。
/// 
/// 对于生产环境，请使用：
/// - [`deadpool`](https://crates.io/crates/deadpool) - 异步连接池
/// - [`object-pool`](https://crates.io/crates/object-pool) - 通用对象池
/// 
/// 此类型仅保留用于向后兼容，将在未来版本中移除。
#[deprecated(note = "这是一个占位符。请使用 deadpool 或 object-pool 进行生产环境开发。将在未来版本中移除。")]
pub struct ObjectPool<T> {
    #[allow(dead_code, deprecated)]
    _phantom: PhantomData<T>,
}

#[allow(deprecated)]
#[allow(clippy::deprecated)]
impl<T: Default + Send + Sync + 'static> ObjectPool<T> {
    /// 创建一个新的对象池
    ///
    /// # 警告
    ///
    /// `max_size` 参数在此占位符实现中被忽略。
    ///
    /// # 参数
    ///
    /// * `_max_size`: 对象池的最大容量（被忽略）
    ///
    /// # 返回值
    ///
    /// 返回一个 `ObjectPool` 实例
    pub fn new(_max_size: usize) -> Self {
        Self {
            #[allow(deprecated)]
            _phantom: PhantomData,
        }
    }

    /// 从对象池中获取一个对象
    ///
    /// # 警告
    ///
    /// 这是一个占位符方法，总是返回 `T::default()`。
    /// 它不会从池中获取或重用对象。
    ///
    /// 对于真正的对象池功能，请使用：
    /// - [`deadpool`](https://crates.io/crates/deadpool)
    /// - [`object-pool`](https://crates.io/crates/object-pool)
    ///
    /// # 返回值
    ///
    /// 返回 `T::default()`
    pub async fn get(&self) -> T {
        T::default()
    }
}

#[allow(deprecated)]
impl<T: Default + Send + Sync + 'static> Default for ObjectPool<T> {
    fn default() -> Self {
        Self::new(10)
    }
}
