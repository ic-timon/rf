# Net 模块教程

Net 模块提供完整的网络功能支持，包括 HTTP 服务器和客户端、WebSocket、TCP/UDP 等。

## 模块概述

Net 模块包含以下功能：

- **HTTP 服务器**：基于 Axum 的高性能 HTTP 服务器
- **HTTP 客户端**：HTTP 请求客户端
- **WebSocket**：WebSocket 支持
- **TCP/UDP**：底层网络协议
- **负载均衡**：服务负载均衡
- **服务发现**：服务注册和发现
- **分布式追踪**：基于 OpenTelemetry 的链路追踪
- **OpenAPI**：自动生成 API 文档

## 快速开始

### 添加依赖

```toml
[dependencies]
rf-net = { path = "../rf/net" }
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"
```

### 基本导入

```rust
use rf_net::http::HttpServer;
use axum::{routing::get, Router};
use std::net::SocketAddr;
```

## 核心功能

### HTTP 服务器

#### 基本服务器

```rust
use rf_net::http::HttpServer;
use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    
    // 创建路由
    let app = Router::new()
        .route("/", get(|| async { "Hello, RF!" }))
        .route("/api/users", get(get_users));
    
    // 创建服务器
    let mut server = HttpServer::new(addr);
    *server.router() = app;
    
    // 启用日志
    let server = server.with_logging();
    
    println!("Server starting on http://{}", addr);
    server.serve().await?;
    
    Ok(())
}

async fn get_users() -> Json<serde_json::Value> {
    Json(json!({
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
    }))
}
```

#### 路由和参数

```rust
use axum::{extract::{Path, Query}, Json};
use serde::Deserialize;

// 路径参数
async fn get_user(Path(user_id): Path<u32>) -> Json<serde_json::Value> {
    Json(json!({"id": user_id, "name": "User"}))
}

// 查询参数
#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_users(Query(params): Query<Pagination>) -> Json<serde_json::Value> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    Json(json!({"page": page, "limit": limit}))
}

// 路由配置
let app = Router::new()
    .route("/users/:id", get(get_user))
    .route("/users", get(list_users));
```

#### 中间件

```rust
use rf_net::http::HttpServer;
use tower_http::cors::CorsLayer;

let mut server = HttpServer::new(addr);

// 启用 CORS
let server = server.with_cors();

// 启用压缩
let server = server.with_compression();

// 启用日志
let server = server.with_logging();

// 设置请求超时
let server = server.with_request_timeout(std::time::Duration::from_secs(30));
```

#### WebSocket

```rust
use rf_net::http::websocket;
use axum::extract::ws::WebSocket;

async fn websocket_handler(ws: WebSocket) {
    // WebSocket 处理逻辑
    websocket::handle(ws, |msg| {
        println!("收到消息: {:?}", msg);
        // 处理消息并返回响应
    }).await;
}

let app = Router::new()
    .route("/ws", get(websocket_handler));
```

### HTTP 客户端

```rust
use rf_net::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = client::Client::new();
    
    // GET 请求
    let response = client.get("https://api.example.com/users")
        .send()
        .await?;
    
    let users: serde_json::Value = response.json().await?;
    println!("{:?}", users);
    
    // POST 请求
    let response = client.post("https://api.example.com/users")
        .json(&json!({"name": "Alice"}))
        .send()
        .await?;
    
    Ok(())
}
```

### TCP/UDP

```rust
use rf_net::{tcp, udp};

// TCP 服务器
async fn tcp_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = tcp::TcpListener::bind("127.0.0.1:8080").await?;
    
    while let Ok((stream, addr)) = listener.accept().await {
        println!("新连接: {}", addr);
        // 处理连接
    }
    
    Ok(())
}

// UDP 服务器
async fn udp_server() -> Result<(), Box<dyn std::error::Error>> {
    let socket = udp::UdpSocket::bind("127.0.0.1:8080").await?;
    
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("收到来自 {} 的数据: {:?}", addr, &buf[..len]);
    }
}
```

### OpenAPI 文档

```rust
use rf_net::http::HttpServer;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(get_users), components(schemas(User)))]
struct ApiDoc;

#[utoipa::path(get, path = "/users", responses((status = 200, body = Vec<User>)))]
async fn get_users() -> Json<Vec<User>> {
    // ...
}

let mut server = HttpServer::new(addr);
// 添加 Swagger UI
let server = server.with_swagger_ui(ApiDoc::openapi(), "/swagger-ui");
```

## 高级用法

### 自定义中间件

```rust
use axum::middleware;
use tower::ServiceBuilder;

let app = Router::new()
    .route("/", get(|| async { "Hello" }))
    .layer(
        ServiceBuilder::new()
            .layer(middleware::from_fn(custom_middleware))
    );

async fn custom_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    println!("请求: {:?}", req.uri());
    next.run(req).await
}
```

### 请求限流

```rust
use rf_net::http::rate_limit::RateLimiter;

let limiter = RateLimiter::new(100, 1); // 每秒 100 个请求

let app = Router::new()
    .route("/", get(|| async { "Hello" }))
    .layer(limiter);
```

## API 参考

### HTTP 服务器

- `HttpServer::new(addr: SocketAddr) -> Self` - 创建服务器
- `router() -> &mut Router` - 获取路由配置
- `with_logging() -> Self` - 启用日志
- `with_cors() -> Self` - 启用 CORS
- `with_compression() -> Self` - 启用压缩
- `serve() -> Result<()>` - 启动服务器

### HTTP 客户端

- `Client::new() -> Self` - 创建客户端
- `get(url: &str) -> RequestBuilder` - GET 请求
- `post(url: &str) -> RequestBuilder` - POST 请求

## 常见问题

### Q: 如何配置 HTTPS？

A: 使用 TLS 配置，可以通过 `with_tls()` 方法配置 TLS 证书。

### Q: 如何实现文件上传？

A: 使用 `rf_net::http::upload` 模块处理文件上传。

### Q: WebSocket 支持哪些协议？

A: 支持标准的 WebSocket 协议，可以处理文本和二进制消息。

## 相关链接

- [database 模块](../database/README.md) - 数据库操作
- [os 模块](../os/README.md) - 配置和日志

