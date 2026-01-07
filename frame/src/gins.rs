//! # gins
//!
//! gins 模块 - 框架实例管理器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Framework instance management
//!
//! 提供全局单例实例管理功能，用于创建、获取和复用框架中的各种服务实例。
//!
//! ## 核心功能
//!
//! - **单例模式**: 确保每种类型的实例在应用中只创建一次
//! - **按名称管理**: 支持为同类型实例创建多个命名实例（如多个数据库连接）
//! - **配置驱动**: 从配置文件自动加载实例参数
//! - **线程安全**: 使用 Mutex 和 Arc 确保多线程环境下的安全访问
//! - **异步友好**: 支持异步初始化的实例
//!
//! ## 管理的实例类型
//!
//! - `server()`: HTTP 服务器实例
//! - `database()`: 数据库连接实例
//! - `redis()`: Redis 客户端实例
//! - `view()`: 视图引擎实例
//! - `config()`: 配置管理实例
//! - `i18n()`: 国际化实例
//! - `resource()`: 资源存储实例
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_frame::gins;
//!
//! // 获取默认数据库实例
//! let db = gins::database(None).await?;
//!
//! // 获取命名数据库实例
//! let cache_db = gins::database(Some("cache")).await?;
//!
//! // 获取默认配置实例
//! let config = gins::config(None);
//!
//! // 获取命名配置实例
//! let app_config = gins::config(Some("app"));
//! ```
//!
//! ## 配置文件示例
//!
//! ```toml
//! [server.default]
//! address = "127.0.0.1:8080"
//!
//! [database.default]
//! url = "postgresql://localhost/mydb"
//!
//! [database.cache]
//! url = "postgresql://localhost/cache_db"
//!
//! [redis.default]
//! url = "redis://127.0.0.1:6379/"
//! ```

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rf_errors::Result;

/// 全局实例管理器
///
/// 此结构体管理应用中的所有全局实例，使用单例模式确保实例的唯一性和复用性。
///
/// # 字段
///
/// * `instances` - 存储所有实例的 HashMap，键为实例名称，值为类型擦除的实例对象
///
/// # 实现细节
///
/// - 使用 `Mutex` 保证线程安全
/// - 使用 `Box<dyn Any>` 实现类型擦除，可以存储任意类型的实例
/// - 实例必须满足 `Send + Sync` 以支持多线程访问
pub struct InstanceManager {
    instances: Mutex<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
}

/// 全局实例管理器的懒加载静态实例
///
/// 使用 `once_cell::sync::Lazy` 确保实例管理器只在第一次访问时初始化。
static INSTANCE_MANAGER: Lazy<InstanceManager> = Lazy::new(|| {
    InstanceManager {
        instances: Mutex::new(HashMap::new()),
    }
});

impl InstanceManager {
    /// 获取或创建一个实例（同步版本）
    ///
    /// 此方法首先尝试从缓存中获取指定名称的实例，如果不存在则使用工厂函数创建新实例。
    /// 适用于实现了 `Clone` trait 的类型。
    ///
    /// # 参数
    ///
    /// * `name` - 实例的名称，用于标识不同的实例
    /// * `factory` - 工厂函数，用于创建新实例
    ///
    /// # 泛型参数
    ///
    /// * `F` - 工厂函数类型
    /// * `T` - 实例类型，必须实现 Clone + Send + Sync
    ///
    /// # 返回值
    ///
    /// 返回实例的克隆副本
    ///
    /// # 使用示例
    ///
    /// ```rust
    /// use rf_frame::gins::InstanceManager;
    ///
    /// // 获取或创建实例
    /// let value: i32 = InstanceManager::get_or_create("counter", || 42);
    /// // 第二次调用会返回缓存的值
    /// let value2: i32 = InstanceManager::get_or_create("counter", || 100);
    /// assert_eq!(value, value2);
    /// ```
    pub fn get_or_create<F, T>(name: &str, factory: F) -> T
    where
        F: FnOnce() -> T,
        T: 'static + Clone + Send + Sync,
    {
        let mut instances = INSTANCE_MANAGER.instances.lock()
            .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        if let Some(instance) = instances.get(name) {
            if let Some(typed) = instance.downcast_ref::<T>() {
                return typed.clone();
            }
        }
        let instance = factory();
        instances.insert(name.to_string(), Box::new(instance.clone()));
        instance
    }

    /// 获取或创建一个实例（异步版本）
    ///
    /// 此方法是 `get_or_create` 的异步版本，适用于需要异步初始化的实例。
    /// 返回 `Arc<T>` 以支持非 Clone 类型。
    ///
    /// # 参数
    ///
    /// * `name` - 实例的名称
    /// * `factory` - 异步工厂函数，返回一个 Future
    ///
    /// # 泛型参数
    ///
    /// * `F` - 异步工厂函数类型
    /// * `Fut` - Future 类型
    /// * `T` - 实例类型，必须满足 Send + Sync
    ///
    /// # 返回值
    ///
    /// 返回实例的 Arc 智能指针
    ///
    /// # 注意事项
    ///
    /// - 在调用 `factory().await` 之前释放锁，避免在持有锁时执行异步操作
    /// - 这可能导致在极少数情况下多次创建实例，但最终只会保留一个
    ///
    /// # 使用示例
    ///
    /// ```rust
    /// use rf_frame::gins::InstanceManager;
    ///
    /// # async fn example() {
    /// // 异步获取或创建数据库连接
    /// let db = InstanceManager::get_or_create_async(
    ///     "db",
    ///     || async {
    ///         // 执行异步初始化...
    ///         "DatabaseConnection"
    ///     }
    /// ).await;
    /// # }
    /// ```
    pub async fn get_or_create_async<F, Fut, T>(name: &str, factory: F) -> Arc<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
        T: 'static + Send + Sync,
    {
        // 第一次检查：只读锁，在 await 之前释放
        {
            let instances = INSTANCE_MANAGER.instances.lock()
                .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
            if let Some(instance) = instances.get(name) {
                if let Some(typed) = instance.downcast_ref::<Arc<T>>() {
                    return Arc::clone(typed);
                }
            }
        };

        // 锁已在此处释放，可以安全地执行 await
        let instance = factory().await;
        let arc_instance = Arc::new(instance);

        // 重新获取锁以插入新实例
        let mut instances = INSTANCE_MANAGER.instances.lock()
            .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        instances.insert(name.to_string(), Box::new(Arc::clone(&arc_instance)));
        arc_instance
    }

    /// 移除指定的实例
    ///
    /// 此方法从管理器中移除指定名称的实例，释放相关资源。
    ///
    /// # 参数
    ///
    /// * `name` - 要移除的实例名称
    ///
    /// # 使用示例
    ///
    /// ```rust
    /// use rf_frame::gins::InstanceManager;
    ///
    /// InstanceManager::remove("temp_instance");
    /// ```
    pub fn remove(name: &str) {
        let mut instances = INSTANCE_MANAGER.instances.lock()
            .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        instances.remove(name);
    }

    /// 清除所有实例
    ///
    /// 此方法移除管理器中的所有实例，通常在应用关闭时调用。
    ///
    /// # 使用示例
    ///
    /// ```rust
    /// use rf_frame::gins::InstanceManager;
    ///
    /// // 应用关闭时清理所有实例
    /// InstanceManager::clear();
    /// ```
    pub fn clear() {
        let mut instances = INSTANCE_MANAGER.instances.lock()
            .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        instances.clear();
    }
}

/// 获取 HTTP 服务器实例（按名称，从配置加载）
///
/// 此方法获取或创建一个命名的 HTTP 服务器实例。
/// 如果实例不存在，会尝试从配置文件中读取服务器地址。
///
/// # 参数
///
/// * `name` - 实例名称，None 表示使用默认名称 "default"
///
/// # 返回值
///
/// 返回 `Result<Arc<HttpServer>>`，成功时包含服务器的 Arc 智能指针
///
/// # 配置项
///
/// 配置文件中的 `server.{name}.address` 字段指定服务器地址
/// - 如果未配置，默认使用 "127.0.0.1:8080"
/// - 地址格式示例：`"0.0.0.0:8080"` 或 `"127.0.0.1:3000"`
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::gins;
///
/// // 获取默认服务器
/// let server = gins::server(None)?;
///
/// // 获取命名服务器
/// let api_server = gins::server(Some("api"))?;
///
/// // 使用服务器...
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn server(name: Option<&str>) -> Result<Arc<rf_net::http::HttpServer>> {
    let instance_name = name.unwrap_or("default");
    let key = format!("server.{}", instance_name);

    // HttpServer 未实现 Clone，因此使用 Arc
    let instances = INSTANCE_MANAGER.instances.lock()
        .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
    if let Some(instance) = instances.get(&key) {
        if let Some(typed) = instance.downcast_ref::<Arc<rf_net::http::HttpServer>>() {
            return Ok(Arc::clone(typed));
        }
    }

    // 尝试从配置加载
    let config = rf_os::cfg::Config::new();
    let addr = if let Ok(Some(addr_str)) = config.get(&format!("server.{}.address", instance_name)) {
        addr_str.parse::<std::net::SocketAddr>().unwrap_or_else(|_| {
            "127.0.0.1:8080".parse().unwrap()
        })
    } else {
        "127.0.0.1:8080".parse().unwrap()
    };

    let server = Arc::new(rf_net::http::HttpServer::new(addr));
    drop(instances);
    let mut instances = INSTANCE_MANAGER.instances.lock().unwrap();
    instances.insert(key, Box::new(Arc::clone(&server)));
    Ok(server)
}

/// 获取数据库实例（按名称，从配置加载）
///
/// 此方法获取或创建一个命名的数据库连接实例。
/// 支持多种数据库类型：PostgreSQL、MySQL、SQLite。
///
/// # 参数
///
/// * `name` - 实例名称，None 表示使用默认名称 "default"
///
/// # 返回值
///
/// 返回 `Result<Arc<Database>>`，成功时包含数据库的 Arc 智能指针
///
/// # 配置项
///
/// 配置文件中的 `database.{name}.url` 字段指定数据库连接 URL
/// - PostgreSQL: `"postgresql://host/database"`
/// - MySQL: `"mysql://host/database"`
/// - SQLite: `"sqlite:///path/to/database.db"`
/// - 如果未配置，默认使用 `"postgresql://localhost/test"`
///
/// # 支持的数据库
///
/// - PostgreSQL (推荐)
/// - MySQL
/// - SQLite
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::gins;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 获取默认数据库
/// let db = gins::database(None).await?;
///
/// // 获取缓存数据库
/// let cache_db = gins::database(Some("cache")).await?;
///
/// // 使用数据库...
/// # Ok(())
/// # }
/// ```
pub async fn database(name: Option<&str>) -> Result<Arc<rf_database::db::Database>> {
    let instance_name = name.unwrap_or("default");
    let key = format!("database.{}", instance_name);

    // 第一次检查：只读锁，在 await 之前释放
    {
        let instances = INSTANCE_MANAGER.instances.lock()
        .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        if let Some(instance) = instances.get(&key) {
            if let Some(typed) = instance.downcast_ref::<Arc<rf_database::db::Database>>() {
                return Ok(Arc::clone(typed));
            }
        }
    } // 锁在此处释放

    // 尝试从配置加载
    let config = rf_os::cfg::Config::new();
    let db = if let Ok(Some(url)) = config.get(&format!("database.{}.url", instance_name)) {
        // 优先尝试 PostgreSQL
        if url.starts_with("postgresql://") {
            rf_database::db::Database::new_postgres(&url).await?
        } else if url.starts_with("mysql://") {
            rf_database::db::Database::new_mysql(&url).await?
        } else if url.starts_with("sqlite://") {
            let sqlite_url = url.trim_start_matches("sqlite://");
            rf_database::db::Database::new_sqlite(sqlite_url).await?
        } else {
            rf_database::db::Database::new_postgres("postgresql://localhost/test").await?
        }
    } else {
        // 默认：PostgreSQL
        rf_database::db::Database::new_postgres("postgresql://localhost/test").await?
    };

    let arc_db = Arc::new(db);
    {
        let mut instances = INSTANCE_MANAGER.instances.lock()
            .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        instances.insert(key, Box::new(Arc::clone(&arc_db)));
    }
    Ok(arc_db)
}

/// 获取 Redis 客户端实例（按名称，从配置加载）
///
/// 此方法获取或创建一个命名的 Redis 客户端连接实例。
///
/// # 参数
///
/// * `name` - 实例名称，None 表示使用默认名称 "default"
///
/// # 返回值
///
/// 返回 `Result<Arc<RedisClient>>`，成功时包含 Redis 客户端的 Arc 智能指针
///
/// # 配置项
///
/// 配置文件中的 `redis.{name}.url` 字段指定 Redis 服务器 URL
/// - 格式: `"redis://host:port/db"`
/// - 如果未配置，默认使用 `"redis://127.0.0.1:6379/"`
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::gins;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // 获取默认 Redis 客户端
/// let redis = gins::redis(None).await?;
///
/// // 获取会话存储 Redis 客户端
/// let session_redis = gins::redis(Some("session")).await?;
///
/// // 使用 Redis...
/// # Ok(())
/// # }
/// ```
pub async fn redis(name: Option<&str>) -> Result<Arc<rf_database::redis::RedisClient>> {
    let instance_name = name.unwrap_or("default");
    let key = format!("redis.{}", instance_name);

    // 第一次检查：只读锁，在 await 之前释放
    {
        let instances = INSTANCE_MANAGER.instances.lock()
        .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        if let Some(instance) = instances.get(&key) {
            if let Some(typed) = instance.downcast_ref::<Arc<rf_database::redis::RedisClient>>() {
                return Ok(Arc::clone(typed));
            }
        }
    } // 锁在此处释放

    // 尝试从配置加载
    let config = rf_os::cfg::Config::new();
    let client = if let Ok(Some(url)) = config.get(&format!("redis.{}.url", instance_name)) {
        rf_database::redis::RedisClient::new(&url).await
            .map_err(|e| rf_errors::RfError::Database(format!("Failed to create Redis client: {}", e)))?
    } else {
        // 默认 URL
        rf_database::redis::RedisClient::new("redis://127.0.0.1:6379/").await
            .map_err(|e| rf_errors::RfError::Database(format!("Failed to create Redis client: {}", e)))?
    };

    let arc_client = Arc::new(client);
    {
        let mut instances = INSTANCE_MANAGER.instances.lock()
            .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        instances.insert(key, Box::new(Arc::clone(&arc_client)));
    }
    Ok(arc_client)
}

/// 获取视图引擎实例（按名称，从配置加载）
///
/// 此方法获取或创建一个命名的视图引擎实例，用于渲染模板文件。
///
/// # 参数
///
/// * `name` - 实例名称，None 表示使用默认名称 "default"
///
/// # 返回值
///
/// 返回 `Result<Arc<View>>`，成功时包含视图引擎的 Arc 智能指针
///
/// # 配置项
///
/// 配置文件中的 `view.{name}.template_dir` 字段指定模板目录路径
/// - 如果未配置，默认使用 `"templates"` 目录
/// - 如果指定的目录无效，会回退到 `"templates"` 目录
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::gins;
///
/// // 获取默认视图引擎
/// let view = gins::view(None)?;
///
/// // 获取邮件模板视图引擎
/// let email_view = gins::view(Some("email"))?;
///
/// // 使用视图引擎渲染模板...
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn view(name: Option<&str>) -> Result<Arc<rf_os::view::View>> {
    let instance_name = name.unwrap_or("default");
    let key = format!("view.{}", instance_name);

    // View 未实现 Clone，因此使用 Arc
    let instances = INSTANCE_MANAGER.instances.lock()
        .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
    if let Some(instance) = instances.get(&key) {
        if let Some(typed) = instance.downcast_ref::<Arc<rf_os::view::View>>() {
            return Ok(Arc::clone(typed));
        }
    }
    drop(instances);

    // 尝试从配置加载
    let config = rf_os::cfg::Config::new();
    let view = if let Ok(Some(template_dir)) = config.get(&format!("view.{}.template_dir", instance_name)) {
        rf_os::view::View::new(&template_dir)
            .unwrap_or_else(|_| {
                rf_os::view::View::new("templates").unwrap()
            })
    } else {
        // 默认模板目录
        rf_os::view::View::new("templates")
            .unwrap_or_else(|_| {
                rf_os::view::View::new("templates").unwrap()
            })
    };

    let arc_view = Arc::new(view);
    {
        let mut instances = INSTANCE_MANAGER.instances.lock()
            .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        instances.insert(key, Box::new(Arc::clone(&arc_view)));
    }
    Ok(arc_view)
}

/// 获取配置实例（按名称）
///
/// 此方法获取或创建一个命名的配置管理实例。
///
/// # 参数
///
/// * `name` - 实例名称，None 表示使用默认名称 "default"
///
/// # 返回值
///
/// 返回配置实例的 Arc 智能指针
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::gins;
///
/// // 获取默认配置
/// let config = gins::config(None);
///
/// // 获取应用配置
/// let app_config = gins::config(Some("app"));
///
/// // 读取配置值
/// if let Ok(Some(value)) = app_config.get("app.name") {
///     println!("App name: {}", value);
/// }
/// ```
pub fn config(name: Option<&str>) -> Arc<rf_os::cfg::Config> {
    let instance_name = name.unwrap_or("default");
    let key = format!("config.{}", instance_name);

    // Config 未实现 Clone，因此使用 Arc
    let instances = INSTANCE_MANAGER.instances.lock()
        .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
    if let Some(instance) = instances.get(&key) {
        if let Some(typed) = instance.downcast_ref::<Arc<rf_os::cfg::Config>>() {
            return Arc::clone(typed);
        }
    }

    let config = Arc::new(rf_os::cfg::Config::new());
    drop(instances);
    let mut instances = INSTANCE_MANAGER.instances.lock().unwrap();
    instances.insert(key, Box::new(Arc::clone(&config)));
    config
}

/// 获取国际化实例（按名称，从配置加载）
///
/// 此方法获取或创建一个命名的国际化 (i18n) 管理实例。
///
/// # 参数
///
/// * `name` - 实例名称，None 表示使用默认名称 "default"
///
/// # 返回值
///
/// 返回国际化实例的 Arc 智能指针
///
/// # 配置项
///
/// 配置文件中的 `i18n.{name}.language` 字段指定默认语言
/// - 如果未配置，默认使用英语 `"en"`
/// - 常见语言代码：`"en"` (英语)、`"zh"` (中文)、`"ja"` (日语) 等
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::gins;
///
/// // 获取默认国际化实例（英语）
/// let i18n = gins::i18n(None);
///
/// // 获取中文国际化实例
/// let zh_i18n = gins::i18n(Some("zh"));
///
/// // 使用国际化实例获取翻译文本...
/// ```
pub fn i18n(name: Option<&str>) -> Arc<rf_i18n::i18n::I18n> {
    let instance_name = name.unwrap_or("default");
    let key = format!("i18n.{}", instance_name);

    // I18n 未实现 Clone，因此使用 Arc
    let instances = INSTANCE_MANAGER.instances.lock()
        .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
    if let Some(instance) = instances.get(&key) {
        if let Some(typed) = instance.downcast_ref::<Arc<rf_i18n::i18n::I18n>>() {
            return Arc::clone(typed);
        }
    }
    drop(instances);

    // 尝试从配置加载
    let config = rf_os::cfg::Config::new();
    let i18n_instance = if let Ok(Some(lang)) = config.get(&format!("i18n.{}.language", instance_name)) {
        rf_i18n::i18n::I18n::new(&lang)
    } else {
        // 默认语言
        rf_i18n::i18n::I18n::new("en")
    };

    let arc_i18n = Arc::new(i18n_instance);
    {
        let mut instances = INSTANCE_MANAGER.instances.lock()
            .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
        instances.insert(key, Box::new(Arc::clone(&arc_i18n)));
    }
    arc_i18n
}

/// 获取资源存储实例（按名称）
///
/// 此方法获取或创建一个命名的资源存储管理实例。
///
/// # 参数
///
/// * `name` - 实例名称，None 表示使用默认名称 "default"
///
/// # 返回值
///
/// 返回资源存储实例的 Arc 智能指针
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::gins;
///
/// // 获取默认资源存储
/// let resources = gins::resource(None);
///
/// // 获取静态资源存储
/// let static_resources = gins::resource(Some("static"));
///
/// // 使用资源存储加载和管理资源文件...
/// ```
pub fn resource(name: Option<&str>) -> Arc<rf_os::res::ResourceStorage> {
    let instance_name = name.unwrap_or("default");
    let key = format!("resource.{}", instance_name);

    // ResourceStorage 未实现 Clone，因此使用 Arc
    let instances = INSTANCE_MANAGER.instances.lock()
        .expect("Mutex poisoned in InstanceManager - this should not happen in normal operation");
    if let Some(instance) = instances.get(&key) {
        if let Some(typed) = instance.downcast_ref::<Arc<rf_os::res::ResourceStorage>>() {
            return Arc::clone(typed);
        }
    }

    let resource = Arc::new(rf_os::res::ResourceStorage::new());
    drop(instances);
    let mut instances = INSTANCE_MANAGER.instances.lock().unwrap();
    instances.insert(key, Box::new(Arc::clone(&resource)));
    resource
}
