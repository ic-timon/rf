# RF 框架文档索引

欢迎使用 RF 框架！本文档提供了所有模块的快速链接和学习路径。

## 快速开始

- [快速开始指南](QUICKSTART.md) - 5 分钟快速上手 RF 框架

## 核心模块

### 基础模块

- [core 模块](core/README.md) - 核心类型和 Trait
- [errors 模块](errors/README.md) - 错误处理机制

### 常用模块

- [container 模块](container/README.md) - 容器数据结构（数组、列表、映射、集合、队列、树等）
- [util 模块](util/README.md) - 工具函数（类型转换、数据验证、随机数、GUID 等）
- [text 模块](text/README.md) - 文本处理（字符串操作、正则表达式）

### 核心功能模块

- [os 模块](os/README.md) - 操作系统相关功能（文件、配置、日志、进程、定时器等）
- [net 模块](net/README.md) - 网络功能（HTTP 服务器/客户端、WebSocket、TCP/UDP 等）
- [database 模块](database/README.md) - 数据库 ORM 和操作（PostgreSQL、MySQL、SQLite、Redis）

### 工具类模块

- [encoding 模块](encoding/README.md) - 编码/解码（JSON、XML、YAML、TOML、Base64 等）
- [crypto 模块](crypto/README.md) - 加密和哈希（AES、DES、RSA、SHA、MD5 等）

### 辅助模块

- [i18n 模块](i18n/README.md) - 国际化支持
- [debug 模块](debug/README.md) - 调试工具
- [test 模块](test/README.md) - 测试工具
- [frame 模块](frame/README.md) - 框架实例管理和便捷函数

## Contrib 模块

### 服务治理

- [config 模块](contrib/config/README.md) - 配置中心适配器（Apollo、Consul、Nacos、K8s ConfigMap、Polaris）
- [registry 模块](contrib/registry/README.md) - 服务注册发现（Consul、etcd、Nacos、Zookeeper、文件注册中心）

### 微服务

- [grpc 模块](contrib/grpc/README.md) - gRPC 客户端和服务器支持
- [trace 模块](contrib/trace/README.md) - 分布式追踪（OpenTelemetry OTLP）

### 扩展模块

- [drivers 模块](contrib/drivers/README.md) - 数据库驱动扩展（ClickHouse、Dameng、GaussDB、OceanBase、Oracle、SQL Server、TiDB）
- [httpclient 模块](contrib/sdk/httpclient/README.md) - HTTP 客户端 SDK

## 学习路径建议

### 初学者路径

1. **第一步**：阅读 [快速开始指南](QUICKSTART.md)
2. **第二步**：学习基础模块
   - [core 模块](core/README.md) - 了解核心类型
   - [errors 模块](errors/README.md) - 掌握错误处理
3. **第三步**：学习常用工具
   - [util 模块](util/README.md) - 类型转换和数据验证
   - [text 模块](text/README.md) - 文本处理
   - [encoding 模块](encoding/README.md) - 数据编码
4. **第四步**：构建 Web 应用
   - [net 模块](net/README.md) - HTTP 服务器
   - [database 模块](database/README.md) - 数据库操作
   - [os 模块](os/README.md) - 配置和日志

### 进阶路径

1. **Web 开发进阶**
   - [net 模块](net/README.md) - 深入了解 HTTP 服务器、中间件、WebSocket
   - [database 模块](database/README.md) - ORM 高级用法、事务、缓存
   - [os 模块](os/README.md) - 文件操作、定时任务、会话管理

2. **微服务开发**
   - [grpc 模块](contrib/grpc/README.md) - gRPC 服务
   - [registry 模块](contrib/registry/README.md) - 服务注册发现
   - [config 模块](contrib/config/README.md) - 配置中心
   - [trace 模块](contrib/trace/README.md) - 分布式追踪

3. **工具和优化**
   - [container 模块](container/README.md) - 高性能容器
   - [crypto 模块](crypto/README.md) - 加密和安全
   - [frame 模块](frame/README.md) - 框架实例管理

### 专业路径

1. **数据库专家**
   - [database 模块](database/README.md) - 深入 ORM 和查询优化
   - [drivers 模块](contrib/drivers/README.md) - 各种数据库驱动

2. **系统架构师**
   - [registry 模块](contrib/registry/README.md) - 服务治理
   - [config 模块](contrib/config/README.md) - 配置管理
   - [trace 模块](contrib/trace/README.md) - 可观测性

3. **性能优化专家**
   - [container 模块](container/README.md) - 数据结构优化
   - [os 模块](os/README.md) - 缓存和资源管理
   - [database 模块](database/README.md) - 查询优化和连接池

## 按功能分类

### Web 开发
- [net 模块](net/README.md) - HTTP 服务器和客户端
- [database 模块](database/README.md) - 数据库操作
- [os 模块](os/README.md) - 配置、日志、会话

### 数据处理
- [encoding 模块](encoding/README.md) - 数据编码/解码
- [text 模块](text/README.md) - 文本处理
- [util 模块](util/README.md) - 类型转换和验证

### 安全
- [crypto 模块](crypto/README.md) - 加密和哈希

### 微服务
- [grpc 模块](contrib/grpc/README.md)
- [registry 模块](contrib/registry/README.md)
- [config 模块](contrib/config/README.md)
- [trace 模块](contrib/trace/README.md)

### 工具和辅助
- [container 模块](container/README.md)
- [util 模块](util/README.md)
- [frame 模块](frame/README.md)
- [debug 模块](debug/README.md)
- [test 模块](test/README.md)

## 相关资源

- [项目主页](https://github.com/gogf/rf)
- [API 文档](https://docs.rs/rf)
- [GoFrame 原项目](https://github.com/gogf/gf)
- [Rust 官网](https://www.rust-lang.org/)

## 贡献

欢迎贡献代码和文档！请查看项目的 CONTRIBUTING.md 了解详细信息。

