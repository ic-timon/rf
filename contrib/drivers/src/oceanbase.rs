//! # oceanbase
//!
//! oceanbase 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! OceanBase 数据库驱动
//!
//! OceanBase 是蚂蚁集团自主研发的企业级分布式关系数据库。
//!
//! ## 主要特性
//! - 兼容性：同时兼容 MySQL 和 Oracle 协议
//! - 分布式架构：支持数百节点集群
//! - 高性能：单机性能和分布式性能均优异
//! - 高可用：支持多地多活、故障自动恢复
//! - 金融级：支持金融场景的一致性和可靠性要求
//!
//! ## 连接字符串格式
//! ```text
//! // MySQL 模式
//! mysql://user:password@host:port/database
//! // Oracle 模式
//! oracle://user:password@host:port/database
//! ```
use super::{DatabaseDriver, DatabaseConnection};
use rf_errors::{Result, RfError};

/// OceanBase 数据库驱动
///
/// 该驱动为 OceanBase 数据库提供支持。由于 OceanBase 兼容 MySQL，
/// 实际使用时应该使用 MySQL 驱动。
///
/// # 注意
/// OceanBase 提供 MySQL 和 Oracle 两种兼容模式。
/// - MySQL 模式：使用 MySQL 驱动
/// - Oracle 模式：使用 Oracle 驱动
///
/// # 示例
///
/// ```rust
/// let driver = OceanBaseDriver::new();
/// let name = driver.name(); // 返回 "oceanbase"
/// ```
pub struct OceanBaseDriver;

impl OceanBaseDriver {
    /// 创建新的 OceanBase 驱动实例
    ///
    /// # 返回值
    ///
    /// 返回一个新的 OceanBaseDriver 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// let driver = OceanBaseDriver::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Default for OceanBaseDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseDriver for OceanBaseDriver {
    /// 获取驱动名称
    ///
    /// 返回 "oceanbase" 作为驱动标识符
    fn name(&self) -> &str {
        "oceanbase"
    }

    /// 连接到 OceanBase 数据库
    ///
    /// 由于 OceanBase 兼容 MySQL，建议使用 MySQL 驱动进行连接。
    ///
    /// # 参数
    ///
    /// * `url` - OceanBase 连接字符串（MySQL 或 Oracle 格式）
    ///
    /// # 返回值
    ///
    /// 返回错误，提示使用 MySQL 驱动
    ///
    /// # 错误
    ///
    /// 始终返回错误，建议使用 MySQL 驱动
    fn connect(&self, url: &str) -> Result<Box<dyn DatabaseConnection>> {
        // OceanBase 兼容 MySQL，因此可以使用 MySQL 驱动
        Err(RfError::Database(
            format!("OceanBase 驱动：请使用 MySQL 驱动配合 OceanBase 连接 URL: {}", url)
        ))
    }

    /// 执行 SQL 语句
    ///
    /// 由于 OceanBase 兼容 MySQL，建议使用 MySQL 驱动执行语句。
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
        Err(RfError::Database("OceanBase 驱动：请使用 MySQL 驱动方法".to_string()))
    }
}
