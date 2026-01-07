//! # cache
//!
//! cache 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Query Result Cache
//!
//! 查询结果缓存模块，用于提升数据库查询性能。
//!
//! ## 功能特性
//!
//! - 基于内存的高性能缓存
//! - 支持设置缓存容量和过期时间
//! - 自动管理缓存失效
//! - 支持表级别的缓存清理
//!
//! ## 使用场景
//!
//! - 频繁查询但数据变化较少的场景
//! - 复杂查询结果缓存
//! - 减少数据库负载
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use rf_database::db::cache::QueryCache;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 创建缓存（容量 1000，TTL 5 分钟）
//! let cache = QueryCache::new(1000, Duration::from_secs(300));
//!
//! // 获取缓存
//! if let Some(data) = cache.get("SELECT * FROM users").await {
//!     println!("从缓存获取: {:?}", data);
//! } else {
//!     // 执行查询后设置缓存
//!     let result_data = b"query result data".to_vec();
//!     cache.set("SELECT * FROM users", result_data).await;
//! }
//!
//! // 清除特定表的缓存
//! cache.invalidate_table("users").await;
//! # Ok(())
//! # }
//! ```

use moka::future::Cache;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use std::sync::Arc;

/// 查询结果缓存的键
///
/// ## 字段说明
///
/// - `sql`: SQL 查询语句，作为缓存键
#[derive(Debug, Clone)]
struct CacheKey {
    sql: String,
    // 可以添加更多字段以支持基于参数的缓存
}

impl Hash for CacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sql.hash(state);
    }
}

impl PartialEq for CacheKey {
    fn eq(&self, other: &Self) -> bool {
        self.sql == other.sql
    }
}

impl Eq for CacheKey {}

/// 查询缓存管理器
///
/// 用于管理查询结果缓存的核心结构。
///
/// ## 字段说明
///
/// - `cache`: Moka 缓存实例，存储查询结果
pub struct QueryCache {
    cache: Arc<Cache<CacheKey, Vec<u8>>>,
}

impl QueryCache {
    /// 创建一个新的查询缓存
    ///
    /// ## 参数
    ///
    /// - `capacity`: 缓存容量，最大可缓存的条目数
    /// - `ttl`: 缓存条目的生存时间（Time To Live）
    ///
    /// ## 返回值
    ///
    /// 返回一个 `QueryCache` 实例。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::db::cache::QueryCache;
    /// # use std::time::Duration;
    /// #
    /// // 创建容量 1000，TTL 5 分钟的缓存
    /// let cache = QueryCache::new(1000, Duration::from_secs(300));
    /// ```
    pub fn new(capacity: u64, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(capacity)
            .time_to_live(ttl)
            .build();
        
        Self {
            cache: Arc::new(cache),
        }
    }

    /// Create a default query cache (1000 entries, 5 minutes TTL)
    pub fn with_defaults() -> Self {
        Self::new(1000, Duration::from_secs(300))
    }

    /// Get cached result
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let cache_key = CacheKey {
            sql: key.to_string(),
        };
        self.cache.get(&cache_key).await
    }

    /// Set cached result
    pub async fn set(&self, key: &str, value: Vec<u8>) {
        let cache_key = CacheKey {
            sql: key.to_string(),
        };
        self.cache.insert(cache_key, value).await;
    }

    /// Invalidate cache entry
    pub async fn invalidate(&self, key: &str) {
        let cache_key = CacheKey {
            sql: key.to_string(),
        };
        self.cache.invalidate(&cache_key).await;
    }

    /// Clear all cache
    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }

    /// Invalidate cache entries matching a pattern (table name)
    pub async fn invalidate_table(&self, _table: &str) {
        // This is a simplified implementation
        // Full implementation would track table->cache_key mappings
        self.cache.invalidate_all();
    }
}

