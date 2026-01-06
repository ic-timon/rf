//! # tcp
//!
//! tcp 模块 - TCP 网络协议支持
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! TCP 客户端和服务器
//!
//! 提供 TCP 服务器和客户端的封装，用于建立可靠的、面向连接的网络通信。
//!
//! # 主要功能
//!
//! - TCP 服务器：绑定端口并接受连接
//! - TCP 客户端：连接到远程服务器
//!
//! # 使用示例
//!
//! ## 服务器端
//! ```ignore
//! use rf_net::TcpServer;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let server = TcpServer::bind("127.0.0.1:8080").await?;
//!     let (stream, addr) = server.accept().await?;
//!     println!("连接来自: {}", addr);
//!     Ok(())
//! }
//! ```
//!
//! ## 客户端
//! ```ignore
//! use rf_net::TcpClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let stream = TcpClient::connect("127.0.0.1:8080").await?;
//!     // 使用 stream 进行通信
//!     Ok(())
//! }
//! ```

use rf_errors::Result;
use tokio::net::{TcpListener, TcpStream};

/// TCP 服务器
///
/// 提供 TCP 服务器功能，可以绑定到指定地址并接受客户端连接。
///
/// # 字段
///
/// - `listener`: 底层的 Tokio TCP 监听器
pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    /// 创建并绑定一个新的 TCP 服务器
    ///
    /// # 参数
    ///
    /// - `addr`: 要绑定的地址，格式为 "host:port"，例如 "127.0.0.1:8080" 或 "0.0.0.0:8080"
    ///
    /// # 返回值
    ///
    /// 返回一个绑定到指定地址的 TcpServer 实例
    ///
    /// # 错误
    ///
    /// 如果绑定失败，返回 RfError::Network 错误
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let server = TcpServer::bind("127.0.0.1:8080").await?;
    /// ```
    pub async fn bind(addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(addr).await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to bind TCP server: {}", e)))?;
        Ok(Self { listener })
    }

    /// 接受一个客户端连接
    ///
    /// # 返回值
    ///
    /// 返回一个元组，包含：
    /// - `TcpStream`: 与客户端的 TCP 流，可用于数据传输
    /// - `SocketAddr`: 客户端的套接字地址
    ///
    /// # 错误
    ///
    /// 如果接受连接失败，返回 RfError::Network 错误
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let (stream, addr) = server.accept().await?;
    /// println!("客户端 {} 已连接", addr);
    /// // 使用 stream 进行读写操作
    /// ```
    pub async fn accept(&self) -> Result<(TcpStream, std::net::SocketAddr)> {
        self.listener.accept().await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to accept connection: {}", e)))
    }
}

/// TCP 客户端
///
/// 提供连接到 TCP 服务器的功能
pub struct TcpClient;

impl TcpClient {
    /// 连接到指定的 TCP 服务器
    ///
    /// # 参数
    ///
    /// - `addr`: 服务器地址，格式为 "host:port"，例如 "127.0.0.1:8080"
    ///
    /// # 返回值
    ///
    /// 返回与服务器建立的 TCP 流
    ///
    /// # 错误
    ///
    /// 如果连接失败，返回 RfError::Network 错误
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let stream = TcpClient::connect("127.0.0.1:8080").await?;
    /// // 使用 stream 进行数据传输
    /// ```
    pub async fn connect(addr: &str) -> Result<TcpStream> {
        TcpStream::connect(addr).await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to connect: {}", e)))
    }
}

