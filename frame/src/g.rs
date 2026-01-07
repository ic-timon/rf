//! # g
//!
//! g 模块 - 框架便捷函数集合
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Framework convenience functions
//!
//! 提供一系列便捷的全局函数，用于快速创建和获取框架中的各种服务实例。
//!
//! ## 功能分类
//!
//! ### 网络服务
//! - `server()`: 创建 HTTP 服务器
//! - `tcp_server()`: 创建 TCP 服务器
//! - `udp_socket()`: 创建 UDP 套接字
//! - `client()`: 创建 HTTP 客户端
//!
//! ### 数据存储
//! - `db()`: 创建数据库连接实例
//! - `model()`: 创建 ORM 模型实例
//! - `redis()`: 创建 Redis 客户端实例
//!
//! ### 系统服务
//! - `config()`: 创建配置管理实例
//! - `view()`: 创建视图引擎实例
//! - `log()`: 初始化日志系统
//! - `i18n()`: 创建国际化实例
//! - `resource()`: 创建资源管理实例
//! - `validator()`: 创建数据验证器实例
//!
//! ### 异步工具
//! - `go()`: 启动异步任务
//! - `wait()`: 等待服务器关闭信号
//! - `listen()`: 监听系统信号
//!
//! ### 调试工具
//! - `dump()`: 打印调试信息
//! - `dump_with_type()`: 打印带类型信息的调试信息
//!
//! ### 错误处理
//! - `try()`: 捕获同步代码错误
//! - `try_catch()`: 捕获异步代码错误
//!
//! ### 工具函数
//! - `is_nil()`: 检查值是否为 None
//! - `is_empty()`: 检查字符串是否为空
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_frame::g;
//!
//! // 创建 HTTP 服务器
//! let server = g::server("127.0.0.1:8080".parse().unwrap());
//!
//! // 创建数据库连接
//! let db = g::db("postgresql://localhost/mydb").await?;
//!
//! // 启动异步任务
//! g::go(async {
//!     println!("Hello from background task!");
//! });
//!
//! // 等待关闭信号
//! g::wait().await?;
//! ```

use rf_errors::Result;
use std::net::SocketAddr;

/// 创建 HTTP 服务器实例
///
/// 此函数创建一个新的 HTTP 服务器实例，绑定到指定的地址。
///
/// # 参数
///
/// * `addr` - 服务器绑定的网络地址（SocketAddr 类型）
///
/// # 返回值
///
/// 返回一个 `HttpServer` 实例，可以用于配置路由和启动服务器
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let server = g::server("127.0.0.1:8080".parse().unwrap());
/// // 配置路由并启动服务器...
/// ```
pub fn server(addr: SocketAddr) -> rf_net::http::HttpServer {
    rf_net::http::HttpServer::new(addr)
}

/// 创建 TCP 服务器实例
///
/// 此函数创建一个 TCP 服务器，可以接受传入的 TCP 连接。
/// 适用于需要自定义协议或长时间连接的场景。
///
/// # 参数
///
/// * `addr` - 服务器绑定的地址字符串，格式为 "IP:PORT"（如 "127.0.0.1:8080"）
///
/// # 返回值
///
/// 返回一个 `Result<TcpServer>`，成功时包含 TCP 服务器实例
///
/// # 错误
///
/// 当地址无效或绑定失败时返回错误
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let server = g::tcp_server("127.0.0.1:8080").await?;
///     // 接受并处理连接
///     let (stream, addr) = server.accept().await?;
///     println!("收到来自 {} 的连接", addr);
///     Ok(())
/// }
/// ```
pub async fn tcp_server(addr: &str) -> Result<rf_net::tcp::TcpServer> {
    rf_net::tcp::TcpServer::bind(addr).await
}

/// 创建 UDP 套接字实例
///
/// 此函数创建一个 UDP 套接字，可以发送和接收数据报。
/// 适用于无需连接的、面向消息的通信场景。
///
/// # 参数
///
/// * `addr` - 套接字绑定的地址字符串，格式为 "IP:PORT"（如 "127.0.0.1:8080"）
///
/// # 返回值
///
/// 返回一个 `Result<UdpSocketWrapper>`，成功时包含 UDP 套接字包装器
///
/// # 错误
///
/// 当地址无效或绑定失败时返回错误
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let socket = g::udp_socket("127.0.0.1:8080").await?;
///     let mut buf = [0u8; 1024];
///     let (len, addr) = socket.recv_from(&mut buf).await?;
///     println!("从 {} 接收到 {} 字节", addr, len);
///     Ok(())
/// }
/// ```
pub async fn udp_socket(addr: &str) -> Result<rf_net::udp::UdpSocketWrapper> {
    rf_net::udp::UdpSocketWrapper::bind(addr).await
}

/// 已弃用：使用 tcp_server 代替
///
/// 这是一个遗留的函数名称，请使用 `tcp_server()` 代替。
/// # Deprecated
/// 
/// **此函数已弃用。** 这是一个占位符实现，不提供真正的UDP服务器功能。
/// 
/// 对于UDP功能，请使用 `udp_socket()` 或直接使用 `tokio::net::UdpSocket`。
/// 
/// 此函数仅保留用于向后兼容，将在未来版本中移除。
#[deprecated(note = "Use udp_socket() or tokio::net::UdpSocket instead. Will be removed in future version.")]
pub fn udp_server(_addr: SocketAddr) -> Result<()> {
    // 已弃用：使用 udp_socket() 来实现实际的 UDP 功能
    // Note: udp_server() is deprecated, use udp_socket() or tokio::net::UdpSocket instead
    Ok(())
}

/// 创建 HTTP 客户端实例
///
/// 此函数创建一个新的 HTTP 客户端，用于发送 HTTP 请求。
/// 客户端内部使用连接池，可以高效地复用连接。
///
/// # 返回值
///
/// 返回一个 `reqwest::Client` 实例
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = g::client();
///     let response = client.get("https://www.rust-lang.org")
///         .send()
///         .await?;
///     Ok(())
/// }
/// ```
pub fn client() -> reqwest::Client {
    reqwest::Client::new()
}

/// 创建数据库连接实例
///
/// 此函数创建一个 PostgreSQL 数据库连接实例。
///
/// # 参数
///
/// * `url` - 数据库连接 URL，格式为 "postgresql://host/database"
///
/// # 返回值
///
/// 返回一个 `Result<Database>`，成功时包含数据库实例
///
/// # 错误
///
/// 当连接失败时返回错误
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let db = g::db("postgresql://localhost/mydb").await?;
///     // 执行数据库操作...
///     Ok(())
/// }
/// ```
pub async fn db(url: &str) -> Result<rf_database::db::Database> {
    rf_database::db::Database::new_postgres(url).await
}

/// 创建 ORM 模型实例
///
/// 此函数为指定表创建一个 ORM 模型，用于数据库操作。
///
/// # 参数
///
/// * `database` - 数据库实例的引用
/// * `table` - 数据表名称
///
/// # 返回值
///
/// 返回一个 `Model` 实例，用于执行 CRUD 操作
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let db = g::db("postgresql://localhost/mydb").await?;
///     let user_model = g::model(&db, "users");
///     // 使用模型进行查询...
///     Ok(())
/// }
/// ```
pub fn model(database: &rf_database::db::Database, table: &str) -> rf_database::db::Model {
    database.model(table)
}

/// 创建 Redis 客户端实例
///
/// 此函数创建一个 Redis 客户端连接。
///
/// # 参数
///
/// * `url` - Redis 服务器 URL，格式为 "redis://host:port/"
///
/// # 返回值
///
/// 返回一个 `Result<RedisClient>`，成功时包含 Redis 客户端实例
///
/// # 错误
///
/// 当连接失败时返回错误
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let redis = g::redis("redis://127.0.0.1:6379/").await?;
///     // 执行 Redis 操作...
///     Ok(())
/// }
/// ```
pub async fn redis(url: &str) -> Result<rf_database::redis::RedisClient> {
    rf_database::redis::RedisClient::new(url).await
}

/// 创建配置管理实例
///
/// 此函数创建一个配置管理器，用于读取和管理应用配置。
/// 配置可以从环境变量、配置文件等来源加载。
///
/// # 返回值
///
/// 返回一个 `Config` 实例
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let config = g::config();
/// if let Ok(Some(value)) = config.get("app.name") {
///     println!("App name: {}", value);
/// }
/// ```
pub fn config() -> rf_os::cfg::Config {
    rf_os::cfg::Config::new()
}

/// 创建视图引擎实例
///
/// 此函数创建一个视图引擎，用于渲染模板文件。
///
/// # 参数
///
/// * `template_dir` - 模板文件目录路径
///
/// # 返回值
///
/// 返回一个 `Result<View>`，成功时包含视图引擎实例
///
/// # 错误
///
/// 当模板目录不存在或无效时返回错误
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::g;
///
/// let view = g::view("templates")?;
/// // 渲染模板...
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn view(template_dir: &str) -> Result<rf_os::view::View> {
    rf_os::view::View::new(template_dir)
}

/// 初始化日志系统
///
/// 此函数初始化 tracing 日志系统。
/// 如果已经初始化过，则不会重复初始化。
///
/// # 返回值
///
/// 总是返回 `Ok(())`
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     g::log()?;
///     // 现在可以使用 tracing::info! 等宏记录日志
///     tracing::info!("日志系统已初始化");
///     Ok(())
/// }
/// ```
pub fn log() -> Result<()> {
    // 如果尚未初始化，则初始化 tracing 订阅者
    use tracing_subscriber;
    tracing_subscriber::fmt::init();
    Ok(())
}

/// 创建国际化实例
///
/// 此函数创建一个国际化 (i18n) 管理器，用于多语言支持。
/// 默认语言为英语 ("en")。
///
/// # 返回值
///
/// 返回一个 `I18n` 实例
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let i18n = g::i18n();
/// // 设置语言并获取翻译文本...
/// ```
pub fn i18n() -> rf_i18n::i18n::I18n {
    rf_i18n::i18n::I18n::new("en")
}

/// 创建资源管理实例
///
/// 此函数创建一个资源存储管理器，用于管理应用资源文件。
///
/// # 返回值
///
/// 返回一个 `ResourceStorage` 实例
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let resources = g::resource();
/// // 加载和管理资源...
/// ```
pub fn resource() -> rf_os::res::ResourceStorage {
    rf_os::res::ResourceStorage::new()
}

/// 创建数据验证器实例
///
/// 此函数创建一个数据验证器，用于验证用户输入和数据完整性。
///
/// # 返回值
///
/// 返回一个 `Validator` 实例
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let validator = g::validator();
/// // 使用验证器验证数据...
/// ```
pub fn validator() -> rf_util::Validator {
    rf_util::Validator::new()
}

/// 启动异步任务
///
/// 此函数在后台启动一个异步任务，不会阻塞当前线程。
/// 任务会在 Tokio 运行时中执行。
///
/// # 参数
///
/// * `task` - 要执行的异步任务（Future）
///
/// # 泛型参数
///
/// * `F` - 实现 Future 的类型，输出为 ()
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// // 启动后台任务
/// g::go(async {
///     // 执行一些异步操作
///     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
///     println!("后台任务完成");
/// });
///
/// // 继续执行其他操作...
/// println!("主线程继续执行");
/// ```
pub fn go<F>(task: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    tokio::spawn(task);
}

/// 等待服务器关闭信号
///
/// 此函数会阻塞当前异步任务，直到收到 Ctrl+C 信号。
/// 通常用于保持服务器运行直到用户主动关闭。
///
/// # 返回值
///
/// 返回 `Result<()>`，成功时表示正常关闭
///
/// # 错误
///
/// 当信号监听失败时返回错误
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // 启动服务器...
///     println!("服务器运行中，按 Ctrl+C 退出");
///
///     // 等待关闭信号
///     g::wait().await?;
///
///     println!("服务器正在关闭...");
///     Ok(())
/// }
/// ```
pub async fn wait() -> Result<()> {
    tokio::signal::ctrl_c().await
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to wait for shutdown: {}", e)))?;
    Ok(())
}

/// 监听系统信号
///
/// 此函数监听系统信号（如 Ctrl+C），用于优雅地关闭应用。
///
/// # 返回值
///
/// 返回 `Result<()>`，成功时表示成功接收到信号
///
/// # 错误
///
/// 当信号监听失败时返回错误
///
/// # 使用示例
///
/// ```no_run
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // 启动服务...
///
///     // 监听关闭信号
///     g::listen().await?;
///
///     // 执行清理工作
///     println!("正在清理资源...");
///     Ok(())
/// }
/// ```
pub async fn listen() -> Result<()> {
    tokio::signal::ctrl_c().await
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to listen for signals: {}", e)))?;
    Ok(())
}

/// 打印调试信息
///
/// 此函数使用调试格式 (Debug format) 打印值的详细信息。
/// 适用于快速调试和数据检查。
///
/// # 参数
///
/// * `value` - 要打印的值的引用，必须实现 Debug trait
///
/// # 泛型参数
///
/// * `T` - 实现 Debug trait 的任意类型
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let data = vec![1, 2, 3];
/// g::dump(&data);
/// // 输出:
/// // [
/// //     1,
/// //     2,
/// //     3,
/// // ]
/// ```
pub fn dump<T: std::fmt::Debug>(value: &T) {
    println!("{:#?}", value);
}

/// 打印带类型信息的调试信息
///
/// 此函数打印值的详细信息，同时显示其类型名称。
/// 适用于需要了解具体类型的调试场景。
///
/// # 参数
///
/// * `value` - 要打印的值的引用，必须实现 Debug trait
///
/// # 泛型参数
///
/// * `T` - 实现 Debug trait 的任意类型
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let data = vec![1, 2, 3];
/// g::dump_with_type(&data);
/// // 输出: Type: alloc::vec::Vec<i32>, Value: [
/// //     1,
/// //     2,
/// //     3,
/// // ]
/// ```
pub fn dump_with_type<T: std::fmt::Debug>(value: &T) {
    println!("Type: {}, Value: {:#?}", std::any::type_name::<T>(), value);
}

/// 捕获同步代码错误
///
/// 此函数执行一个可能返回结果的闭包，并将任何错误转换为框架内部的错误类型。
///
/// # 参数
///
/// * `f` - 一个返回 Result 的闭包
///
/// # 泛型参数
///
/// * `F` - 闭包类型
/// * `T` - 成功时的值类型
/// * `E` - 错误类型，必须实现 Display
///
/// # 返回值
///
/// 返回 `Result<T>`，成功时包含原值，失败时包含内部错误
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let result = g::try(|| {
///     let value = "42".parse::<i32>()?;
///     Ok(value * 2)
/// });
///
/// match result {
///     Ok(value) => println!("结果: {}", value),
///     Err(e) => println!("错误: {}", e),
/// }
/// ```
pub fn r#try<F, T, E>(f: F) -> Result<T>
where
    F: FnOnce() -> std::result::Result<T, E>,
    E: std::fmt::Display,
{
    f().map_err(|e| rf_errors::RfError::Internal(format!("{}", e)))
}

/// 捕获异步代码错误
///
/// 此函数执行一个可能返回结果的异步闭包，并将任何错误转换为框架内部的错误类型。
///
/// # 参数
///
/// * `f` - 一个返回异步 Result 的闭包
///
/// # 泛型参数
///
/// * `F` - 闭包类型
/// * `Fut` - Future 类型
/// * `T` - 成功时的值类型
/// * `E` - 错误类型，必须实现 Display
///
/// # 返回值
///
/// 返回 `Result<T>`，成功时包含原值，失败时包含内部错误
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let result = g::try_catch(|| async {
///         let value = "42".parse::<i32>()?;
///         Ok(value * 2)
///     }).await?;
///
///     println!("结果: {}", result);
///     Ok(())
/// }
/// ```
pub async fn try_catch<F, Fut, T, E>(f: F) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = std::result::Result<T, E>>,
    E: std::fmt::Display,
{
    f().await.map_err(|e| rf_errors::RfError::Internal(format!("{}", e)))
}

/// 检查值是否为 None
///
/// 此函数检查一个 Option 值是否为 None。
///
/// # 参数
///
/// * `value` - 要检查的 Option 值的引用
///
/// # 返回值
///
/// 如果值为 None 返回 true，否则返回 false
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// let some_value = Some(42);
/// let none_value: Option<i32> = None;
///
/// assert_eq!(g::is_nil(&some_value), false);
/// assert_eq!(g::is_nil(&none_value), true);
/// ```
pub fn is_nil<T>(value: &Option<T>) -> bool {
    value.is_none()
}

/// 检查字符串是否为空
///
/// 此函数检查一个字符串是否为空。
///
/// # 参数
///
/// * `value` - 要检查的字符串值，任何实现 AsRef<str> 的类型
///
/// # 返回值
///
/// 如果字符串为空返回 true，否则返回 false
///
/// # 使用示例
///
/// ```rust
/// use rf_frame::g;
///
/// assert_eq!(g::is_empty(""), true);
/// assert_eq!(g::is_empty("hello"), false);
/// assert_eq!(g::is_empty(String::from("text")), false);
/// ```
pub fn is_empty<T: AsRef<str>>(value: T) -> bool {
    value.as_ref().is_empty()
}
