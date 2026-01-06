//! # ipv4
//!
//! ipv4 模块 - IPv4 地址处理工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! IPv4 地址工具
//!
//! 提供 IPv4 地址解析和网络判断功能。
//!
//! # 主要功能
//!
//! - IPv4 地址解析
//! - 网络地址范围判断
//!
//! # 使用示例
//!
//! ```ignore
//! use rf_net::ipv4;
//! use ipnet::Ipv4Net;
//!
//! // 解析 IPv4 地址
//! let addr = ipv4::parse("192.168.1.1")?;
//!
//! // 判断地址是否在网络范围内
//! let network = Ipv4Net::new("192.168.1.0".parse()?, 24)?;
//! assert!(ipv4::in_network(addr, network));
//! ```

use ipnet::Ipv4Net;
use std::net::Ipv4Addr;

/// 解析 IPv4 地址
///
/// # 参数
///
/// - `s`: IPv4 地址字符串，格式为 "a.b.c.d"，例如 "192.168.1.1"
///
/// # 返回值
///
/// - 成功时返回解析后的 `Ipv4Addr`
/// - 失败时返回 `AddrParseError`
///
/// # 示例
///
/// ```ignore
/// let addr = parse("127.0.0.1")?;
/// assert_eq!(addr, Ipv4Addr::new(127, 0, 0, 1));
/// ```
pub fn parse(s: &str) -> Result<Ipv4Addr, std::net::AddrParseError> {
    s.parse()
}

/// 检查 IP 地址是否在指定网络范围内
///
/// # 参数
///
/// - `ip`: 要检查的 IPv4 地址
/// - `network`: 网络范围，使用 CIDR 表示法，例如 "192.168.1.0/24"
///
/// # 返回值
///
/// - `true`: IP 地址在网络范围内
/// - `false`: IP 地址不在网络范围内
///
/// # 示例
///
/// ```ignore
/// let ip = Ipv4Addr::new(192, 168, 1, 100);
/// let network = Ipv4Net::new("192.168.1.0".parse()?, 24)?;
/// assert!(in_network(ip, network));
///
/// // 不同网段
/// let ip2 = Ipv4Addr::new(192, 168, 2, 100);
/// assert!(!in_network(ip2, network));
/// ```
pub fn in_network(ip: Ipv4Addr, network: Ipv4Net) -> bool {
    network.contains(&ip)
}
