//! # types
//!
//! types 模块 - 核心类型定义和别名
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 核心类型定义和别名
//!
//! 该模块定义了 RF 框架中使用的各种类型别名，主要包括：
//!
//! - **Map 系列**: 基于 HashMap 的键值对容器，支持不同的键值类型组合
//! - **List 系列**: 基于 Vec 的列表容器，用于存储多个 Map 对象
//! - **Var**: 通用变量类型，基于 serde_json::Value
//!
//! # 类型命名规范
//!
//! 类型名称采用 `Map<KeyType><ValueType>` 或 `List<KeyType><ValueType>` 的格式：
//! - `Any`: 表示任意 JSON 值 (serde_json::Value)
//! - `Str`: 表示字符串 (String)
//! - `Int`: 表示整数 (i64)
//! - `Bool`: 表示布尔值 (bool)
//!
//! # 使用示例
//!
//! ```rust
//! use rf_core::{Map, MapStrStr, List, Var};
//! use serde_json::json;
//!
//! // 创建一个 Map (String -> Value)
//! let mut map: Map = Map::new();
//! map.insert("name".to_string(), json!("张三"));
//! map.insert("age".to_string(), json!(25));
//!
//! // 创建一个 MapStrStr (String -> String)
//! let mut str_map: MapStrStr = MapStrStr::new();
//! str_map.insert("city".to_string(), "北京".to_string());
//!
//! // 创建一个 List
//! let list: List = vec![map];
//!
//! // 使用 Var 类型
//! let var: Var = json!({"key": "value"});
//! ```

use serde_json::Value;

/// Map 类型别名 - 最常用的键值对容器
///
/// 这是一个从 String 到 serde_json::Value 的 HashMap，用于存储键值对数据。
/// Value 可以是任意 JSON 类型的值（对象、数组、字符串、数字、布尔值或 null）。
///
/// # 类型说明
///
/// - 键 (Key): `String` - 字符串类型的键
/// - 值 (Value): `Value` - 任意 JSON 值
///
/// # 使用场景
///
/// - 存储配置信息
/// - 传递动态数据结构
/// - API 请求和响应数据
///
/// # 示例
///
/// ```rust
/// use rf_core::Map;
/// use serde_json::json;
///
/// let mut config: Map = Map::new();
/// config.insert("host".to_string(), json!("localhost"));
/// config.insert("port".to_string(), json!(8080));
/// config.insert("debug".to_string(), json!(true));
/// ```
pub type Map = std::collections::HashMap<String, Value>;

/// MapAnyAny 类型别名 - String 到 Value 的映射
///
/// 与 Map 相同，这是最通用的键值对容器类型。
///
/// # 使用示例
///
/// ```rust
/// use rf_core::MapAnyAny;
/// use serde_json::json;
///
/// let mut data: MapAnyAny = MapAnyAny::new();
/// data.insert("name".to_string(), json!("产品"));
/// data.insert("price".to_string(), json!(99.99));
/// ```
pub type MapAnyAny = std::collections::HashMap<String, Value>;

/// MapAnyStr 类型别名 - String 到 String 的映射
///
/// 专门用于存储字符串到字符串的映射关系。
///
/// # 使用场景
///
/// - 存储标签和元数据
/// - 存储配置字符串
/// - 存储国际化文本
///
/// # 示例
///
/// ```rust
/// use rf_core::MapAnyStr;
///
/// let mut labels: MapAnyStr = MapAnyStr::new();
/// labels.insert("title".to_string(), "用户管理".to_string());
/// labels.insert("description".to_string(), "管理系统用户".to_string());
/// ```
pub type MapAnyStr = std::collections::HashMap<String, String>;

/// MapAnyInt 类型别名 - String 到 i64 的映射
///
/// 用于存储字符串键到整数值的映射。
///
/// # 使用场景
///
/// - 统计计数器
/// - 索引映射
/// - 数值配置
///
/// # 示例
///
/// ```rust
/// use rf_core::MapAnyInt;
///
/// let mut counters: MapAnyInt = MapAnyInt::new();
/// counters.insert("views".to_string(), 1000);
/// counters.insert("likes".to_string(), 50);
/// ```
pub type MapAnyInt = std::collections::HashMap<String, i64>;

/// MapStrAny 类型别名 - String 到 Value 的映射
///
/// 与 Map 相同，提供语义化的命名以强调字符串键的特点。
pub type MapStrAny = std::collections::HashMap<String, Value>;

/// MapStrStr 类型别名 - String 到 String 的映射
///
/// 与 MapAnyStr 相同，提供语义化的命名。
///
/// # 使用示例
///
/// ```rust
/// use rf_core::MapStrStr;
///
/// let mut headers: MapStrStr = MapStrStr::new();
/// headers.insert("Content-Type".to_string(), "application/json".to_string());
/// headers.insert("Authorization".to_string(), "Bearer token".to_string());
/// ```
pub type MapStrStr = std::collections::HashMap<String, String>;

/// MapStrInt 类型别名 - String 到 i64 的映射
///
/// 与 MapAnyInt 相同，提供语义化的命名。
pub type MapStrInt = std::collections::HashMap<String, i64>;

/// MapIntAny 类型别名 - i64 到 Value 的映射
///
/// 使用整数作为键的映射容器。
///
/// # 使用场景
///
/// - 索引到数据的映射
/// - ID 到对象的映射
/// - 有序键值对存储
///
/// # 示例
///
/// ```rust
/// use rf_core::MapIntAny;
/// use serde_json::json;
///
/// let mut indexed_data: MapIntAny = MapIntAny::new();
/// indexed_data.insert(1, json!("第一项"));
/// indexed_data.insert(2, json!("第二项"));
/// ```
pub type MapIntAny = std::collections::HashMap<i64, Value>;

/// MapIntStr 类型别名 - i64 到 String 的映射
///
/// 使用整数作为键，字符串作为值的映射。
///
/// # 使用场景
///
/// - ID 到名称的映射
/// - 错误代码到消息的映射
///
/// # 示例
///
/// ```rust
/// use rf_core::MapIntStr;
///
/// let mut error_messages: MapIntStr = MapIntStr::new();
/// error_messages.insert(404, "未找到".to_string());
/// error_messages.insert(500, "服务器错误".to_string());
/// ```
pub type MapIntStr = std::collections::HashMap<i64, String>;

/// MapIntInt 类型别名 - i64 到 i64 的映射
///
/// 整数到整数的映射容器。
///
/// # 使用场景
///
/// - 数据映射和转换
/// - 统计数据
///
/// # 示例
///
/// ```rust
/// use rf_core::MapIntInt;
///
/// let mut mapping: MapIntInt = MapIntInt::new();
/// mapping.insert(1, 100);
/// mapping.insert(2, 200);
/// ```
pub type MapIntInt = std::collections::HashMap<i64, i64>;

/// MapAnyBool 类型别名 - String 到 bool 的映射
///
/// 字符串键到布尔值的映射。
///
/// # 使用场景
///
/// - 功能开关配置
/// - 权限标记
/// - 状态标志
///
/// # 示例
///
/// ```rust
/// use rf_core::MapAnyBool;
///
/// let mut features: MapAnyBool = MapAnyBool::new();
/// features.insert("dark_mode".to_string(), true);
/// features.insert("notifications".to_string(), false);
/// ```
pub type MapAnyBool = std::collections::HashMap<String, bool>;

/// MapStrBool 类型别名 - String 到 bool 的映射
///
/// 与 MapAnyBool 相同，提供语义化的命名。
pub type MapStrBool = std::collections::HashMap<String, bool>;

/// MapIntBool 类型别名 - i64 到 bool 的映射
///
/// 整数键到布尔值的映射。
///
/// # 使用场景
///
/// - 索引到状态标志的映射
/// - ID 到启用状态的映射
///
/// # 示例
///
/// ```rust
/// use rf_core::MapIntBool;
///
/// let mut status: MapIntBool = MapIntBool::new();
/// status.insert(1, true);
/// status.insert(2, false);
/// ```
pub type MapIntBool = std::collections::HashMap<i64, bool>;

/// List 类型别名 - Map 的列表容器
///
/// 用于存储多个 Map 对象的向量，是最常用的列表类型。
///
/// # 使用场景
///
/// - 数据表格和记录集
/// - API 返回的数据列表
/// - 批量数据处理
///
/// # 示例
///
/// ```rust
/// use rf_core::List;
/// use serde_json::json;
///
/// let mut users: List = List::new();
///
/// let mut user1 = std::collections::HashMap::new();
/// user1.insert("id".to_string(), json!(1));
/// user1.insert("name".to_string(), json!("张三"));
///
/// let mut user2 = std::collections::HashMap::new();
/// user2.insert("id".to_string(), json!(2));
/// user2.insert("name".to_string(), json!("李四"));
///
/// users.push(user1);
/// users.push(user2);
/// ```
pub type List = Vec<Map>;

/// ListAnyAny 类型别名 - MapAnyAny 的列表
///
/// 存储多个 MapAnyAny 对象的向量。
pub type ListAnyAny = Vec<MapAnyAny>;

/// ListAnyStr 类型别名 - MapAnyStr 的列表
///
/// 存储多个字符串映射的向量。
///
/// # 使用示例
///
/// ```rust
/// use rf_core::ListAnyStr;
///
/// let mut tags: ListAnyStr = ListAnyStr::new();
///
/// let mut tag1 = std::collections::HashMap::new();
/// tag1.insert("key".to_string(), "value1".to_string());
///
/// tags.push(tag1);
/// ```
pub type ListAnyStr = Vec<MapAnyStr>;

/// ListAnyInt 类型别名 - MapAnyInt 的列表
///
/// 存储多个整数映射的向量。
pub type ListAnyInt = Vec<MapAnyInt>;

/// ListStrAny 类型别名 - MapStrAny 的列表
///
/// 与 List 相同，提供语义化的命名。
pub type ListStrAny = Vec<MapStrAny>;

/// ListStrStr 类型别名 - MapStrStr 的列表
///
/// 存储多个字符串到字符串映射的向量。
///
/// # 使用场景
///
/// - 配置项列表
/// - 标签组列表
///
/// # 示例
///
/// ```rust
/// use rf_core::ListStrStr;
///
/// let mut headers_list: ListStrStr = ListStrStr::new();
///
/// let mut headers = std::collections::HashMap::new();
/// headers.insert("Content-Type".to_string(), "text/html".to_string());
///
/// headers_list.push(headers);
/// ```
pub type ListStrStr = Vec<MapStrStr>;

/// ListStrInt 类型别名 - MapStrInt 的列表
///
/// 存储多个字符串到整数映射的向量。
pub type ListStrInt = Vec<MapStrInt>;

/// ListIntAny 类型别名 - MapIntAny 的列表
///
/// 存储多个整数键映射的向量。
pub type ListIntAny = Vec<MapIntAny>;

/// ListIntStr 类型别名 - MapIntStr 的列表
///
/// 存储多个整数到字符串映射的向量。
pub type ListIntStr = Vec<MapIntStr>;

/// ListIntInt 类型别名 - MapIntInt 的列表
///
/// 存储多个整数到整数映射的向量。
pub type ListIntInt = Vec<MapIntInt>;

/// Var 类型别名 - 通用变量接口
///
/// 这是 serde_json::Value 的别名，用于表示任意 JSON 值。
/// 它是框架中最通用的动态类型，可以表示：
///
/// - null (空值)
/// - bool (布尔值)
/// - number (数字：整数或浮点数)
/// - string (字符串)
/// - array (数组)
/// - object (对象)
///
/// # 使用场景
///
/// - 动态数据处理
/// - 配置值存储
/// - API 数据传输
/// - 不确定类型的变量
///
/// # 示例
///
/// ```rust
/// use rf_core::Var;
/// use serde_json::json;
///
/// // 各种类型的 Var
/// let null_var: Var = json!(null);
/// let bool_var: Var = json!(true);
/// let number_var: Var = json!(42);
/// let string_var: Var = json!("你好");
/// let array_var: Var = json!([1, 2, 3]);
/// let object_var: Var = json!({"key": "value"});
///
/// // 访问 Var 的值
/// if let Some(str_val) = string_var.as_str() {
///     println!("字符串值: {}", str_val);
/// }
/// ```
///
/// # 类型转换
///
/// ```rust
/// use rf_core::Var;
/// use serde_json::json;
///
/// let var: Var = json!(123);
///
/// // 转换为不同类型
/// let as_i64 = var.as_i64();  // Some(123)
/// let as_str = var.as_str();  // None
/// let as_f64 = var.as_f64();  // Some(123.0)
/// ```
pub type Var = Value;

