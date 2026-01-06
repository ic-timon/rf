//! # pool_monitor
//!
//! pool_monitor 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Connection Pool Monitoring
//!
//! 数据库连接池监控模块，用于跟踪连接池状态和健康度。
//!
//! ## 功能特性
//!
//! - 实时监控连接池状态
//! - 检测连接池健康度
//! - 统计连接使用情况
//! - 记录等待时间和等待次数
//!
//! ## 监控指标
//!
//! - `total_connections`: 总连接数
//! - `idle_connections`: 空闲连接数
//! - `active_connections`: 活跃连接数
//! - `max_connections`: 最大连接数
//! - `wait_count`: 等待次数
//! - `wait_duration`: 总等待时间
//! - `last_activity`: 最后活动时间
//!
//! ## 健康状态
//!
//! - `Healthy`: 池使用率 < 80%
//! - `Degraded`: 池使用率 > 80%
//! - `Unhealthy`: 池已满（活跃连接 = 最大连接）
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use rf_database::db::pool_monitor::PoolMonitor;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 创建连接池监控器（最大连接 100）
//! let monitor = PoolMonitor::new(100);
//!
//! // 更新连接计数
//! monitor.update_connection_count(10, 3, 7).await;
//!
//! // 检查健康状态
//! let health = monitor.check_health().await;
//!
//! // 获取统计信息
//! let stats = monitor.get_stats().await;
//! println!("利用率: {:.1}%", monitor.utilization().await);
//! # Ok(())
//! # }
//! ```

use rf_errors::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime};

/// 连接池统计信息
///
/// 记录连接池的运行状态和性能指标。
///
/// ## 字段说明
///
/// - `total_connections`: 总连接数
/// - `idle_connections`: 空闲连接数
/// - `active_connections`: 活跃连接数
/// - `max_connections`: 最大连接数
/// - `wait_count`: 等待获取连接的次数
/// - `wait_duration`: 累计等待时间
/// - `last_activity`: 最后一次活动时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    pub total_connections: usize,
    pub idle_connections: usize,
    pub active_connections: usize,
    pub max_connections: usize,
    pub wait_count: usize,
    pub wait_duration: Duration,
    pub last_activity: Option<SystemTime>,
}

/// 连接池健康状态
///
/// 表示连接池当前的健康程度。
///
/// ## 状态说明
///
/// - `Healthy`: 健康，连接池使用率 < 80%，可以正常处理请求
/// - `Degraded`: 降级，连接池使用率 > 80%，需要关注
/// - `Unhealthy`: 不健康，连接池已满，无法处理新请求
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PoolHealth {
    Healthy,
    Degraded,
    Unhealthy,
}

/// 连接池监控器
///
/// 监控数据库连接池的状态和健康度。
///
/// ## 字段说明
///
/// - `stats`: 连接池统计信息（线程安全）
/// - `health_check_interval`: 健康检查间隔
pub struct PoolMonitor {
    stats: Arc<RwLock<PoolStats>>,
    health_check_interval: Duration,
}

impl PoolMonitor {
    /// 创建一个新的连接池监控器
    ///
    /// ## 参数
    ///
    /// - `max_connections`: 最大连接数
    ///
    /// ## 返回值
    ///
    /// 返回一个 `PoolMonitor` 实例。
    ///
    /// ## 默认配置
    ///
    /// - 健康检查间隔：30 秒
    pub fn new(max_connections: usize) -> Self {
        Self {
            stats: Arc::new(RwLock::new(PoolStats {
                total_connections: 0,
                idle_connections: 0,
                active_connections: 0,
                max_connections,
                wait_count: 0,
                wait_duration: Duration::from_secs(0),
                last_activity: None,
            })),
            health_check_interval: Duration::from_secs(30),
        }
    }

    /// 更新连接计数
    ///
    /// ## 参数
    ///
    /// - `total`: 总连接数
    /// - `idle`: 空闲连接数
    /// - `active`: 活跃连接数
    ///
    /// ## 注意事项
    ///
    /// 此方法应在每次获取或释放连接时调用，以保持统计信息的准确性。
    pub async fn update_connection_count(&self, total: usize, idle: usize, active: usize) {
        let mut stats = self.stats.write().await;
        stats.total_connections = total;
        stats.idle_connections = idle;
        stats.active_connections = active;
        stats.last_activity = Some(SystemTime::now());
    }

    /// 增加等待计数
    ///
    /// ## 参数
    ///
    /// - `duration`: 等待时间
    ///
    /// ## 注意事项
    ///
    /// 此方法应在等待获取连接时调用，用于跟踪连接池的压力。
    pub async fn increment_wait(&self, duration: Duration) {
        let mut stats = self.stats.write().await;
        stats.wait_count += 1;
        stats.wait_duration += duration;
    }

    /// 获取当前统计信息
    ///
    /// ## 返回值
    ///
    /// 返回连接池统计信息的副本。
    pub async fn get_stats(&self) -> PoolStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// 检查连接池健康状态
    ///
    /// ## 返回值
    ///
    /// 返回当前的健康状态。
    ///
    /// ## 健康判断标准
    ///
    /// - `Unhealthy`: 活跃连接 >= 最大连接
    /// - `Degraded`: 使用率 > 80%
    /// - `Healthy`: 使用率 <= 80%
    pub async fn check_health(&self) -> PoolHealth {
        let stats = self.stats.read().await;

        // 检查连接池是否已满
        if stats.active_connections >= stats.max_connections {
            return PoolHealth::Unhealthy;
        }

        // 检查使用率是否过高（>80%）
        let utilization = if stats.max_connections > 0 {
            (stats.active_connections as f64 / stats.max_connections as f64) * 100.0
        } else {
            0.0
        };

        if utilization > 80.0 {
            PoolHealth::Degraded
        } else {
            PoolHealth::Healthy
        }
    }

    /// 获取连接池使用率
    ///
    /// ## 返回值
    ///
    /// 返回使用率百分比（0.0 - 100.0）。
    pub async fn utilization(&self) -> f64 {
        let stats = self.stats.read().await;
        if stats.max_connections > 0 {
            (stats.active_connections as f64 / stats.max_connections as f64) * 100.0
        } else {
            0.0
        }
    }

    /// 获取平均等待时间
    ///
    /// ## 返回值
    ///
    /// 返回每次等待的平均时间。
    ///
    /// ## 计算方式
    ///
    /// 平均等待时间 = 总等待时间 / 等待次数
    pub async fn average_wait_time(&self) -> Duration {
        let stats = self.stats.read().await;
        if stats.wait_count > 0 {
            stats.wait_duration / stats.wait_count as u32
        } else {
            Duration::from_secs(0)
        }
    }

    /// 重置统计信息
    ///
    /// 清除等待计数和等待时间，其他统计信息保持不变。
    pub async fn reset(&self) {
        let mut stats = self.stats.write().await;
        stats.wait_count = 0;
        stats.wait_duration = Duration::from_secs(0);
    }

    /// 设置健康检查间隔
    ///
    /// ## 参数
    ///
    /// - `interval`: 健康检查的时间间隔
    pub fn set_health_check_interval(&mut self, interval: Duration) {
        self.health_check_interval = interval;
    }
}
