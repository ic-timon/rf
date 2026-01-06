//! # log
//!
//! log 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Logging system

use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::EnvFilter;

/// Initialize the logging system
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}

/// Initialize with custom level
pub fn init_with_level(level: Level) {
    let filter = EnvFilter::new(level.as_str());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}

/// Log at trace level
pub fn trace(msg: &str) {
    trace!("{}", msg);
}

/// Log at debug level
pub fn debug(msg: &str) {
    debug!("{}", msg);
}

/// Log at info level
pub fn info(msg: &str) {
    info!("{}", msg);
}

/// Log at warn level
pub fn warn(msg: &str) {
    warn!("{}", msg);
}

/// Log at error level
pub fn error(msg: &str) {
    error!("{}", msg);
}
