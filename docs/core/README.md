# Core 模块教程

Core 模块是 RF 框架的核心基础模块，提供了框架中使用的核心类型、Trait 和通用接口。

## 模块概述

Core 模块定义了 RF 框架的基础类型系统，包括：

- **类型别名**：为常用的容器类型提供简短的别名（Map、List、Var 等）
- **核心 Trait**：定义对象转换、克隆、比较和哈希的基本接口
- **通用接口**：提供统一的变量类型 Var 用于处理动态数据

## 快速开始

### 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
rf-core = { path = "../rf/core" }
serde_json = "1.0"
```

### 基本导入

```rust
use rf_core::{Map, List, Var, MapStrStr};
use serde_json::json;
```

## 核心功能

### 类型别名

#### Map 类型

Map 是框架中最常用的键值对容器，基于 `HashMap<String, serde_json::Value>`。

```rust
use rf_core::Map;
use serde_json::json;

// 创建 Map
let mut config: Map = Map::new();
config.insert("host".to_string(), json!("localhost"));
config.insert("port".to_string(), json!(8080));
config.insert("debug".to_string(), json!(true));

// 访问值
if let Some(host) = config.get("host") {
    println!("Host: {}", host);
}
```

#### 不同类型的 Map

框架提供了多种 Map 类型别名，适用于不同的场景：

```rust
use rf_core::{MapStrStr, MapStrInt, MapIntStr, MapAnyBool};

// MapStrStr: String -> String
let mut headers: MapStrStr = MapStrStr::new();
headers.insert("Content-Type".to_string(), "application/json".to_string());

// MapStrInt: String -> i64
let mut counters: MapStrInt = MapStrInt::new();
counters.insert("views".to_string(), 1000);
counters.insert("likes".to_string(), 50);

// MapIntStr: i64 -> String
let mut error_messages: MapIntStr = MapIntStr::new();
error_messages.insert(404, "未找到".to_string());
error_messages.insert(500, "服务器错误".to_string());

// MapAnyBool: String -> bool
let mut features: MapAnyBool = MapAnyBool::new();
features.insert("dark_mode".to_string(), true);
features.insert("notifications".to_string(), false);
```

#### List 类型

List 用于存储多个 Map 对象，通常用于表示数据表格或记录集。

```rust
use rf_core::List;
use serde_json::json;

let mut users: List = List::new();

// 创建用户记录
let mut user1 = std::collections::HashMap::new();
user1.insert("id".to_string(), json!(1));
user1.insert("name".to_string(), json!("张三"));
user1.insert("age".to_string(), json!(25));

let mut user2 = std::collections::HashMap::new();
user2.insert("id".to_string(), json!(2));
user2.insert("name".to_string(), json!("李四"));
user2.insert("age".to_string(), json!(30));

users.push(user1);
users.push(user2);

// 遍历列表
for user in &users {
    if let Some(name) = user.get("name") {
        println!("用户: {}", name);
    }
}
```

#### Var 类型

Var 是 `serde_json::Value` 的别名，用于表示任意 JSON 值。

```rust
use rf_core::Var;
use serde_json::json;

// 各种类型的 Var
let null_var: Var = json!(null);
let bool_var: Var = json!(true);
let number_var: Var = json!(42);
let string_var: Var = json!("你好");
let array_var: Var = json!([1, 2, 3]);
let object_var: Var = json!({"key": "value"});

// 访问 Var 的值
if let Some(str_val) = string_var.as_str() {
    println!("字符串值: {}", str_val);
}

if let Some(num) = number_var.as_i64() {
    println!("数字值: {}", num);
}
```

### 核心 Trait

#### ToString Trait

ToString trait 定义了将对象转换为字符串表示的能力。

```rust
use rf_core::traits::ToString;

struct User {
    id: u32,
    username: String,
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!("User[{}]@{}", self.id, self.username)
    }
}

let user = User {
    id: 123,
    username: "alice".to_string(),
};

println!("{}", user.to_string()); // 输出: User[123]@alice
```

#### Clone Trait

Clone trait 表示对象可以被克隆。

```rust
use rf_core::traits::Clone;

#[derive(Clone)]
struct Config {
    host: String,
    port: u16,
}

let config1 = Config {
    host: "localhost".to_string(),
    port: 8080,
};

let config2 = config1.clone(); // 创建深拷贝
```

#### Compare Trait

Compare trait 表示对象可以进行比较。

```rust
use rf_core::traits::Compare;

#[derive(PartialEq, Eq)]
struct UserId(u32);

impl Compare for UserId {}

let id1 = UserId(123);
let id2 = UserId(123);
let id3 = UserId(456);

assert_eq!(id1, id2); // 相等
assert_ne!(id1, id3); // 不相等
```

#### Hash Trait

Hash trait 表示对象可以被哈希，可用作 HashMap 的键。

```rust
use rf_core::traits::Hash;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct Key {
    namespace: String,
    id: u32,
}

let mut map: HashMap<Key, String> = HashMap::new();
let key = Key {
    namespace: "user".to_string(),
    id: 123,
};
map.insert(key, "数据".to_string());
```

## 高级用法

### 类型转换

```rust
use rf_core::Var;
use serde_json::json;

let var: Var = json!(123);

// 转换为不同类型
let as_i64 = var.as_i64();  // Some(123)
let as_f64 = var.as_f64();  // Some(123.0)
let as_str = var.as_str();  // None
```

### 动态数据结构

```rust
use rf_core::{Map, Var};
use serde_json::json;

fn create_dynamic_config() -> Map {
    let mut config: Map = Map::new();
    
    // 嵌套对象
    let mut db_config = std::collections::HashMap::new();
    db_config.insert("host".to_string(), json!("localhost"));
    db_config.insert("port".to_string(), json!(5432));
    config.insert("database".to_string(), json!(db_config));
    
    // 数组
    config.insert("allowed_hosts".to_string(), json!(["localhost", "127.0.0.1"]));
    
    config
}
```

### 与 JSON 互操作

```rust
use rf_core::{Map, Var};
use serde_json::{json, to_string, from_str};

// Map 转 JSON 字符串
let mut map: Map = Map::new();
map.insert("name".to_string(), json!("RF"));
let json_str = to_string(&map).unwrap();
println!("{}", json_str); // {"name":"RF"}

// JSON 字符串转 Var
let json_str = r#"{"key":"value"}"#;
let var: Var = from_str(json_str).unwrap();
```

## API 参考

### 类型别名

- `Map` / `MapAnyAny` / `MapStrAny`: `HashMap<String, Value>`
- `MapStrStr` / `MapAnyStr`: `HashMap<String, String>`
- `MapStrInt` / `MapAnyInt`: `HashMap<String, i64>`
- `MapIntAny`: `HashMap<i64, Value>`
- `MapIntStr`: `HashMap<i64, String>`
- `MapIntInt`: `HashMap<i64, i64>`
- `MapAnyBool` / `MapStrBool`: `HashMap<String, bool>`
- `MapIntBool`: `HashMap<i64, bool>`
- `List` / `ListAnyAny` / `ListStrAny`: `Vec<Map>`
- `ListStrStr` / `ListAnyStr`: `Vec<MapStrStr>`
- `ListStrInt` / `ListAnyInt`: `Vec<MapStrInt>`
- `ListIntAny`: `Vec<MapIntAny>`
- `ListIntStr`: `Vec<MapIntStr>`
- `ListIntInt`: `Vec<MapIntInt>`
- `Var`: `serde_json::Value`

### Trait

- `ToString`: 字符串转换
- `Clone`: 克隆标记
- `Compare`: 比较标记（PartialEq + Eq）
- `Hash`: 哈希标记

## 常见问题

### Q: Map 和 HashMap 有什么区别？

A: Map 是 `HashMap<String, serde_json::Value>` 的类型别名，专门用于存储动态 JSON 数据。它提供了更简洁的命名和统一的接口。

### Q: 什么时候使用 Var？

A: Var 适用于需要处理不确定类型数据的场景，如配置值、API 响应、动态数据结构等。

### Q: 如何选择不同的 Map 类型？

A: 
- 使用 `Map` / `MapStrAny` 处理动态 JSON 数据
- 使用 `MapStrStr` 处理字符串键值对
- 使用 `MapStrInt` 处理计数器或索引
- 使用 `MapIntStr` 处理 ID 到名称的映射

## 相关链接

- [errors 模块](../errors/README.md) - 错误处理
- [container 模块](../container/README.md) - 容器数据结构
- [util 模块](../util/README.md) - 工具函数

