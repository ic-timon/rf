//! # tidb
//!
//! tidb 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! TiDB 数据库驱动
//!
//! TiDB 是 PingCAP 开发的开源分布式关系型数据库，兼容 MySQL 协议。
//!
//! ## 主要特性
//! - MySQL 兼容：可以直接使用 MySQL 驱动和客户端
//! - 水平扩展：支持自动分片和负载均衡
//! - 强一致性：使用 Raft 协议保证数据一致性
//! - 云原生：支持 Kubernetes 和云环境部署
//! - HTAP：同时支持 OLTP 和 OLAP 工作负载
//!
//! ## 连接字符串格式
//! ```text
//! mysql://user:password@host:port/database
//! ```
use super::{DatabaseDriver, DatabaseConnection};
use rf_errors::{Result, RfError};

/// TiDB 数据库驱动
///
/// 该驱动为 TiDB 数据库提供支持。由于 TiDB 完全兼容 MySQL 协议，
/// 实际使用时应该使用 MySQL 驱动。
///
/// # 注意
/// TiDB 使用 MySQL 协议，因此可以使用任何 MySQL 客户端或驱动连接。
///
/// # 示例
///
/// ```rust
/// let driver = TiDBDriver::new();
/// let name = driver.name(); // 返回 "tidb"
/// ```
pub struct TiDBDriver;

impl TiDBDriver {
    /// 创建新的 TiDB 驱动实例
    ///
    /// # 返回值
    ///
    /// 返回一个新的 TiDBDriver 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// let driver = TiDBDriver::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Default for TiDBDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseDriver for TiDBDriver {
    /// 获取驱动名称
    ///
    /// 返回 "tidb" 作为驱动标识符
    fn name(&self) -> &str {
        "tidb"
    }

    /// 连接到 TiDB 数据库
    ///
    /// 由于 TiDB 兼容 MySQL，建议使用 MySQL 驱动进行连接。
    ///
    /// # 参数
    ///
    /// * `url` - TiDB 连接字符串（MySQL 格式）
    ///
    /// # 返回值
    ///
    /// 返回错误，提示使用 MySQL 驱动
    ///
    /// # 错误
    ///
    /// 始终返回错误，建议使用 MySQL 驱动
    fn connect(&self, url: &str) -> Result<Box<dyn DatabaseConnection>> {
        // TiDB 兼容 MySQL，因此可以使用 MySQL 连接
        // 在完整实现中，会使用 MySQL 驱动从主数据库模块
        Err(RfError::Database(
            format!("TiDB 驱动：请使用 MySQL 驱动配合 TiDB 连接 URL: {}", url)
        ))
    }

    /// 执行 SQL 语句
    ///
    /// 由于 TiDB 兼容 MySQL，建议使用 MySQL 驱动执行语句。
    ///
    /// # 参数
    ///
    /// * `_query` - SQL 查询语句（未使用）
    /// * `_params` - 查询参数（未使用）
    ///
    /// # 返回值
    ///
    /// 返回错误，提示使用 MySQL 驱动
    fn execute(&self, _query: &str, _params: &[&dyn std::any::Any]) -> Result<()> {
        Err(RfError::Database("TiDB 驱动：请使用 MySQL 驱动方法".to_string()))
    }
}
