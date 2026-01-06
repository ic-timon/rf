# HTTP Client SDK 教程

HTTP Client SDK 提供 HTTP 客户端功能。

## 模块概述

HTTP Client SDK 功能：

- HTTP 请求
- 请求配置
- 响应处理

## 快速开始

```rust
use rf_contrib_sdk_httpclient::Client;

let client = Client::new()?;
let response = client.get("https://api.example.com").send().await?;
```

## 相关链接

- [net 模块](../../net/README.md) - HTTP 客户端

