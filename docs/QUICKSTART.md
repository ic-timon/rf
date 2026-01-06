# RF 框架快速开始指南

## 安装

```bash
# 克隆仓库
git clone https://github.com/gogf/rf.git
cd rf

# 构建项目
cargo build --release

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

## 创建第一个项目

### 使用 CLI 工具

```bash
cd cmd/rf
cargo run -- init my-first-app
cd my-first-app
cargo run
```

### 手动创建

1. 创建新项目：
```bash
cargo new my-first-app
cd my-first-app
```

2. 添加依赖到 `Cargo.toml`：
```toml
[dependencies]
rf-net = { path = "../rf/net" }
rf-database = { path = "../rf/database" }
rf-os = { path = "../rf/os" }
rf-util = { path = "../rf/util" }
rf-errors = { path = "../rf/errors" }
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
axum = "0.7"
```

3. 创建 `src/main.rs`：
```rust
use rf_net::http::HttpServer;
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    
    // 创建 HTTP 服务器
    let mut server = HttpServer::new(addr);
    
    // 配置路由
    let app = Router::new()
        .route("/", get(|| async { "Hello, RF Framework!" }))
        .route("/api/users", get(|| async { r#"{"users": []}"# }));
    
    // 将路由添加到服务器
    *server.router() = app;
    
    // 启用日志中间件
    let server = server.with_logging();
    
    println!("Server starting on http://{}", addr);
    server.serve().await?;
    
    Ok(())
}
```

4. 运行：
```bash
cargo run
```

访问 http://127.0.0.1:8080 查看结果。

## 数据库操作示例

```rust
use rf_database::db::{Database, Model};
use serde::{Deserialize, Serialize};

// 定义用户结构体
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<i64>,
    name: String,
    email: String,
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接 PostgreSQL 数据库
    let db = Database::new_postgres("postgresql://user:pass@localhost/dbname").await?;
    
    // 创建模型
    let user_model = Model::new(&db, "users".to_string());
    
    // 查询用户（使用查询构建器）
    let users: Vec<serde_json::Value> = user_model
        .where_("status", "=", "active")
        .order_by("created_at", "DESC")
        .limit(10)
        .all()
        .await?;
    
    // 插入新用户
    let new_user = serde_json::json!({
        "name": "Alice",
        "email": "alice@example.com",
        "status": "active"
    });
    
    let _ = user_model.insert(&new_user).await?;
    
    // 更新用户
    user_model
        .where_("email", "=", "alice@example.com")
        .update(&serde_json::json!({
            "status": "inactive"
        }))
        .await?;
    
    Ok(())
}
```

## 配置管理示例

```rust
use rf_os::cfg::*;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置管理器
    let config = Config::new()
        .adapter(Arc::new(FileConfigAdapter::new("config.toml")?))
        .adapter(Arc::new(EnvConfigAdapter::new()));
    
    // 读取配置值
    let db_url = config.get("database.url")?;
    let port: u16 = config.get("server.port")?
        .unwrap_or("8080".to_string())
        .parse()?;
    
    println!("Database URL: {:?}", db_url);
    println!("Server Port: {}", port);
    
    Ok(())
}
```

## 数据验证示例

```rust
use rf_util::valid::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建验证器
    let validator = Validator::new()
        .rule(Rule::new("email", "required", vec![]))
        .rule(Rule::new("email", "email", vec![]))
        .rule(Rule::new("password", "min_length", vec!["8".to_string()]));
    
    // 准备要验证的数据
    let mut data = HashMap::new();
    data.insert("email".to_string(), "user@example.com".to_string());
    data.insert("password".to_string(), "securepass123".to_string());
    
    // 执行验证
    validator.validate(&data)?;
    
    println!("验证通过！");
    Ok(())
}
```

## 更多功能示例

### 编码/解码

```rust
use rf_encoding::{json_encode, json_decode, base64_encode, base64_decode};

// JSON 编码
let data = serde_json::json!({"name": "RF", "version": "0.1.0"});
let json_str = json_encode(&data)?;
println!("JSON: {}", json_str);

// Base64 编码
let encoded = base64_encode(b"Hello, RF!");
println!("Base64: {}", encoded);
```

### 加密/哈希

```rust
use rf_crypto::{md5, sha256, crc32};

// MD5 哈希
let md5_hash = md5::hash(b"Hello, RF!");
println!("MD5: {:x}", md5_hash);

// SHA256 哈希
let sha256_hash = sha256::hash(b"Hello, RF!");
println!("SHA256: {:x}", sha256_hash);
```

### 文本处理

```rust
use rf_text::str::replace;
use rf_text::regex::is_match;

// 字符串替换
let result = replace("Hello World", "World", "RF");
println!("{}", result); // 输出: Hello RF

// 正则表达式匹配
let matched = is_match(r"\d+", "abc123def")?;
println!("匹配结果: {}", matched);
```

## 下一步学习

- 📚 [查看完整文档索引](INDEX.md)
- 🔧 [核心模块教程](core/README.md)
- 🌐 [网络模块教程](net/README.md)
- 💾 [数据库模块教程](database/README.md)
- ⚙️ [操作系统模块教程](os/README.md)

## 常见问题

### Q: 如何添加更多依赖？

A: 在 `Cargo.toml` 中添加所需的 RF 模块，例如：
```toml
[dependencies]
rf-net = { path = "../rf/net" }
rf-database = { path = "../rf/database" }
rf-encoding = { path = "../rf/encoding" }
```

### Q: 如何配置数据库连接池？

A: 使用 `Database::new_postgres()` 等方法创建连接时，会自动创建连接池。可以通过环境变量或配置文件调整连接池参数。

### Q: 支持哪些数据库？

A: 目前支持 PostgreSQL、MySQL 和 SQLite。更多数据库驱动请查看 [contrib/drivers](contrib/drivers/README.md) 模块。

### Q: 如何启用日志？

A: 使用 `rf_os::log` 模块，或在使用 HTTP 服务器时调用 `with_logging()` 方法。

## 获取帮助

- 📖 查看各模块的详细文档
- 🐛 提交 Issue: https://github.com/gogf/rf/issues
- 💬 参与讨论: https://github.com/gogf/rf/discussions

