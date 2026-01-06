//! # timer
//!
//! timer 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Timer utilities

use tokio::time::{interval as tokio_interval, Duration, Interval};

/// Create an interval timer
pub fn interval(period: Duration) -> Interval {
    tokio_interval(period)
}

/// Sleep for a duration
pub async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}
