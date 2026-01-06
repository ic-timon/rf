//! # templates
//!
//! templates 模块 - 代码生成模板
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 代码生成模板
//!
//! 本模块提供从数据库架构信息生成 Rust 代码的模板函数，包括：
//! - Model 模板：生成数据模型结构体
//! - DAO 模板：生成数据访问对象
//! - 类型映射：数据库类型到 Rust 类型的映射

use crate::gen::database::TableInfo;

/// 默认 Model 模板生成函数
///
/// 根据表信息生成 Rust Model 结构体的代码
///
/// # 参数
///
/// * `table` - 表信息（包含列、类型、注释等）
/// * `struct_name` - 生成的结构体名称
///
/// # 返回
///
/// 返回生成的 Rust 代码字符串
///
/// # 生成的代码特性
///
/// - derive 宏：Debug, Clone, Serialize, Deserialize, FromRow
/// - 包含字段的数据库注释（如果有）
/// - 自动映射数据库类型到 Rust 类型
/// - 可空字段使用 Option<T>
///
/// # 示例
///
/// ```rust
/// let table = get_table_info("users").await?;
/// let code = model_template(&table, "User");
/// println!("{}", code);
/// ```
pub fn model_template(table: &TableInfo, struct_name: &str) -> String {
    let mut code = String::new();
    
    // Add imports
    code.push_str("use serde::{Deserialize, Serialize};\n");
    code.push_str("use sqlx::FromRow;\n\n");
    
    // Add struct definition
    code.push_str(&format!("/// {} model\n", table.name));
    code.push_str("#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]\n");
    code.push_str(&format!("pub struct {} {{\n", struct_name));
    
    // Add fields
    for column in &table.columns {
        let rust_type = map_db_type_to_rust(&column.data_type, column.is_nullable);
        let field_name = to_snake_case(&column.name);
        
        if let Some(ref comment) = column.comment {
            code.push_str(&format!("    /// {}\n", comment));
        }
        
        code.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
    }
    
    code.push_str("}\n");
    
    code
}

/// 默认 DAO 模板生成函数
///
/// 根据表信息生成 Rust DAO 结构体的代码
///
/// # 参数
///
/// * `table` - 表信息
/// * `_struct_name` - 对应的 Model 结构体名称（当前未使用，保留用于未来扩展）
/// * `dao_name` - 生成的 DAO 结构体名称
///
/// # 返回
///
/// 返回生成的 Rust 代码字符串
///
/// # 生成的代码特性
///
/// - 包含 model() 方法用于获取 Model 实例
/// - 基础的数据库访问抽象
///
/// # 示例
///
/// ```rust
/// let table = get_table_info("users").await?;
/// let code = dao_template(&table, "User", "UserDao");
/// println!("{}", code);
/// ```
pub fn dao_template(table: &TableInfo, _struct_name: &str, dao_name: &str) -> String {
    let mut code = String::new();
    
    code.push_str("use rf_database::db::{Database, Model};\n");
    code.push_str("use rf_errors::Result;\n\n");
    
    code.push_str(&format!("/// {} DAO\n", table.name));
    code.push_str(&format!("pub struct {};\n\n", dao_name));
    
    code.push_str(&format!("impl {} {{\n", dao_name));
    code.push_str(&format!("    /// Get model for {}\n", table.name));
    code.push_str("    pub fn model(db: &Database) -> Model {\n");
    code.push_str(&format!("        db.model(\"{}\")\n", table.name));
    code.push_str("    }\n");
    code.push_str("}\n");
    
    code
}

/// 映射数据库类型到 Rust 类型
///
/// 根据数据库列类型和可空性生成对应的 Rust 类型
///
/// # 参数
///
/// * `db_type` - 数据库类型名称（如 "varchar", "integer" 等）
/// * `is_nullable` - 列是否可为空
///
/// # 返回
///
/// 返回对应的 Rust 类型字符串，可空类型返回 `Option<T>`
///
/// # 支持的类型映射
///
/// ## 整数类型
/// - `integer`, `int`, `int4`, `serial` -> `i32`
/// - `bigint`, `int8`, `bigserial` -> `i64`
/// - `smallint`, `int2` -> `i16`
///
/// ## 浮点类型
/// - `real`, `float4` -> `f32`
/// - `double precision`, `float8`, `numeric`, `decimal` -> `f64`
///
/// ## 其他类型
/// - `boolean`, `bool` -> `bool`
/// - `text`, `varchar`, `char` -> `String`
/// - `timestamp`, `timestamptz` -> `chrono::DateTime<chrono::Utc>`
/// - `date` -> `chrono::NaiveDate`
/// - `time` -> `chrono::NaiveTime`
/// - `uuid` -> `uuid::Uuid`
/// - `json`, `jsonb` -> `serde_json::Value`
/// - `bytea` -> `Vec<u8>`
///
/// # 示例
///
/// ```rust
/// map_db_type_to_rust("varchar", false) -> "String"
/// map_db_type_to_rust("integer", true) -> "Option<i32>"
/// map_db_type_to_rust("timestamp", false) -> "chrono::DateTime<chrono::Utc>"
/// ```
fn map_db_type_to_rust(db_type: &str, is_nullable: bool) -> String {
    let base_type = match db_type.to_lowercase().as_str() {
        "integer" | "int" | "int4" | "serial" => "i32",
        "bigint" | "int8" | "bigserial" => "i64",
        "smallint" | "int2" => "i16",
        "real" | "float4" => "f32",
        "double precision" | "float8" | "numeric" | "decimal" => "f64",
        "boolean" | "bool" => "bool",
        "text" | "varchar" | "character varying" | "char" => "String",
        "timestamp" | "timestamp without time zone" | "timestamptz" | "timestamp with time zone" => "chrono::DateTime<chrono::Utc>",
        "date" => "chrono::NaiveDate",
        "time" => "chrono::NaiveTime",
        "uuid" => "uuid::Uuid",
        "json" | "jsonb" => "serde_json::Value",
        "bytea" => "Vec<u8>",
        _ => "String", // Default to String for unknown types
    };
    
    if is_nullable {
        format!("Option<{}>", base_type)
    } else {
        base_type.to_string()
    }
}

/// 转换为 snake_case 命名
///
/// 将字符串转换为蛇形命名法（snake_case）
///
/// # 参数
///
/// * `s` - 输入字符串
///
/// # 返回
///
/// 返回 snake_case 格式的字符串
///
/// # 规则
///
/// - 在大写字母前添加下划线（除了第一个字符）
/// - 将所有字母转换为小写
///
/// # 示例
///
/// ```rust
/// to_snake_case("UserName") -> "user_name"
/// to_snake_case("User") -> "user"
/// to_snake_case("userID") -> "user_i_d"
/// ```
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_upper = false;
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_upper = true;
        } else {
            result.push(c);
            prev_upper = false;
        }
    }
    
    result
}

/// 转换为 PascalCase 命名
///
/// 将字符串转换为帕斯卡命名法（PascalCase）
///
/// # 参数
///
/// * `s` - 输入字符串（通常为 snake_case）
///
/// # 返回
///
/// 返回 PascalCase 格式的字符串
///
/// # 规则
///
/// - 按下划线分割字符串
/// - 每个单词首字母大写，其余字母小写
/// - 移除所有下划线
///
/// # 示例
///
/// ```rust
/// to_pascal_case("user_name") -> "UserName"
/// to_pascal_case("user_profile") -> "UserProfile"
/// to_pascal_case("id") -> "Id"
/// ```
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// 转换为 camelCase 命名
///
/// 将字符串转换为驼峰命名法（camelCase）
///
/// # 参数
///
/// * `s` - 输入字符串（通常为 snake_case）
///
/// # 返回
///
/// 返回 camelCase 格式的字符串
///
/// # 规则
///
/// - 先转换为 PascalCase
/// - 将首字母转换为小写
///
/// # 示例
///
/// ```rust
/// to_camel_case("user_name") -> "userName"
/// to_camel_case("user_profile") -> "userProfile"
/// to_camel_case("id") -> "id"
/// ```
pub fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    if pascal.is_empty() {
        return pascal;
    }
    let mut chars = pascal.chars();
    chars.next().unwrap().to_lowercase().collect::<String>() + chars.as_str()
}

