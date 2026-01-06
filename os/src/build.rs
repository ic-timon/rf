//! # build
//!
//! build 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Build information

//! // Get build information
pub fn info() -> BuildInfo {
    BuildInfo {
        version: env!("CARGO_PKG_VERSION"),
        name: env!("CARGO_PKG_NAME"),
    }
}

/// Build information structure
pub struct BuildInfo {
    pub version: &'static str,
    pub name: &'static str,
}

