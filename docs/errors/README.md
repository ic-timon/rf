# Errors 模块教程

Errors 模块为 RF 框架提供统一的错误处理机制，包括错误类型定义、错误码管理和错误传播。

## 模块概述

Errors 模块提供了：

- **错误类型定义**：使用枚举定义所有可能的错误情况
- **错误码映射**：为每种错误类型关联对应的错误码
- **自动转换**：支持从标准库错误类型自动转换
- **错误信息**：提供详细的错误描述信息

## 快速开始

### 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
rf-errors = { path = "../rf/errors" }
```

### 基本导入

```rust
use rf_errors::{RfError, Result};
```

## 核心功能

### 错误类型

RF 框架定义了以下错误类型：

```rust
use rf_errors::RfError;

// 内部服务器错误
let err = RfError::Internal("系统内部错误".to_string());

// 参数验证失败
let err = RfError::InvalidParameter("用户名不能为空".to_string());

// 资源未找到
let err = RfError::NotFound("用户不存在".to_string());

// 未授权访问
let err = RfError::Unauthorized("需要登录".to_string());

// 权限不足
let err = RfError::Forbidden("权限不足".to_string());

// 操作超时
let err = RfError::Timeout("请求超时".to_string());

// 数据库操作错误
let err = RfError::Database("数据库连接失败".to_string());

// 网络通信错误
let err = RfError::Network("网络连接失败".to_string());

// 配置错误
let err = RfError::Config("配置文件格式错误".to_string());

// 序列化错误
let err = RfError::Serialization("JSON 解析失败".to_string());

// 数据验证错误
let err = RfError::Validation("邮箱格式不正确".to_string());

// 自定义错误
let err = RfError::Custom("自定义错误消息".to_string());
```

### Result 类型

框架提供了 `Result<T>` 类型别名，等同于 `Result<T, RfError>`。

```rust
use rf_errors::Result;

fn read_config() -> Result<String> {
    // 成功时返回 Ok
    Ok("配置内容".to_string())
    
    // 失败时返回 Err
    // Err(RfError::Config("配置读取失败".to_string()))
}
```

### 基本使用

#### 创建错误

```rust
use rf_errors::{RfError, Result};

fn validate_user(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(RfError::InvalidParameter(
            "用户名不能为空".to_string()
        ));
    }
    Ok(())
}
```

#### 处理错误

```rust
use rf_errors::{RfError, Result};

fn handle_operation() -> Result<String> {
    Err(RfError::NotFound("资源未找到".to_string()))
}

match handle_operation() {
    Ok(data) => println!("操作成功: {}", data),
    Err(e) => {
        eprintln!("错误码: {}", e.code());
        eprintln!("错误信息: {}", e.message());
    }
}
```

#### 错误传播

使用 `?` 运算符自动传播错误：

```rust
use rf_errors::{RfError, Result};

fn inner_function() -> Result<String> {
    Err(RfError::Database("数据库连接失败".to_string()))
}

fn outer_function() -> Result<String> {
    // 使用 ? 运算符传播错误
    inner_function()?;
    Ok("成功".to_string())
}

// 调用
match outer_function() {
    Ok(_) => println!("成功"),
    Err(e) => eprintln!("错误: {}", e.message()),
}
```

### 错误码和消息

每个错误类型都有对应的错误码和消息：

```rust
use rf_errors::RfError;

let err = RfError::NotFound("用户不存在".to_string());

// 获取错误码
let code = err.code();
println!("错误码: {}", code);

// 获取错误消息
let message = err.message();
println!("错误消息: {}", message);
```

### 自动转换

Errors 模块支持从标准库错误类型自动转换：

```rust
use rf_errors::{RfError, Result};
use std::fs;

// 从 std::io::Error 自动转换
fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .map_err(|e| RfError::Io(e))
}
```

## 高级用法

### 错误链

可以创建错误链来保留原始错误信息：

```rust
use rf_errors::{RfError, Result};

fn process_data() -> Result<()> {
    // 模拟底层错误
    let io_err = std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "文件未找到"
    );
    
    // 包装为框架错误
    Err(RfError::Io(io_err))
}

fn high_level_operation() -> Result<()> {
    process_data()
        .map_err(|e| RfError::Internal(
            format!("数据处理失败: {}", e.message())
        ))
}
```

### 自定义错误上下文

```rust
use rf_errors::{RfError, Result};

fn validate_email(email: &str) -> Result<()> {
    if !email.contains('@') {
        return Err(RfError::Validation(
            format!("邮箱格式不正确: {}", email)
        ));
    }
    Ok(())
}

fn create_user(email: &str) -> Result<()> {
    validate_email(email)?;
    // ... 创建用户的逻辑
    Ok(())
}
```

### 错误类型匹配

```rust
use rf_errors::{RfError, Result};

fn handle_error(err: RfError) {
    match err {
        RfError::NotFound(msg) => {
            eprintln!("资源未找到: {}", msg);
        }
        RfError::Database(msg) => {
            eprintln!("数据库错误: {}", msg);
        }
        RfError::InvalidParameter(msg) => {
            eprintln!("参数错误: {}", msg);
        }
        _ => {
            eprintln!("其他错误: {}", err.message());
        }
    }
}
```

### 错误转换

```rust
use rf_errors::{RfError, Result};
use serde_json;

fn parse_json(json_str: &str) -> Result<serde_json::Value> {
    serde_json::from_str(json_str)
        .map_err(|e| RfError::Serialization(
            format!("JSON 解析失败: {}", e)
        ))
}
```

## 最佳实践

### 1. 使用具体的错误类型

```rust
// 好的做法：使用具体的错误类型
fn validate_age(age: i32) -> Result<()> {
    if age < 0 {
        return Err(RfError::InvalidParameter(
            "年龄不能为负数".to_string()
        ));
    }
    Ok(())
}

// 避免：使用过于通用的错误类型
// Err(RfError::Internal("错误".to_string()))
```

### 2. 提供有意义的错误消息

```rust
// 好的做法：提供详细的错误信息
fn find_user(id: u32) -> Result<User> {
    Err(RfError::NotFound(
        format!("用户 ID {} 不存在", id)
    ))
}

// 避免：错误消息过于简单
// Err(RfError::NotFound("未找到".to_string()))
```

### 3. 在适当的位置处理错误

```rust
use rf_errors::{RfError, Result};

// 在业务逻辑层传播错误
fn business_logic() -> Result<()> {
    validate_input()?;
    process_data()?;
    Ok(())
}

// 在应用层处理错误
fn main() {
    match business_logic() {
        Ok(_) => println!("操作成功"),
        Err(e) => {
            eprintln!("错误: {}", e.message());
            // 记录日志、返回 HTTP 错误等
        }
    }
}
```

### 4. 使用错误码进行分类

```rust
use rf_errors::RfError;

fn categorize_error(err: &RfError) -> &str {
    match err.code() {
        code if code >= 400 && code < 500 => "客户端错误",
        code if code >= 500 => "服务器错误",
        _ => "未知错误",
    }
}
```

## API 参考

### 错误类型

- `RfError::Internal(String)` - 内部服务器错误（500）
- `RfError::InvalidParameter(String)` - 参数验证失败（400）
- `RfError::NotFound(String)` - 资源未找到（404）
- `RfError::Unauthorized(String)` - 未授权访问（401）
- `RfError::Forbidden(String)` - 权限不足（403）
- `RfError::Timeout(String)` - 操作超时（408）
- `RfError::Database(String)` - 数据库操作错误（500）
- `RfError::Network(String)` - 网络通信错误（500）
- `RfError::Config(String)` - 配置错误（500）
- `RfError::Io(std::io::Error)` - IO 操作错误（500）
- `RfError::Serialization(String)` - 序列化错误（500）
- `RfError::Validation(String)` - 数据验证错误（400）
- `RfError::Custom(String)` - 自定义错误（500）

### 主要方法

- `code() -> u32` - 获取错误码
- `message() -> String` - 获取错误消息

### 类型别名

- `Result<T>` - `Result<T, RfError>` 的别名

## 常见问题

### Q: 如何创建自定义错误？

A: 使用 `RfError::Custom` 或创建新的错误变体（需要修改 errors 模块）。

### Q: 错误码是如何定义的？

A: 错误码定义在 `code.rs` 模块中，每种错误类型都有对应的错误码常量。

### Q: 如何处理多个可能的错误类型？

A: 使用 `match` 表达式或 `map_err` 进行错误转换。

### Q: 错误消息应该包含哪些信息？

A: 错误消息应该包含：
- 错误发生的原因
- 相关的上下文信息（如 ID、参数值等）
- 可能的解决方案（可选）

## 相关链接

- [core 模块](../core/README.md) - 核心类型
- [util 模块](../util/README.md) - 数据验证工具

