//! # generator
//!
//! generator 模块 - 代码生成引擎
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 代码生成引擎
//!
//! 本模块提供从数据库架构生成 Rust 代码的功能，包括：
//! - Model: 数据模型结构体
//! - DAO: 数据访问对象
//! - 支持自定义命名风格和过滤选项

use crate::gen::database::SchemaInspector;
use crate::gen::templates::{model_template, dao_template, to_pascal_case, to_camel_case};
use rf_errors::Result;
use std::fs;
use std::path::PathBuf;

/// 代码生成选项
///
/// 控制代码生成行为的各种配置选项
#[derive(Debug, Clone)]
pub struct GenOptions {
    /// 输出目录
    pub output_dir: PathBuf,
    /// Model 文件输出目录（可选，默认为 output_dir/model）
    pub model_dir: Option<PathBuf>,
    /// DAO 文件输出目录（可选，默认为 output_dir/dao）
    pub dao_dir: Option<PathBuf>,
    /// 表名前缀过滤（只生成以此前缀开头的表）
    pub table_prefix: Option<String>,
    /// 表名后缀过滤（只生成以此后缀结尾的表）
    pub table_suffix: Option<String>,
    /// 要排除的表名列表
    pub exclude_tables: Vec<String>,
    /// 要生成的表名列表（为空则生成所有表）
    pub only_tables: Vec<String>,
    /// Schema 名称（PostgreSQL）
    pub schema: Option<String>,
    /// 命名风格
    pub naming_style: NamingStyle,
}

/// 命名风格枚举
///
/// 定义生成代码的命名规范
#[derive(Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
pub enum NamingStyle {
    /// 帕斯卡命名法（PascalCase）
    /// 例如：UserName, UserProfile
    PascalCase,
    /// 蛇形命名法（snake_case）
    /// 例如：user_name, user_profile
    /// 
    /// Note: Currently unused, reserved for future implementation
    #[allow(dead_code)] // Reserved for future use
    SnakeCase,
    /// 驼峰命名法（camelCase）
    /// 例如：userName, userProfile
    /// 
    /// Note: Currently unused, reserved for future implementation
    #[allow(dead_code)] // Reserved for future use
    CamelCase,
}

impl Default for GenOptions {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("generated"),
            model_dir: None,
            dao_dir: None,
            table_prefix: None,
            table_suffix: None,
            exclude_tables: Vec::new(),
            only_tables: Vec::new(),
            schema: None,
            naming_style: NamingStyle::PascalCase,
        }
    }
}

/// 代码生成器
///
/// 从数据库架构生成 Rust 代码的核心组件
///
/// # 功能
///
/// - 生成 Model 结构体
/// - 生成 DAO 结构体
/// - 支持表名过滤和转换
/// - 支持自定义命名风格
pub struct CodeGenerator {
    /// 数据库架构检查器
    inspector: SchemaInspector,
    /// 生成选项配置
    options: GenOptions,
}

impl CodeGenerator {
    /// 创建新的代码生成器
    ///
    /// # 参数
    ///
    /// * `inspector` - 数据库架构检查器实例
    /// * `options` - 代码生成选项配置
    ///
    /// # 返回
    ///
    /// 返回 CodeGenerator 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// let inspector = SchemaInspector::new(database);
    /// let options = GenOptions {
    ///     output_dir: PathBuf::from("src/models"),
    ///     naming_style: NamingStyle::PascalCase,
    ///     ..Default::default()
    /// };
    /// let generator = CodeGenerator::new(inspector, options);
    /// ```
    pub fn new(inspector: SchemaInspector, options: GenOptions) -> Self {
        Self { inspector, options }
    }

    /// 为所有表生成代码
    ///
    /// 根据配置选项过滤表，并为每个符合条件的表生成 Model 和 DAO 代码
    ///
    /// # 过滤逻辑
    ///
    /// 1. 表名前缀过滤：只生成以此前缀开头的表（如果指定）
    /// 2. 表名后缀过滤：只生成以此后缀结尾的表（如果指定）
    /// 3. 排除列表：跳过排除列表中的表
    /// 4. 包含列表：只生成包含列表中的表（如果非空）
    ///
    /// # 返回
    ///
    /// 成功返回 Ok(())，失败返回错误
    ///
    /// # 错误
    ///
    /// - 如果无法获取表列表
    /// - 如果无法获取表信息
    /// - 如果无法生成或写入文件
    pub async fn generate_all(&self) -> Result<()> {
        let tables = self.inspector.get_tables(self.options.schema.as_deref()).await?;
        
        // Filter tables
        let filtered_tables: Vec<String> = tables
            .into_iter()
            .filter(|table| {
                // Check prefix
                if let Some(ref prefix) = self.options.table_prefix {
                    if !table.starts_with(prefix) {
                        return false;
                    }
                }
                
                // Check suffix
                if let Some(ref suffix) = self.options.table_suffix {
                    if !table.ends_with(suffix) {
                        return false;
                    }
                }
                
                // Check exclude list
                if self.options.exclude_tables.contains(table) {
                    return false;
                }
                
                // Check only list
                if !self.options.only_tables.is_empty() && !self.options.only_tables.contains(table) {
                    return false;
                }
                
                true
            })
            .collect();

        // Generate code for each table
        for table_name in filtered_tables {
            self.generate_table(&table_name).await?;
        }

        Ok(())
    }

    /// 为指定表生成代码
    ///
    /// 获取表信息并生成对应的 Model 和 DAO 代码文件
    ///
    /// # 参数
    ///
    /// * `table_name` - 要生成代码的表名
    ///
    /// # 返回
    ///
    /// 成功返回 Ok(())，失败返回错误
    ///
    /// # 生成的文件
    ///
    /// - Model 文件：`{model_dir}/{table_name}.rs`
    /// - DAO 文件：`{dao_dir}/{table_name}_dao.rs`
    ///
    /// # 示例
    ///
    /// ```rust
    /// // 为 users 表生成代码
    /// generator.generate_table("users").await?;
    /// // 生成：model/user.rs 和 dao/user_dao.rs
    /// ```
    pub async fn generate_table(&self, table_name: &str) -> Result<()> {
        let table_info = self.inspector
            .get_table_info(table_name, self.options.schema.as_deref())
            .await?;

        // Generate struct name
        let struct_name = self.generate_struct_name(&table_info.name);
        let dao_name = format!("{}Dao", struct_name);

        // Generate model code
        let model_code = model_template(&table_info, &struct_name);
        
        // Generate DAO code
        let dao_code = dao_template(&table_info, &struct_name, &dao_name);

        // Determine output directories
        let model_dir = self.options.model_dir
            .clone()
            .unwrap_or_else(|| self.options.output_dir.join("model"));
        let dao_dir = self.options.dao_dir
            .clone()
            .unwrap_or_else(|| self.options.output_dir.join("dao"));

        // Create directories
        fs::create_dir_all(&model_dir)?;
        fs::create_dir_all(&dao_dir)?;

        // Write model file
        let model_file = model_dir.join(format!("{}.rs", to_snake_case(&struct_name)));
        fs::write(&model_file, model_code)?;
        println!("Generated model: {}", model_file.display());

        // Write DAO file
        let dao_file = dao_dir.join(format!("{}_dao.rs", to_snake_case(&struct_name)));
        fs::write(&dao_file, dao_code)?;
        println!("Generated DAO: {}", dao_file.display());

        Ok(())
    }

    /// 从表名生成结构体名称
    ///
    /// 根据配置的命名规则和表名前缀/后缀处理，生成对应的 Rust 结构体名称
    ///
    /// # 参数
    ///
    /// * `table_name` - 数据库表名
    ///
    /// # 返回
    ///
    /// 返回处理后的结构体名称
    ///
    /// # 处理步骤
    ///
    /// 1. 移除表名前缀（如果配置了 `table_prefix`）
    /// 2. 移除表名后缀（如果配置了 `table_suffix`）
    /// 3. 根据命名风格转换（PascalCase/CamelCase/SnakeCase）
    ///
    /// # 示例
    ///
    /// ```rust
    /// // 假设 table_prefix = "app_"
    /// // 表名: "app_users" -> "Users" (PascalCase)
    /// // 表名: "app_user_profiles" -> "UserProfiles" (PascalCase)
    /// ```
    fn generate_struct_name(&self, table_name: &str) -> String {
        // Remove prefix if specified
        let mut name = table_name.to_string();
        if let Some(ref prefix) = self.options.table_prefix {
            if name.starts_with(prefix) {
                name = name[prefix.len()..].to_string();
            }
        }

        // Remove suffix if specified
        if let Some(ref suffix) = self.options.table_suffix {
            if name.ends_with(suffix) {
                let len = name.len() - suffix.len();
                name = name[..len].to_string();
            }
        }

        // Convert to appropriate case
        match self.options.naming_style {
            NamingStyle::PascalCase => to_pascal_case(&name),
            NamingStyle::CamelCase => to_camel_case(&name),
            NamingStyle::SnakeCase => name.to_lowercase(),
        }
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
/// # 示例
///
/// ```rust
/// to_snake_case("UserName") -> "user_name"
/// to_snake_case("User") -> "user"
/// ```
fn to_snake_case(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_uppercase() { format!("_{}", c.to_lowercase()) } else { c.to_string() })
        .collect::<String>()
        .trim_start_matches('_')
        .to_string()
}

