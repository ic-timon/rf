# Registry 模块教程

Registry 模块提供服务注册发现功能。

## 模块概述

支持的服务注册中心：

- Consul
- etcd
- Nacos
- Zookeeper
- 文件注册中心

## 快速开始

```rust
use rf_contrib_registry::{ConsulRegistry, ServiceInstance};

let registry = ConsulRegistry::new("http://consul:8500")?;

let instance = ServiceInstance {
    id: "service-1".to_string(),
    name: "my-service".to_string(),
    address: "127.0.0.1:8080".parse().unwrap(),
    metadata: HashMap::new(),
    health: ServiceHealth::Healthy,
};

registry.register(&instance)?;
```

## 相关链接

- [net 模块](../../net/README.md) - HTTP 服务器

