# RF - Rust Framework

Rust 实现的 GoFrame 框架，提供完整的 Web 开发工具集。

## 项目结构

本项目采用 Cargo workspace 组织，每个模块作为独立的 crate：

### 核心模块
- `core` - 核心框架模块
- `container` - 容器模块（数组、列表、映射、集合、队列、树等）
- `os` - 操作系统相关模块（文件、配置、日志、进程、定时器等）
- `net` - 网络模块（HTTP服务器、客户端、WebSocket、gRPC等）
- `database` - 数据库模块（ORM、查询构建器、事务、缓存等）
- `encoding` - 编码/解码模块（JSON、XML、YAML、TOML、Base64等）
- `crypto` - 加密模块（AES、DES、RSA、SHA、MD5等）
- `util` - 工具模块（类型转换、数据验证、随机数、GUID等）
- `errors` - 错误处理模块
- `text` - 文本处理模块（字符串操作、正则表达式）
- `i18n` - 国际化模块
- `debug` - 调试模块
- `test` - 测试工具模块
- `frame` - 框架实例管理

### Contrib 模块
- `contrib/config` - 配置中心适配器（Apollo、Consul、Nacos、K8s ConfigMap、Polaris）
- `contrib/registry` - 服务注册发现（Consul、etcd、Nacos、Zookeeper、文件注册中心）
- `contrib/grpc` - gRPC 客户端和服务器支持
- `contrib/drivers` - 数据库驱动扩展（ClickHouse、Dameng、GaussDB、OceanBase、Oracle、SQL Server、TiDB）
- `contrib/sdk/httpclient` - HTTP 客户端 SDK
- `contrib/trace` - 分布式追踪支持（OpenTelemetry OTLP）

### CLI 工具
- `cmd/rf` - RF 框架命令行工具

## 快速开始

### 安装

```bash
# 克隆仓库
git clone https://github.com/gogf/rf.git
cd rf

# 构建所有模块
cargo build

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

### 创建新项目

```bash
# 使用 CLI 工具创建新项目
cd cmd/rf
cargo run -- init my-project

# 或者手动创建
cd my-project
cargo run
```

### 基本使用示例

#### HTTP 服务器

```rust
use rf_net::http::HttpServer;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let server = HttpServer::new(addr)
        .route("/", |_| async { "Hello, RF!" });
    
    println!("Server starting on {}", addr);
    server.serve().await?;
    
    Ok(())
}
```

#### 数据库操作

```rust
use rf_database::db::Database;
use rf_frame::g;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 连接数据库
    let db = g::db("postgresql://user:pass@localhost/dbname").await?;
    
    // 创建模型
    let user_model = db.model("users");
    
    // 查询
    let users = user_model
        .where_("age", ">", "18")
        .order_by("created_at", "DESC")
        .limit(10)
        .all_postgres::<User>()
        .await?;
    
    // 插入
    user_model
        .insert(&serde_json::json!({
            "name": "John",
            "age": 25
        }))
        .await?;
    
    Ok(())
}
```

#### 配置管理

```rust
use rf_os::cfg::*;
use std::sync::Arc;

let config = Config::new()
    .adapter(Arc::new(FileConfigAdapter::new("config.toml")?))
    .adapter(Arc::new(EnvConfigAdapter::new()))
    .with_validator(ConfigValidator::new()
        .rule("database.url", Box::new(RequiredRule))
        .rule("server.port", Box::new(RangeRule::new(Some(1), Some(65535)))));

let db_url = config.get("database.url")?;
```

#### 数据验证

```rust
use rf_util::valid::*;

let validator = Validator::new()
    .rule(Rule::new("email", "required", vec![]))
    .rule(Rule::new("email", "email", vec![]))
    .rule(Rule::new("age", "between", vec!["18".to_string(), "100".to_string()]));

validator.validate(&data)?;
```

## 主要特性

### ORM 功能
- ✅ 关联查询（With/Join）
- ✅ 软删除支持
- ✅ 查询结果缓存
- ✅ 批量操作优化
- ✅ 原始 SQL 执行
- ✅ 多数据库支持（PostgreSQL、MySQL、SQLite）

### HTTP 服务器功能
- ✅ 中间件支持
- ✅ 路由和参数解析
- ✅ WebSocket 支持
- ✅ 静态文件服务
- ✅ 文件上传处理
- ✅ OpenAPI/Swagger UI 集成
- ✅ 路由缓存
- ✅ 请求体大小限制
- ✅ URL 重写

### 配置管理
- ✅ 多种适配器（文件、环境变量、内存）
- ✅ 配置验证
- ✅ 配置加密
- ✅ 配置变更通知

### 服务注册发现
- ✅ Consul 适配器
- ✅ etcd 适配器
- ✅ Nacos 适配器
- ✅ Zookeeper 适配器
- ✅ 文件注册中心

### 配置中心
- ✅ Apollo 适配器
- ✅ Consul 适配器
- ✅ Nacos 适配器
- ✅ Kubernetes ConfigMap 适配器
- ✅ Polaris 适配器

### 数据库驱动扩展
- ✅ ClickHouse 驱动
- ✅ Dameng（达梦）驱动
- ✅ GaussDB 驱动
- ✅ OceanBase 驱动
- ✅ Oracle 驱动
- ✅ SQL Server 驱动
- ✅ TiDB 驱动

### 分布式追踪
- ✅ OpenTelemetry OTLP 支持

### 其他功能
- ✅ Session 管理（Memory/File/Redis/Database）
- ✅ 文件操作高级功能（匹配、排序、缓存、权限、搜索）
- ✅ 类型转换系统（支持缓存和指针转换）
- ✅ 数据验证系统（50+ 验证规则）
- ✅ CLI 工具（项目初始化、代码生成、数据库迁移、服务管理）

## 文档

- [API 文档](https://docs.rs/rf)
- [使用示例](examples/)
- [迁移指南](docs/migration.md)

## 贡献

欢迎贡献代码！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详细信息。

## 许可证

MIT License

## 相关链接

- [GoFrame 原项目](https://github.com/gogf/gf)
- [Rust 官网](https://www.rust-lang.org/)
