//! # ipv6
//!
//! ipv6 模块 - IPv6 地址处理工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! IPv6 地址工具
//!
//! 提供 IPv6 地址解析和网络判断功能。
//!
//! # 主要功能
//!
//! - IPv6 地址解析
//! - 网络地址范围判断
//!
//! # 使用示例
//!
//! ```ignore
//! use rf_net::ipv6;
//! use ipnet::Ipv6Net;
//!
//! // 解析 IPv6 地址
//! let addr = ipv6::parse("2001:db8::1")?;
//!
//! // 判断地址是否在网络范围内
//! let network = Ipv6Net::new("2001:db8::".parse()?, 32)?;
//! assert!(ipv6::in_network(addr, network));
//! ```

use ipnet::Ipv6Net;
use std::net::Ipv6Addr;

/// 解析 IPv6 地址
///
/// # 参数
///
/// - `s`: IPv6 地址字符串，支持多种格式：
///   - 完整形式：`"2001:0db8:85a3:0000:0000:8a2e:0370:7334"`
///   - 简写形式：`"2001:db8:85a3::8a2e:370:7334"`
///   - 环回地址：`"::1"`
///   - 任意地址：`"::"`
///
/// # 返回值
///
/// - 成功时返回解析后的 `Ipv6Addr`
/// - 失败时返回 `AddrParseError`
///
/// # 示例
///
/// ```ignore
/// // 解析完整的 IPv6 地址
/// let addr1 = parse("2001:db8::1")?;
///
/// // 解析环回地址
/// let addr2 = parse("::1")?;
/// assert_eq!(addr2, Ipv6Addr::LOCALHOST);
///
/// // 解析任意地址
/// let addr3 = parse("::")?;
/// assert_eq!(addr3, Ipv6Addr::UNSPECIFIED);
/// ```
pub fn parse(s: &str) -> Result<Ipv6Addr, std::net::AddrParseError> {
    s.parse()
}

/// 检查 IP 地址是否在指定网络范围内
///
/// # 参数
///
/// - `ip`: 要检查的 IPv6 地址
/// - `network`: 网络范围，使用 CIDR 表示法，例如 "2001:db8::/32"
///
/// # 返回值
///
/// - `true`: IP 地址在网络范围内
/// - `false`: IP 地址不在网络范围内
///
/// # 示例
///
/// ```ignore
/// let ip = "2001:db8::1".parse::<Ipv6Addr>()?;
/// let network = Ipv6Net::new("2001:db8::".parse()?, 32)?;
/// assert!(in_network(ip, network));
///
/// // 不同网段
/// let ip2 = "2001:db9::1".parse::<Ipv6Addr>()?;
/// assert!(!in_network(ip2, network));
/// ```
pub fn in_network(ip: Ipv6Addr, network: Ipv6Net) -> bool {
    network.contains(&ip)
}
