# Util 模块教程

Util 模块提供了各种工具函数，包括类型转换、数据验证、随机数生成、GUID 生成等。

## 模块概述

Util 模块包含以下功能：

- **类型转换（conv）**：各种类型之间的转换
- **数据验证（valid）**：数据验证规则和验证器
- **随机数生成（rand）**：随机整数和浮点数
- **GUID 生成（guid）**：全局唯一标识符
- **分页工具（page）**：分页数据处理
- **元数据处理（meta）**：元数据操作
- **标签处理（tag）**：标签解析和处理

## 快速开始

### 添加依赖

```toml
[dependencies]
rf-util = { path = "../rf/util" }
```

### 基本导入

```rust
use rf_util::{int, int_range, float, float_range};
use rf_util::valid::{Validator, Rule};
use rf_util::guid;
```

## 核心功能

### 类型转换

#### 基本类型转换

```rust
use rf_util::conv;

// 字符串转整数
let num: i64 = conv::string_to_int("123").unwrap();
println!("{}", num); // 123

// 字符串转浮点数
let num: f64 = conv::string_to_float("3.14").unwrap();
println!("{}", num); // 3.14

// 整数转字符串
let str = conv::int_to_string(123);
println!("{}", str); // "123"

// 浮点数转字符串
let str = conv::float_to_string(3.14);
println!("{}", str); // "3.14"
```

#### 时间转换

```rust
use rf_util::conv;
use std::time::SystemTime;

// 时间戳转字符串
let timestamp = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs();
let time_str = conv::timestamp_to_string(timestamp);
println!("{}", time_str);

// 字符串转时间戳
let timestamp = conv::string_to_timestamp("2024-01-01 00:00:00").unwrap();
```

### 数据验证

#### 基本验证

```rust
use rf_util::valid::{Validator, Rule};
use std::collections::HashMap;

// 创建验证器
let validator = Validator::new()
    .rule(Rule::new("email", "required", vec![]))
    .rule(Rule::new("email", "email", vec![]))
    .rule(Rule::new("age", "between", vec!["18".to_string(), "100".to_string()]))
    .rule(Rule::new("password", "min_length", vec!["8".to_string()]));

// 准备数据
let mut data = HashMap::new();
data.insert("email".to_string(), "user@example.com".to_string());
data.insert("age".to_string(), "25".to_string());
data.insert("password".to_string(), "securepass123".to_string());

// 执行验证
match validator.validate_map(&data) {
    Ok(_) => println!("验证通过"),
    Err(e) => println!("验证失败: {}", e),
}
```

#### 验证规则

```rust
use rf_util::valid::Rule;

// required - 必填
let rule = Rule::new("name", "required", vec![]);

// email - 邮箱格式
let rule = Rule::new("email", "email", vec![]);

// min_length - 最小长度
let rule = Rule::new("password", "min_length", vec!["8".to_string()]);

// max_length - 最大长度
let rule = Rule::new("username", "max_length", vec!["20".to_string()]);

// between - 范围
let rule = Rule::new("age", "between", vec!["18".to_string(), "100".to_string()]);

// regex - 正则表达式
let rule = Rule::new("phone", "regex", vec![r"^\d{11}$".to_string()]);
```

#### 结构体验证

```rust
use rf_util::valid::{Validator, Rule};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: String,
    age: i32,
}

let user = User {
    email: "user@example.com".to_string(),
    age: 25,
};

let validator = Validator::new()
    .rule(Rule::new("email", "required", vec![]))
    .rule(Rule::new("email", "email", vec![]))
    .rule(Rule::new("age", "between", vec!["18".to_string(), "100".to_string()]));

match validator.validate_struct(&user) {
    Ok(_) => println!("验证通过"),
    Err(e) => println!("验证失败: {}", e),
}
```

### 随机数生成

```rust
use rf_util::{int, int_range, float, float_range};

// 生成随机整数（0 到 i64::MAX）
let random_int = int();

// 生成指定范围的随机整数
let random_in_range = int_range(1, 100);

// 生成随机浮点数（0.0 到 1.0）
let random_float = float();

// 生成指定范围的随机浮点数
let random_float_in_range = float_range(1.0, 100.0);

println!("随机整数: {}", random_int);
println!("范围随机整数: {}", random_in_range);
println!("随机浮点数: {}", random_float);
println!("范围随机浮点数: {}", random_float_in_range);
```

### GUID 生成

```rust
use rf_util::guid;

// 生成 GUID
let id = guid::new();
println!("GUID: {}", id);

// 生成短 GUID
let short_id = guid::short();
println!("短 GUID: {}", short_id);
```

### 分页工具

```rust
use rf_util::page::{Page, Paginator};

// 创建分页器
let paginator = Paginator::new(100, 10); // 总数 100，每页 10 条

// 获取第 2 页的数据
let page = paginator.page(2);

println!("当前页: {}", page.current());
println!("总页数: {}", page.total_pages());
println!("每页数量: {}", page.per_page());
println!("总数: {}", page.total());
```

### 元数据处理

```rust
use rf_util::meta;

// 获取类型名称
let type_name = meta::type_name::<i32>();
println!("类型名称: {}", type_name);

// 获取函数名称（在宏中使用）
// let func_name = meta::function_name!();
```

### 标签处理

```rust
use rf_util::tag;

// 解析标签字符串
let tags_str = "rust,web,framework";
let tags = tag::parse(tags_str);

for tag in tags {
    println!("标签: {}", tag);
}

// 合并标签
let tags1 = vec!["rust".to_string(), "web".to_string()];
let tags2 = vec!["framework".to_string()];
let merged = tag::merge(&tags1, &tags2);
```

## 高级用法

### 自定义验证规则

```rust
use rf_util::valid::{Validator, Rule, ValidationError};

fn custom_validation(value: &str) -> Result<(), ValidationError> {
    if value.starts_with("RF-") {
        Ok(())
    } else {
        Err(ValidationError::new("必须以 RF- 开头"))
    }
}

let validator = Validator::new()
    .rule(Rule::new("code", "custom", vec![]));
```

### 批量验证

```rust
use rf_util::valid::{Validator, Rule};
use std::collections::HashMap;

let validator = Validator::new()
    .rule(Rule::new("email", "required", vec![]))
    .rule(Rule::new("email", "email", vec![]));

let mut data_list = Vec::new();
for i in 0..10 {
    let mut data = HashMap::new();
    data.insert("email".to_string(), format!("user{}@example.com", i));
    data_list.push(data);
}

// 批量验证
for (index, data) in data_list.iter().enumerate() {
    match validator.validate_map(data) {
        Ok(_) => println!("记录 {} 验证通过", index),
        Err(e) => println!("记录 {} 验证失败: {}", index, e),
    }
}
```

### 验证器链

```rust
use rf_util::valid::{Validator, Rule};

// 创建多个验证器
let email_validator = Validator::new()
    .rule(Rule::new("email", "required", vec![]))
    .rule(Rule::new("email", "email", vec![]));

let password_validator = Validator::new()
    .rule(Rule::new("password", "required", vec![]))
    .rule(Rule::new("password", "min_length", vec!["8".to_string()]));

// 依次验证
// email_validator.validate(...)?;
// password_validator.validate(...)?;
```

## API 参考

### 类型转换函数

- `string_to_int(s: &str) -> Result<i64>` - 字符串转整数
- `string_to_float(s: &str) -> Result<f64>` - 字符串转浮点数
- `int_to_string(n: i64) -> String` - 整数转字符串
- `float_to_string(n: f64) -> String` - 浮点数转字符串

### 验证相关

- `Validator::new() -> Self` - 创建验证器
- `Validator::rule(rule: Rule) -> Self` - 添加验证规则
- `Validator::validate_map(data: &HashMap) -> Result<()>` - 验证映射数据
- `Rule::new(field: &str, rule: &str, params: Vec<String>) -> Self` - 创建验证规则

### 随机数函数

- `int() -> i64` - 生成随机整数
- `int_range(min: i64, max: i64) -> i64` - 生成范围随机整数
- `float() -> f64` - 生成随机浮点数
- `float_range(min: f64, max: f64) -> f64` - 生成范围随机浮点数

### GUID 函数

- `guid::new() -> String` - 生成 GUID
- `guid::short() -> String` - 生成短 GUID

## 常见问题

### Q: 如何添加自定义验证规则？

A: 使用 `Rule::new` 创建规则时，rule 参数可以是内置规则名称或自定义规则名称，然后实现对应的验证逻辑。

### Q: 验证器是线程安全的吗？

A: Validator 本身不是线程安全的，但可以在多线程环境中为每个线程创建独立的验证器实例。

### Q: 如何验证嵌套结构？

A: 使用结构体验证功能，或手动展开嵌套结构进行验证。

## 相关链接

- [errors 模块](../errors/README.md) - 错误处理
- [text 模块](../text/README.md) - 文本处理

