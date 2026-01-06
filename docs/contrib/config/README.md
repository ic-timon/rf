# Config 模块教程

Config 模块提供配置中心适配器，支持多种配置中心。

## 模块概述

支持的配置中心：

- Apollo
- Consul
- Nacos
- Kubernetes ConfigMap
- Polaris

## 快速开始

```rust
use rf_contrib_config::{ApolloAdapter, ConfigCenterAdapter};

let adapter = ApolloAdapter::new("http://apollo-server", "app")?;
let value = adapter.get("key")?;
```

## 相关链接

- [os 模块](../../os/README.md) - 配置管理

