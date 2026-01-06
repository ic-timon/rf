//! # migration_rs
//!
//! migration_rs 模块 - 数据库迁移文件管理
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 数据库迁移管理
//!
//! 本模块提供数据库迁移文件的管理功能，包括：
//! - 创建新的迁移文件
//! - 列出所有迁移文件
//! - 解析迁移文件内容
//! - 提取迁移 SQL 语句

use chrono::{DateTime, Utc};
use rf_errors::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

/// 迁移文件结构体
///
/// 表示一个数据库迁移文件，包含版本、名称和迁移脚本
#[derive(Debug, Clone)]
pub struct Migration {
    /// 迁移版本号（时间戳格式：YYYYMMDDHHMMSS）
    pub version: String,
    /// 迁移名称
    pub name: String,
    /// UP 迁移 SQL（应用迁移时执行的 SQL）
    pub up_sql: String,
    /// DOWN 迁移 SQL（回滚迁移时执行的 SQL）
    pub down_sql: String,
    /// 迁移创建时间
    #[allow(dead_code)]
    pub created_at: DateTime<Utc>,
}

/// 迁移文件管理器
///
/// 负责管理数据库迁移文件的生命周期
///
/// # 功能
///
/// - 创建新的迁移文件
/// - 列出所有迁移
/// - 解析迁移文件
/// - 获取迁移文件路径
pub struct MigrationManager {
    /// 迁移文件存储目录
    migrations_dir: PathBuf,
}

impl MigrationManager {
    /// 创建新的迁移管理器
    ///
    /// # 参数
    ///
    /// * `migrations_dir` - 迁移文件存储目录路径
    ///
    /// # 返回
    ///
    /// 返回 MigrationManager 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// let manager = MigrationManager::new("migrations");
    /// let manager = MigrationManager::new("./custom/migrations");
    /// ```
    pub fn new(migrations_dir: impl AsRef<Path>) -> Self {
        Self {
            migrations_dir: migrations_dir.as_ref().to_path_buf(),
        }
    }

    /// 创建新的迁移文件
    ///
    /// 创建一个带时间戳的 Rust 迁移文件，包含 up() 和 down() 函数模板
    ///
    /// # 参数
    ///
    /// * `name` - 迁移名称（会被清理，特殊字符替换为下划线）
    ///
    /// # 返回
    ///
    /// 返回创建的迁移文件的完整路径
    ///
    /// # 错误
    ///
    /// - 如果无法创建目录
    /// - 如果无法写入文件
    ///
    /// # 示例
    ///
    /// ```rust
    /// let manager = MigrationManager::new("migrations");
    /// let path = manager.create_migration("add_users_table")?;
    /// // 创建文件: migrations/20240101120000_add_users_table.rs
    /// ```
    pub fn create_migration(&self, name: &str) -> Result<PathBuf> {
        // Ensure migrations directory exists
        fs::create_dir_all(&self.migrations_dir)?;

        // Generate version timestamp
        let now = Utc::now();
        let version = now.format("%Y%m%d%H%M%S").to_string();
        
        // Sanitize migration name (replace spaces and special chars with underscores)
        let sanitized_name = name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect::<String>();
        
        let filename = format!("{}_{}.rs", version, sanitized_name);
        let filepath = self.migrations_dir.join(&filename);

        // Create migration file with template
        let template = format!(
            r#"//! Migration: {}
//! Created: {}

use rf_database::db::Database;

/// Migration UP: Apply this migration
pub async fn up(db: &Database) -> Result<(), Box<dyn std::error::Error>> {{
    // TODO: Add your migration SQL here
    // Example:
    // db.execute("CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR(255))").await?;
    Ok(())
}}

/// Migration DOWN: Rollback this migration
pub async fn down(db: &Database) -> Result<(), Box<dyn std::error::Error>> {{
    // TODO: Add your rollback SQL here
    // Example:
    // db.execute("DROP TABLE IF EXISTS users").await?;
    Ok(())
}}
"#,
            name, now.to_rfc3339()
        );

        fs::write(&filepath, template)?;
        Ok(filepath)
    }

    /// 列出所有迁移文件
    ///
    /// 扫描迁移目录，解析所有 .rs 迁移文件，按版本号排序并去重
    ///
    /// # 返回
    ///
    /// 返回按版本号排序的迁移列表
    ///
    /// # 错误
    ///
    /// - 如果无法读取目录
    /// - 如果文件解析失败（失败的文件会被忽略）
    ///
    /// # 注意
    ///
    /// - 只解析 .rs 扩展名的文件
    /// - 自动按版本号排序
    /// - 自动去除重复版本的迁移
    pub fn list_migrations(&self) -> Result<Vec<Migration>> {
        if !self.migrations_dir.exists() {
            return Ok(Vec::new());
        }

        let mut migrations = Vec::new();
        let entries = fs::read_dir(&self.migrations_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Some(migration) = self.parse_migration_file(&path)? {
                    migrations.push(migration);
                }
            }
        }

        // Sort by version and remove duplicates
        migrations.sort_by(|a, b| a.version.cmp(&b.version));
        
        // Remove duplicates by version
        let mut seen = HashSet::new();
        migrations.retain(|m| seen.insert(m.version.clone()));
        
        Ok(migrations)
    }

    /// 解析迁移文件
    ///
    /// 从迁移文件路径解析出版本、名称和 SQL 内容
    ///
    /// # 参数
    ///
    /// * `path` - 迁移文件路径
    ///
    /// # 返回
    ///
    /// - Ok(Some(Migration)) - 如果文件解析成功
    /// - Ok(None) - 如果文件名格式不正确
    /// - Err(...) - 如果解析过程出错
    ///
    /// # 文件名格式
    ///
    /// 迁移文件名格式: `VERSION_NAME.rs`
    /// 例如: `20240101120000_add_users_table.rs`
    fn parse_migration_file(&self, path: &Path) -> Result<Option<Migration>> {
        let filename = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| rf_errors::RfError::Internal("Invalid filename".to_string()))?;

        // Parse version and name from filename: VERSION_NAME.rs
        let parts: Vec<&str> = filename.splitn(2, '_').collect();
        if parts.len() != 2 {
            return Ok(None);
        }

        let version = parts[0].to_string();
        let name = parts[1].to_string();

        // Read file content
        let content = fs::read_to_string(path)?;
        
        // Extract UP and DOWN SQL from comments or function bodies
        // This is a simplified parser - in production, you'd use a proper parser
        let up_sql = self.extract_sql(&content, "up")?;
        let down_sql = self.extract_sql(&content, "down")?;

        // Parse created_at from file metadata or content
        let metadata = fs::metadata(path)?;
        let created_at = metadata
            .created()
            .or_else(|_| metadata.modified())
            .map(DateTime::<Utc>::from)
            .unwrap_or_else(|_| Utc::now());

        Ok(Some(Migration {
            version,
            name,
            up_sql,
            down_sql,
            created_at,
        }))
    }

    /// 从迁移文件中提取 SQL 语句
    ///
    /// 从迁移文件的 Rust 代码中提取 up 或 down 函数中的 SQL 语句
    ///
    /// # 参数
    ///
    /// * `content` - 迁移文件内容
    /// * `direction` - 方向： "up" 或 "down"
    ///
    /// # 返回
    ///
    /// 返回提取的 SQL 语句（简化版）
    ///
    /// # 注意
    ///
    /// 这是一个简化实现，实际上应该使用适当的 Rust 解析器
    /// 当前版本只提取注释和 db.execute 调用
    fn extract_sql(&self, content: &str, direction: &str) -> Result<String> {
        // Look for function body or SQL comments
        let pattern = if direction == "up" {
            "pub async fn up"
        } else {
            "pub async fn down"
        };

        if let Some(start) = content.find(pattern) {
            // Find the function body
            if let Some(body_start) = content[start..].find('{') {
                let body = &content[start + body_start + 1..];
                // Extract SQL from comments or string literals
                // This is simplified - full implementation would parse properly
                let sql = body
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.starts_with("//") || line.starts_with("db.execute") {
                            Some(line.to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                return Ok(sql);
            }
        }
        Ok(String::new())
    }

    /// 获取迁移文件路径
    ///
    /// 根据版本号查找并返回迁移文件的完整路径
    ///
    /// # 参数
    ///
    /// * `version` - 迁移版本号
    ///
    /// # 返回
    ///
    /// 返回迁移文件路径，如果找不到则返回 `VERSION_unknown.rs`
    #[allow(dead_code)]
    pub fn get_migration_path(&self, version: &str) -> PathBuf {
        // Find migration file with this version
        if let Ok(migrations) = self.list_migrations() {
            for migration in migrations {
                if migration.version == version {
                    return self.migrations_dir.join(format!("{}_{}.rs", migration.version, migration.name));
                }
            }
        }
        self.migrations_dir.join(format!("{}_unknown.rs", version))
    }
}

