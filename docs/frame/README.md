# Frame 模块教程

Frame 模块提供框架实例管理和便捷函数。

## 模块概述

Frame 模块包含：

- **便捷函数（g）**：快速创建各种服务实例
- **实例管理（gins）**：全局实例管理器

## 快速开始

```rust
use rf_frame::{g, gins};

// 使用便捷函数
let server = g::server("127.0.0.1:8080".parse().unwrap());
let db = g::db("postgresql://...").await?;

// 使用实例管理器
let db = gins::database(Some("default")).await?;
```

## 相关链接

- [net 模块](../net/README.md) - HTTP 服务器
- [database 模块](../database/README.md) - 数据库操作

