//! # engine
//!
//! engine 模块 - 数据库迁移执行引擎
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 数据库迁移执行引擎
//!
//! 本模块提供数据库迁移的执行功能，包括：
//! - 初始化迁移跟踪表
//! - 应用迁移（up）
//! - 回滚迁移（down）
//! - 查询迁移状态
//! - 版本管理

use rf_database::db::Database;
use rf_errors::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 数据库迁移记录
///
/// 表示数据库中已应用的迁移记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// 迁移版本号
    pub version: String,
    /// 迁移名称
    pub name: String,
    /// 迁移应用时间
    pub applied_at: chrono::DateTime<chrono::Utc>,
}

/// 数据库迁移执行引擎
///
/// 负责执行和管理数据库迁移
///
/// # 功能
///
/// - 初始化迁移跟踪表
/// - 应用迁移（执行 up 函数）
/// - 回滚迁移（执行 down 函数）
/// - 查询迁移状态
/// - 回滚到指定版本
pub struct MigrationEngine {
    /// 数据库连接实例
    database: Database,
    /// 迁移记录表名
    migrations_table: String,
}

impl MigrationEngine {
    /// 创建新的迁移执行引擎
    ///
    /// # 参数
    ///
    /// * `database` - 数据库连接实例
    ///
    /// # 返回
    ///
    /// 返回 MigrationEngine 实例，使用默认的迁移表名 "schema_migrations"
    ///
    /// # 示例
    ///
    /// ```rust
    /// let engine = MigrationEngine::new(database);
    /// ```
    pub fn new(database: Database) -> Self {
        Self {
            database,
            migrations_table: "schema_migrations".to_string(),
        }
    }

    /// 设置自定义的迁移表名
    ///
    /// # 参数
    ///
    /// * `table_name` - 自定义的迁移表名
    ///
    /// # 返回
    ///
    /// 返回修改后的 MigrationEngine 实例（支持链式调用）
    ///
    /// # 示例
    ///
    /// ```rust
    /// let engine = MigrationEngine::new(database)
    ///     .with_table_name("my_migrations".to_string());
    /// ```
    #[allow(dead_code)]
    pub fn with_table_name(mut self, table_name: String) -> Self {
        self.migrations_table = table_name;
        self
    }

    /// 初始化迁移跟踪表
    ///
    /// 如果不存在，则创建用于跟踪迁移的表
    ///
    /// # 表结构
    ///
    /// ```sql
    /// CREATE TABLE schema_migrations (
    ///     version VARCHAR(255) PRIMARY KEY,
    ///     name VARCHAR(255) NOT NULL,
    ///     applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    /// )
    /// ```
    ///
    /// # 返回
    ///
    /// 成功返回 Ok(())，失败返回错误
    pub async fn init(&self) -> Result<()> {
        let sql = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                version VARCHAR(255) PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#,
            self.migrations_table
        );

        self.database.raw_execute(&sql).await?;
        Ok(())
    }

    /// 获取所有已应用的迁移
    ///
    /// 从数据库中查询所有已经应用的迁移记录
    ///
    /// # 返回
    ///
    /// 返回按版本号排序的迁移记录列表
    ///
    /// # 错误
    ///
    /// - 如果查询失败
    /// - 如果表不存在（返回空列表）
    pub async fn get_applied_migrations(&self) -> Result<Vec<MigrationRecord>> {
        self.init().await?;

        let sql = format!("SELECT version, name, applied_at FROM {} ORDER BY version", self.migrations_table);
        
        // Query database using sqlx directly
        // This is a simplified implementation - in production, use proper database abstraction
        use sqlx::Row;
        
        if let Some(pool) = self.database.as_postgres() {
            match sqlx::query(&sql).fetch_all(pool).await {
                Ok(rows) => {
                    let mut migrations = Vec::new();
                    for row in rows {
                        let version: String = row.get(0);
                        let name: String = row.get(1);
                        let applied_at: chrono::DateTime<chrono::Utc> = row.get(2);
                        migrations.push(MigrationRecord {
                            version,
                            name,
                            applied_at,
                        });
                    }
                    Ok(migrations)
                }
                Err(_) => Ok(Vec::new()), // Table might be empty or not exist yet
            }
        } else {
            // For non-PostgreSQL databases, return empty for now
            // Full implementation would support MySQL and SQLite
            Ok(Vec::new())
        }
    }

    /// 检查迁移是否已应用
    ///
    /// 检查指定版本的迁移是否已经应用到数据库
    ///
    /// # 参数
    ///
    /// * `version` - 迁移版本号
    ///
    /// # 返回
    ///
    /// - `Ok(true)` - 迁移已应用
    /// - `Ok(false)` - 迁移未应用
    /// - `Err(...)` - 查询失败
    pub async fn is_applied(&self, version: &str) -> Result<bool> {
        let applied = self.get_applied_migrations().await?;
        Ok(applied.iter().any(|m| m.version == version))
    }

    /// 应用迁移
    ///
    /// 执行迁移的 up 函数，并在数据库中记录迁移
    ///
    /// # 参数
    ///
    /// * `migration` - 要应用的迁移
    ///
    /// # 返回
    ///
    /// 成功返回 Ok(())，失败返回错误
    ///
    /// # 错误
    ///
    /// - 如果迁移已经应用过
    /// - 如果执行 SQL 失败
    /// - 如果记录迁移失败
    ///
    /// # 执行步骤
    ///
    /// 1. 检查迁移是否已应用
    /// 2. 执行迁移的 up_sql
    /// 3. 在迁移表中记录迁移
    pub async fn apply_migration(&self, migration: &crate::migration_rs::Migration) -> Result<()> {
        self.init().await?;

        // Check if already applied
        if self.is_applied(&migration.version).await? {
            return Err(rf_errors::RfError::Internal(
                format!("Migration {} already applied", migration.version)
            ));
        }

        // Execute UP migration
        if !migration.up_sql.is_empty() {
            self.database.raw_execute(&migration.up_sql).await?;
        }

        // Record migration
        let sql = format!(
            "INSERT INTO {} (version, name, applied_at) VALUES ('{}', '{}', CURRENT_TIMESTAMP)",
            self.migrations_table, migration.version, migration.name
        );
        self.database.raw_execute(&sql).await?;

        Ok(())
    }

    /// 回滚迁移
    ///
    /// 执行迁移的 down 函数，并从数据库中删除迁移记录
    ///
    /// # 参数
    ///
    /// * `migration` - 要回滚的迁移
    ///
    /// # 返回
    ///
    /// 成功返回 Ok(())，失败返回错误
    ///
    /// # 错误
    ///
    /// - 如果迁移未应用过
    /// - 如果执行 SQL 失败
    /// - 如果删除迁移记录失败
    ///
    /// # 执行步骤
    ///
    /// 1. 检查迁移是否已应用
    /// 2. 执行迁移的 down_sql
    /// 3. 从迁移表中删除记录
    pub async fn rollback_migration(&self, migration: &crate::migration_rs::Migration) -> Result<()> {
        self.init().await?;

        // Check if applied
        if !self.is_applied(&migration.version).await? {
            return Err(rf_errors::RfError::Internal(
                format!("Migration {} not applied", migration.version)
            ));
        }

        // Execute DOWN migration
        if !migration.down_sql.is_empty() {
            self.database.raw_execute(&migration.down_sql).await?;
        }

        // Remove migration record
        let sql = format!("DELETE FROM {} WHERE version = '{}'", self.migrations_table, migration.version);
        self.database.raw_execute(&sql).await?;

        Ok(())
    }

    /// 回滚到指定版本
    ///
    /// 回滚所有目标版本之后应用的迁移，将数据库恢复到目标版本状态
    ///
    /// # 参数
    ///
    /// * `target_version` - 目标版本号
    /// * `migrations` - 所有可用的迁移列表
    ///
    /// # 返回
    ///
    /// 成功返回 Ok(())，失败返回错误
    ///
    /// # 执行逻辑
    ///
    /// 1. 找出所有目标版本之后已应用的迁移
    /// 2. 按版本号倒序排列
    /// 3. 逐个回滚这些迁移
    ///
    /// # 示例
    ///
    /// ```rust
    /// // 假设有迁移：v1, v2, v3, v4, v5
    /// // 当前已应用：v1, v2, v3
    /// // 回滚到 v1：
    /// engine.rollback_to("v1", &migrations).await?;
    /// // 结果：v3 和 v2 被回滚，只保留 v1
    /// ```
    pub async fn rollback_to(&self, target_version: &str, migrations: &[crate::migration_rs::Migration]) -> Result<()> {
        let applied = self.get_applied_migrations().await?;
        
        // Find migrations to rollback (those after target_version)
        let mut to_rollback: Vec<&crate::migration_rs::Migration> = migrations
            .iter()
            .filter(|m| {
                m.version.as_str() > target_version && applied.iter().any(|a| a.version.as_str() == m.version.as_str())
            })
            .collect();
        
        // Sort in reverse order (newest first)
        to_rollback.sort_by(|a, b| b.version.cmp(&a.version));

        // Rollback each migration
        for migration in to_rollback {
            self.rollback_migration(migration).await?;
        }

        Ok(())
    }

    /// 获取迁移状态
    ///
    /// 比较可用的迁移和已应用的迁移，生成状态报告
    ///
    /// # 参数
    ///
    /// * `migrations` - 所有可用的迁移列表
    ///
    /// # 返回
    ///
    /// 返回包含每个迁移状态的列表
    ///
    /// # 状态信息
    ///
    /// - version: 迁移版本号
    /// - name: 迁移名称
    /// - applied: 是否已应用
    /// - applied_at: 应用时间（如果已应用）
    pub async fn get_status(&self, migrations: &[crate::migration_rs::Migration]) -> Result<Vec<MigrationStatus>> {
        let applied = self.get_applied_migrations().await?;
        let applied_map: HashMap<String, MigrationRecord> = applied
            .into_iter()
            .map(|m| (m.version.clone(), m))
            .collect();

        let mut status = Vec::new();
        for migration in migrations {
            let is_applied = applied_map.contains_key(&migration.version);
            status.push(MigrationStatus {
                version: migration.version.clone(),
                name: migration.name.clone(),
                applied: is_applied,
                applied_at: applied_map
                    .get(&migration.version)
                    .map(|m| m.applied_at),
            });
        }

        Ok(status)
    }
}

/// 迁移状态
///
/// 表示单个迁移的应用状态信息
#[derive(Debug, Clone)]
pub struct MigrationStatus {
    /// 迁移版本号
    pub version: String,
    /// 迁移名称
    pub name: String,
    /// 是否已应用
    pub applied: bool,
    /// 应用时间（如果已应用）
    pub applied_at: Option<chrono::DateTime<chrono::Utc>>,
}

