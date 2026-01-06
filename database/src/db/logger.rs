//! # logger
//!
//! logger 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Query Logging
//!
//! 查询日志记录模块，用于监控和调试数据库查询。
//!
//! ## 功能特性
//!
//! - 记录所有执行的 SQL 查询
//! - 记录查询执行时间
//! - 支持慢查询监控
//! - 可配置的日志级别
//! - 自动管理日志大小
//!
//! ## 日志级别
//!
//! - `None`: 不记录任何日志
//! - `Error`: 仅记录错误日志（未实现）
//! - `Slow`: 仅记录慢查询（超过阈值的查询）
//! - `All`: 记录所有查询
//!
//! ## 使用场景
//!
//! - 性能分析和优化
//! - 查询调试
//! - 慢查询监控
//! - 审计和合规
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use rf_database::db::logger::{QueryLogger, QueryLogLevel};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 创建日志记录器（记录所有查询，慢查询阈值 100ms，最大 1000 条）
//! let logger = QueryLogger::new(QueryLogLevel::All, Duration::from_millis(100), 1000);
//!
//! // 记录查询
//! logger.log("SELECT * FROM users", Duration::from_millis(50), None).await?;
//!
//! // 获取慢查询
//! let slow_queries = logger.get_slow_queries().await;
//!
//! // 清除日志
//! logger.clear().await;
//! # Ok(())
//! # }
//! ```

use rf_errors::Result;
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

/// 查询日志条目
///
/// 记录单次查询的详细信息。
///
/// ## 字段说明
///
/// - `sql`: SQL 查询语句
/// - `duration`: 查询执行时间
/// - `timestamp`: 查询执行时间戳
/// - `params`: 查询参数（可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLogEntry {
    pub sql: String,
    pub duration: Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub params: Option<Vec<String>>,
}

/// 查询日志级别
///
/// 控制日志记录的详细程度。
///
/// ## 级别说明
///
/// - `None`: 不记录任何日志
/// - `Error`: 仅记录错误（当前未实现）
/// - `Slow`: 仅记录超过阈值的慢查询
/// - `All`: 记录所有查询
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryLogLevel {
    None,
    Error,
    Slow, // 仅记录慢查询
    All,
}

/// 查询日志记录器
///
/// 管理查询日志的存储和检索。
///
/// ## 字段说明
///
/// - `level`: 日志级别
/// - `slow_query_threshold`: 慢查询阈值
/// - `logs`: 日志存储（线程安全）
/// - `max_logs`: 最大日志条目数
pub struct QueryLogger {
    level: QueryLogLevel,
    slow_query_threshold: Duration,
    logs: Arc<RwLock<Vec<QueryLogEntry>>>,
    max_logs: usize,
}

impl QueryLogger {
    /// 创建一个新的查询日志记录器
    ///
    /// ## 参数
    ///
    /// - `level`: 日志级别
    /// - `slow_query_threshold`: 慢查询阈值
    /// - `max_logs`: 最大日志条目数
    ///
    /// ## 返回值
    ///
    /// 返回一个 `QueryLogger` 实例。
    pub fn new(level: QueryLogLevel, slow_query_threshold: Duration, max_logs: usize) -> Self {
        Self {
            level,
            slow_query_threshold,
            logs: Arc::new(RwLock::new(Vec::new())),
            max_logs,
        }
    }

    /// 创建默认配置的查询日志记录器
    ///
    /// ## 默认配置
    ///
    /// - 日志级别：记录所有查询
    /// - 慢查询阈值：100ms
    /// - 最大日志数：1000 条
    ///
    /// ## 返回值
    ///
    /// 返回一个 `QueryLogger` 实例。
    pub fn default() -> Self {
        Self::new(QueryLogLevel::All, Duration::from_millis(100), 1000)
    }

    /// 记录一条查询
    ///
    /// ## 参数
    ///
    /// - `sql`: SQL 查询语句
    /// - `duration`: 查询执行时间
    /// - `params`: 查询参数（可选）
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<()>`。
    ///
    /// ## 注意事项
    ///
    /// - 只有符合日志级别的查询会被记录
    /// - 当日志超过最大条目数时，自动删除最旧的日志
    pub async fn log(&self, sql: &str, duration: Duration, params: Option<Vec<String>>) -> Result<()> {
        if self.level == QueryLogLevel::None {
            return Ok(());
        }

        let should_log = match self.level {
            QueryLogLevel::None => false,
            QueryLogLevel::Error => false, // 需要错误信息
            QueryLogLevel::Slow => duration >= self.slow_query_threshold,
            QueryLogLevel::All => true,
        };

        if should_log {
            let entry = QueryLogEntry {
                sql: sql.to_string(),
                duration,
                timestamp: chrono::Utc::now(),
                params,
            };

            let mut logs = self.logs.write().await;
            logs.push(entry);

            // 限制日志大小
            if logs.len() > self.max_logs {
                logs.remove(0);
            }
        }

        Ok(())
    }

    /// 获取所有日志
    ///
    /// ## 返回值
    ///
    /// 返回所有日志条目的副本。
    pub async fn get_logs(&self) -> Vec<QueryLogEntry> {
        let logs = self.logs.read().await;
        logs.clone()
    }

    /// 获取慢查询
    ///
    /// ## 返回值
    ///
    /// 返回所有超过阈值的慢查询。
    pub async fn get_slow_queries(&self) -> Vec<QueryLogEntry> {
        let logs = self.logs.read().await;
        logs.iter()
            .filter(|entry| entry.duration >= self.slow_query_threshold)
            .cloned()
            .collect()
    }

    /// 清除所有日志
    pub async fn clear(&self) {
        let mut logs = self.logs.write().await;
        logs.clear();
    }

    /// 获取日志条目数
    pub async fn count(&self) -> usize {
        let logs = self.logs.read().await;
        logs.len()
    }

    /// 设置日志级别
    pub fn set_level(&mut self, level: QueryLogLevel) {
        self.level = level;
    }

    /// 设置慢查询阈值
    pub fn set_slow_query_threshold(&mut self, threshold: Duration) {
        self.slow_query_threshold = threshold;
    }
}

/// 查询计时器
///
/// 用于测量查询执行时间。
///
/// ## 字段说明
///
/// - `start`: 查询开始时间
/// - `logger`: 日志记录器（可选）
/// - `sql`: SQL 查询语句
/// - `params`: 查询参数（可选）
pub struct QueryTimer {
    start: Instant,
    logger: Option<Arc<QueryLogger>>,
    sql: String,
    params: Option<Vec<String>>,
}

impl QueryTimer {
    /// 开始计时一个查询
    ///
    /// ## 参数
    ///
    /// - `sql`: SQL 查询语句
    /// - `logger`: 日志记录器（可选）
    /// - `params`: 查询参数（可选）
    ///
    /// ## 返回值
    ///
    /// 返回一个 `QueryTimer` 实例。
    pub fn start(sql: String, logger: Option<Arc<QueryLogger>>, params: Option<Vec<String>>) -> Self {
        Self {
            start: Instant::now(),
            logger,
            sql,
            params,
        }
    }

    /// 完成计时并记录
    ///
    /// ## 返回值
    ///
    /// 返回查询执行时间。
    pub async fn finish(self) -> Duration {
        let duration = self.start.elapsed();
        if let Some(ref logger) = self.logger {
            let _ = logger.log(&self.sql, duration, self.params).await;
        }
        duration
    }
}
