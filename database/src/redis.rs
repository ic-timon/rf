//! # redis
//!
//! redis 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Redis Client
//!
//! Redis 客户端模块，提供对 Redis 数据库的异步操作接口。
//!
//! ## 功能特性
//!
//! - 支持连接池管理
//! - 提供多种 Redis 数据结构的操作接口
//! - 支持多种数据类型：String、Hash、List、Set、SortedSet 等
//! - 支持发布订阅、Lua 脚本等高级功能
//!
//! ## 操作分组
//!
//! Redis 操作按数据类型分组，每组提供独立的接口：
//!
//! - `string()`: String 操作（GET、SET、INCR 等）
//! - `hash()`: Hash 操作（HGET、HSET、HGETALL 等）
//! - `list()`: List 操作（LPUSH、RPOP、LRANGE 等）
//! - `set_ops()`: Set 操作（SADD、SREM、SMEMBERS 等）
//! - `sorted_set()`: Sorted Set 操作（ZADD、ZRANGE、ZREM 等）
//! - `generic()`: 通用操作（EXISTS、DEL、EXPIRE 等）
//! - `pubsub()`: 发布订阅操作（PUBLISH）
//! - `script()`: Lua 脚本操作（EVAL、SCRIPT LOAD）
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use rf_database::redis::RedisClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 创建 Redis 客户端
//! let client = RedisClient::new("redis://127.0.0.1/").await?;
//!
//! // String 操作
//! client.set("key", "value").await?;
//! let value = client.get("key").await?;
//!
//! // 使用分组操作
//! let string_ops = client.string();
//! string_ops.set_ex("key", "value", 60).await?;
//!
//! // Hash 操作
//! let hash_ops = client.hash();
//! hash_ops.hset("user:1", "name", "Alice").await?;
//! let name = hash_ops.hget("user:1", "name").await?;
//! # Ok(())
//! # }
//! ```

mod groups;

use redis::aio::MultiplexedConnection;
use redis::{Client, AsyncCommands};
use rf_errors::{Result, RfError};
use std::sync::Arc;
use tokio::sync::Mutex;

pub use groups::*;

/// Redis 客户端包装器，提供连接池和操作接口
///
/// ## 字段说明
///
/// - `connection`: 多路复用连接的共享引用
pub struct RedisClient {
    connection: Arc<Mutex<MultiplexedConnection>>,
}

impl RedisClient {
    /// 创建一个新的 Redis 客户端
    ///
    /// ## 参数
    ///
    /// - `url`: Redis 服务器连接字符串，格式：`redis://host[:port][/db]`
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<RedisClient>`，成功时包含 Redis 客户端实例，失败时返回错误信息。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// use rf_database::redis::RedisClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // 连接本地 Redis
    /// let client = RedisClient::new("redis://127.0.0.1/").await?;
    ///
    /// // 连接远程 Redis
    /// let client = RedisClient::new("redis://192.168.1.100:6379/").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(url: &str) -> Result<Self> {
        let client = Client::open(url)
            .map_err(|e| RfError::Database(format!("Failed to create Redis client: {}", e)))?;

        let connection = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RfError::Database(format!("Failed to create connection: {}", e)))?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    /// 设置键值对
    ///
    /// ## 参数
    ///
    /// - `key`: 键名
    /// - `value`: 值
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<()>`，成功时返回空，失败时返回错误信息。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// client.set("username", "alice").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.connection.lock().await;
        conn.set::<_, _, ()>(key, value).await
            .map_err(|e| RfError::Database(format!("Redis SET failed: {}", e)))?;
        Ok(())
    }

    /// 获取键的值
    ///
    /// ## 参数
    ///
    /// - `key`: 键名
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<String>`，成功时返回键对应的值，失败时返回错误信息。
    /// 如果键不存在，返回空字符串。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let value = client.get("username").await?;
    /// println!("Username: {}", value);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, key: &str) -> Result<String> {
        let mut conn = self.connection.lock().await;
        conn.get::<_, String>(key).await
            .map_err(|e| RfError::Database(format!("Redis GET failed: {}", e)))
    }

    /// 获取 String 操作分组
    ///
    /// ## 返回值
    ///
    /// 返回 `StringGroup`，提供 String 类型的所有操作。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let string_ops = client.string();
    /// string_ops.set_ex("key", "value", 60).await?;
    /// string_ops.incr("counter").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn string(&self) -> StringGroup {
        StringGroup {
            connection: self.connection.clone(),
        }
    }

    /// 获取 Hash 操作分组
    ///
    /// ## 返回值
    ///
    /// 返回 `HashGroup`，提供 Hash 类型的所有操作。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let hash_ops = client.hash();
    /// hash_ops.hset("user:1", "name", "Alice").await?;
    /// hash_ops.hset("user:1", "age", "25").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn hash(&self) -> HashGroup {
        HashGroup {
            connection: self.connection.clone(),
        }
    }

    /// 获取 List 操作分组
    ///
    /// ## 返回值
    ///
    /// 返回 `ListGroup`，提供 List 类型的所有操作。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let list_ops = client.list();
    /// list_ops.lpush("queue", &["task1", "task2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self) -> ListGroup {
        ListGroup {
            connection: self.connection.clone(),
        }
    }

    /// 获取 Set 操作分组
    ///
    /// ## 返回值
    ///
    /// 返回 `SetGroup`，提供 Set 类型的所有操作。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let set_ops = client.set_ops();
    /// set_ops.sadd("tags", &["rust", "redis", "database"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_ops(&self) -> SetGroup {
        SetGroup {
            connection: self.connection.clone(),
        }
    }

    /// 获取 Sorted Set 操作分组
    ///
    /// ## 返回值
    ///
    /// 返回 `SortedSetGroup`，提供 Sorted Set 类型的所有操作。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let sorted_set = client.sorted_set();
    /// sorted_set.zadd("leaderboard", 100.0, "player1").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn sorted_set(&self) -> SortedSetGroup {
        SortedSetGroup {
            connection: self.connection.clone(),
        }
    }

    /// 获取通用操作分组
    ///
    /// ## 返回值
    ///
    /// 返回 `GenericGroup`，提供通用操作（EXISTS、DEL、EXPIRE 等）。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let generic = client.generic();
    /// generic.expire("key", 60).await?;
    /// generic.del(&["key1", "key2"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn generic(&self) -> GenericGroup {
        GenericGroup {
            connection: self.connection.clone(),
        }
    }

    /// 获取发布订阅操作分组
    ///
    /// ## 返回值
    ///
    /// 返回 `PubSubGroup`，提供发布订阅功能。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let pubsub = client.pubsub();
    /// pubsub.publish("channel", "message").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn pubsub(&self) -> PubSubGroup {
        PubSubGroup {
            connection: self.connection.clone(),
        }
    }

    /// 获取 Lua 脚本操作分组
    ///
    /// ## 返回值
    ///
    /// 返回 `ScriptGroup`，提供 Lua 脚本执行功能。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::redis::RedisClient;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = RedisClient::new("redis://127.0.0.1/").await?;
    /// let script = client.script();
    /// // 加载脚本
    /// let sha = script.script_load("return redis.call('GET', KEYS[1])").await?;
    /// // 执行脚本
    /// script.evalsha(&sha, &["key"], &[]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn script(&self) -> ScriptGroup {
        ScriptGroup {
            connection: self.connection.clone(),
        }
    }
}
