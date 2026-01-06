# gRPC 模块教程

gRPC 模块提供 gRPC 客户端和服务器支持。

## 模块概述

gRPC 模块功能：

- gRPC 服务器
- gRPC 客户端
- 中间件支持

## 快速开始

```rust
use rf_contrib_grpc::{Server, Client};

// 创建服务器
let server = Server::new("127.0.0.1:50051")?;
server.serve().await?;

// 创建客户端
let client = Client::new("http://127.0.0.1:50051")?;
```

## 相关链接

- [net 模块](../../net/README.md) - HTTP 服务器

