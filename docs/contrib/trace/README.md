# Trace 模块教程

Trace 模块提供分布式追踪支持。

## 模块概述

Trace 模块功能：

- OpenTelemetry OTLP 集成
- 分布式追踪配置
- 追踪数据收集

## 快速开始

```rust
use rf_contrib_trace;

// 初始化追踪
rf_contrib_trace::init("service-name", "http://collector:4317")?;

// 创建 span
let span = rf_contrib_trace::span("operation");
// 执行操作...
```

## 相关链接

- [net 模块](../../net/README.md) - HTTP 服务器

