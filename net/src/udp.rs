//! # udp
//!
//! udp 模块 - UDP 网络协议支持
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! UDP 客户端和服务器
//!
//! 提供 UDP 套接字的封装，用于无连接的、不可靠的数据报传输。
//! UDP 适用于对实时性要求高、可以容忍少量数据丢失的场景。
//!
//! # 主要功能
//!
//! - UDP 套接字绑定：绑定到本地端口
//! - 数据发送：向指定地址发送数据
//! - 数据接收：从网络接收数据
//!
//! # 使用示例
//!
//! ## 服务器端
//! ```ignore
//! use rf_net::UdpSocketWrapper;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let socket = UdpSocketWrapper::bind("127.0.0.1:8080").await?;
//!     let mut buf = [0u8; 1024];
//!     loop {
//!         let (len, addr) = socket.recv_from(&mut buf).await?;
//!         println!("收到来自 {} 的 {} 字节数据", addr, len);
//!     }
//! }
//! ```
//!
//! ## 客户端
//! ```ignore
//! let socket = UdpSocketWrapper::bind("127.0.0.1:0").await?;
//! let data = b"Hello UDP";
//! socket.send_to(data, "127.0.0.1:8080").await?;
//! ```

use rf_errors::Result;
use tokio::net::UdpSocket;

/// UDP 套接字封装
///
/// 提供 UDP 套接字的封装，用于发送和接收 UDP 数据报。
///
/// # 字段
///
/// - `socket`: 底层的 Tokio UDP 套接字
///
/// # 特性
///
/// UDP 是无连接的协议，具有以下特点：
/// - 无连接：不需要建立连接，直接发送数据
/// - 不可靠：数据可能丢失、重复或乱序
/// - 高效：开销小，传输速度快
/// - 支持单播、广播和多播
pub struct UdpSocketWrapper {
    socket: UdpSocket,
}

impl UdpSocketWrapper {
    /// 绑定 UDP 套接字到指定地址
    ///
    /// # 参数
    ///
    /// - `addr`: 要绑定的地址，格式为 "host:port"
    ///   - 对于服务器，通常绑定到 "0.0.0.0:port" 监听所有接口
    ///   - 对于客户端，可以绑定到 "127.0.0.1:0" 让系统自动分配端口
    ///
    /// # 返回值
    ///
    /// 返回一个绑定到指定地址的 UDP 套接字封装
    ///
    /// # 错误
    ///
    /// 如果绑定失败，返回 RfError::Network 错误
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 服务器：绑定到特定端口
    /// let socket = UdpSocketWrapper::bind("0.0.0.0:8080").await?;
    ///
    /// // 客户端：绑定到任意端口
    /// let socket = UdpSocketWrapper::bind("127.0.0.1:0").await?;
    /// ```
    pub async fn bind(addr: &str) -> Result<Self> {
        let socket = UdpSocket::bind(addr).await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to bind UDP socket: {}", e)))?;
        Ok(Self { socket })
    }

    /// 向指定目标发送数据
    ///
    /// # 参数
    ///
    /// - `buf`: 要发送的数据缓冲区
    /// - `target`: 目标地址，格式为 "host:port"
    ///
    /// # 返回值
    ///
    /// 返回实际发送的字节数
    ///
    /// # 错误
    ///
    /// 如果发送失败，返回 RfError::Network 错误
    ///
    /// # 注意
    ///
    /// - UDP 不保证数据一定送达
    /// - 单次发送的数据大小受 MTU 限制（通常不超过 65507 字节）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let data = b"Hello, UDP!";
    /// let sent = socket.send_to(data, "127.0.0.1:8080").await?;
    /// println!("发送了 {} 字节", sent);
    /// ```
    pub async fn send_to(&self, buf: &[u8], target: &str) -> Result<usize> {
        self.socket.send_to(buf, target).await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to send: {}", e)))
    }

    /// 从套接字接收数据
    ///
    /// # 参数
    ///
    /// - `buf`: 接收数据的缓冲区
    ///
    /// # 返回值
    ///
    /// 返回一个元组：
    /// - `usize`: 实际接收的字节数
    /// - `SocketAddr`: 发送方的地址
    ///
    /// # 错误
    ///
    /// 如果接收失败，返回 RfError::Network 错误
    ///
    /// # 注意
    ///
    /// - 此方法会阻塞直到接收到数据
    /// - 如果缓冲区太小，超出的数据会被丢弃
    /// - UDP 不保证消息的边界和顺序
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let mut buf = [0u8; 1024];
    /// loop {
    ///     let (len, addr) = socket.recv_from(&mut buf).await?;
    ///     println!("从 {} 收到 {} 字节: {:?}", addr, len, &buf[..len]);
    /// }
    /// ```
    pub async fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, std::net::SocketAddr)> {
        self.socket.recv_from(buf).await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to receive: {}", e)))
    }
}
