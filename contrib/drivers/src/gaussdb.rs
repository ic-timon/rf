//! # gaussdb
//!
//! gaussdb 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! GaussDB 数据库驱动
//!
//! GaussDB 是华为企业级分布式数据库，兼容 PostgreSQL 协议。
//!
//! ## 特性
//! - PostgreSQL 兼容：可以直接使用 PostgreSQL 驱动连接
//! - 高性能：针对 OLTP 和 OLAP 场景优化
//! - 分布式架构：支持 shared-nothing 架构
//! - 高可用：支持自动故障转移和数据冗余
//!
//! ## 连接字符串格式
//! ```text
//! postgresql://user:password@host:port/database
//! ```
use super::{DatabaseDriver, DatabaseConnection};
use rf_errors::{Result, RfError};

/// GaussDB 数据库驱动
///
/// 该驱动为 GaussDB 数据库提供支持，由于 GaussDB 兼容 PostgreSQL，
/// 实际使用时应该使用 PostgreSQL 驱动。
///
/// # 示例
///
/// ```rust
/// use rf_contrib_drivers::{gaussdb::GaussDBDriver, DatabaseDriver};
/// let driver = GaussDBDriver::new();
/// let name = driver.name(); // 返回 "gaussdb"
/// ```
pub struct GaussDBDriver;

impl GaussDBDriver {
    /// 创建新的 GaussDB 驱动实例
    ///
    /// # 返回值
    ///
    /// 返回一个新的 GaussDBDriver 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_contrib_drivers::gaussdb::GaussDBDriver;
    /// let driver = GaussDBDriver::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Default for GaussDBDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseDriver for GaussDBDriver {
    /// 获取驱动名称
    ///
    /// 返回 "gaussdb" 作为驱动标识符
    fn name(&self) -> &str {
        "gaussdb"
    }

    /// 连接到 GaussDB 数据库
    ///
    /// 由于 GaussDB 兼容 PostgreSQL，建议使用 PostgreSQL 驱动进行连接。
    ///
    /// # 参数
    ///
    /// * `url` - GaussDB 连接字符串（PostgreSQL 格式）
    ///
    /// # 返回值
    ///
    /// 返回错误，提示使用 PostgreSQL 驱动
    ///
    /// # 错误
    ///
    /// 始终返回错误，建议使用 PostgreSQL 驱动
    fn connect(&self, url: &str) -> Result<Box<dyn DatabaseConnection>> {
        // GaussDB 兼容 PostgreSQL，因此可以使用 PostgreSQL 驱动
        Err(RfError::Database(
            format!("GaussDB 驱动：请使用 PostgreSQL 驱动配合 GaussDB 连接 URL: {}", url)
        ))
    }

    /// 执行 SQL 语句
    ///
    /// 由于 GaussDB 兼容 PostgreSQL，建议使用 PostgreSQL 驱动执行语句。
    ///
    /// # 参数
    ///
    /// * `_query` - SQL 查询语句（未使用）
    /// * `_params` - 查询参数（未使用）
    ///
    /// # 返回值
    ///
    /// 返回错误，提示使用 PostgreSQL 驱动
    fn execute(&self, _query: &str, _params: &[&dyn std::any::Any]) -> Result<()> {
        Err(RfError::Database("GaussDB 驱动：请使用 PostgreSQL 驱动方法".to_string()))
    }
}

