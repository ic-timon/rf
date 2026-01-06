# OS 模块教程

OS 模块提供与操作系统相关的功能，包括文件操作、配置管理、日志、进程管理、定时任务等。

## 模块概述

OS 模块包含以下功能：

- **文件操作（file）**：文件读写、目录操作、文件搜索
- **配置管理（cfg）**：多种配置适配器（文件、环境变量、内存）
- **日志系统（log）**：日志记录和管理
- **进程管理（proc）**：进程操作和监控
- **定时任务（cron、timer）**：定时任务调度
- **会话管理（session）**：Session 存储和管理
- **时间处理（time）**：时间格式化、解析
- **环境变量（env）**：环境变量操作
- **缓存系统（cache）**：缓存管理
- **视图模板（view）**：模板引擎

## 快速开始

### 添加依赖

```toml
[dependencies]
rf-os = { path = "../rf/os" }
```

### 基本导入

```rust
use rf_os::{file, cfg, log, time, env};
```

## 核心功能

### 文件操作

```rust
use rf_os::file;

// 读取文件
let content = file::read_to_string("config.toml")?;
println!("{}", content);

// 写入文件
file::write("output.txt", "Hello, RF!")?;

// 检查文件是否存在
if file::exists("config.toml") {
    println!("文件存在");
}

// 创建目录
file::create_dir_all("logs/2024")?;

// 列出目录内容
let entries = file::read_dir(".")?;
for entry in entries {
    println!("{}", entry.path().display());
}
```

### 配置管理

```rust
use rf_os::cfg::*;
use std::sync::Arc;

// 创建配置管理器
let config = Config::new()
    .adapter(Arc::new(FileConfigAdapter::new("config.toml")?))
    .adapter(Arc::new(EnvConfigAdapter::new()));

// 读取配置
let db_url = config.get("database.url")?;
let port: u16 = config.get("server.port")?
    .unwrap_or("8080".to_string())
    .parse()?;

// 设置配置
config.set("debug", "true")?;
```

### 日志系统

```rust
use rf_os::log;

// 初始化日志
log::init()?;

// 记录日志
log::info("应用启动");
log::warn("配置项缺失");
log::error("数据库连接失败");

// 使用格式化日志
log::info(&format!("用户 {} 登录", "alice"));
```

### 时间处理

```rust
use rf_os::time;

// 获取当前时间戳
let timestamp = time::now().timestamp();

// 格式化时间
let formatted = time::format(time::now(), "%Y-%m-%d %H:%M:%S")?;
println!("{}", formatted);

// 解析时间字符串
let parsed = time::parse("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")?;
```

### 环境变量

```rust
use rf_os::env;

// 获取环境变量
if let Some(value) = env::get("DATABASE_URL") {
    println!("数据库URL: {}", value);
}

// 设置环境变量
env::set("DEBUG", "true")?;

// 获取所有环境变量
let vars = env::all();
for (key, value) in vars {
    println!("{} = {}", key, value);
}
```

### 定时任务

```rust
use rf_os::cron;
use std::time::Duration;

// 创建定时任务
let scheduler = cron::Scheduler::new();

// 添加每分钟执行的任务
scheduler.add_job(
    cron::Job::new("0 * * * * *", || {
        println!("每分钟执行");
    })
)?;

// 添加延迟任务
tokio::spawn(async {
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("5秒后执行");
});
```

### 会话管理

```rust
use rf_os::session::{SessionManager, MemorySessionStorage};

// 创建会话管理器
let manager = SessionManager::new();

// 创建会话
let session_id = "session_123";
// manager.store(session_id, session)?;

// 获取会话
// let session = manager.get(session_id)?;
```

### 缓存系统

```rust
use rf_os::cache;
use std::time::Duration;

// 创建缓存
let cache = cache::new();

// 设置缓存（带过期时间）
cache.set("key", "value", Some(Duration::from_secs(60)))?;

// 获取缓存
if let Some(value) = cache.get::<String>("key")? {
    println!("缓存值: {}", value);
}

// 删除缓存
cache.remove("key")?;
```

## 高级用法

### 文件监控

```rust
use rf_os::fsnotify;

// 创建文件监控器
let watcher = fsnotify::Watcher::new()?;

// 监控文件变化
watcher.watch("config.toml", |event| {
    println!("文件变化: {:?}", event);
})?;
```

### 配置验证

```rust
use rf_os::cfg::*;
use std::sync::Arc;

let config = Config::new()
    .adapter(Arc::new(FileConfigAdapter::new("config.toml")?))
    .with_validator(ConfigValidator::new()
        .rule("database.url", Box::new(RequiredRule))
        .rule("server.port", Box::new(RangeRule::new(Some(1), Some(65535)))));

// 验证配置
config.validate()?;
```

### 日志级别控制

```rust
use rf_os::log;

// 设置日志级别
log::set_level(log::Level::Info);

// 只记录 Info 及以上级别的日志
log::debug("调试信息"); // 不会输出
log::info("信息"); // 会输出
log::warn("警告"); // 会输出
```

## API 参考

### 文件操作

- `read_to_string(path: &str) -> Result<String>` - 读取文件
- `write(path: &str, content: &str) -> Result<()>` - 写入文件
- `exists(path: &str) -> bool` - 检查文件是否存在
- `create_dir_all(path: &str) -> Result<()>` - 创建目录
- `read_dir(path: &str) -> Result<Vec<DirEntry>>` - 列出目录

### 配置管理

- `Config::new() -> Self` - 创建配置管理器
- `Config::adapter(adapter) -> Self` - 添加配置适配器
- `Config::get(key: &str) -> Result<Option<String>>` - 获取配置值
- `Config::set(key: &str, value: &str) -> Result<()>` - 设置配置值

### 日志系统

- `log::init() -> Result<()>` - 初始化日志
- `log::info(msg: &str)` - 记录信息日志
- `log::warn(msg: &str)` - 记录警告日志
- `log::error(msg: &str)` - 记录错误日志
- `log::debug(msg: &str)` - 记录调试日志

## 常见问题

### Q: 如何配置日志输出到文件？

A: 使用日志配置功能，可以设置日志输出目标（控制台、文件等）。

### Q: 配置文件支持哪些格式？

A: 支持 TOML、JSON、YAML 等格式，通过不同的适配器实现。

### Q: 定时任务支持哪些表达式格式？

A: 支持标准的 cron 表达式格式（秒 分 时 日 月 周）。

## 相关链接

- [net 模块](../net/README.md) - 网络功能
- [database 模块](../database/README.md) - 数据库操作

