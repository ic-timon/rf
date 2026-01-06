//! # database
//!
//! database 模块 - 数据库架构检查
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 数据库连接和架构检查，用于代码生成
//!
//! 本模块提供数据库架构信息的检查功能，支持：
//! - 获取数据库表列表
//! - 获取表结构信息（列、主键、索引等）
//! - 为代码生成提供必要的数据库元数据

use rf_database::db::Database;
use rf_errors::Result;
use serde::{Deserialize, Serialize};

/// 数据库表信息
///
/// 包含表的完整元数据信息，用于代码生成
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    /// 表名
    pub name: String,
    /// Schema 名称（PostgreSQL）
    pub schema: Option<String>,
    /// 列信息列表
    pub columns: Vec<ColumnInfo>,
    /// 主键列名
    pub primary_key: Option<String>,
    /// 索引信息列表
    pub indexes: Vec<IndexInfo>,
}

/// 数据库列信息
///
/// 包含表中单个列的详细元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    /// 列名
    pub name: String,
    /// 数据类型（如：varchar, integer, timestamp 等）
    pub data_type: String,
    /// 是否可为空
    pub is_nullable: bool,
    /// 是否是主键
    pub is_primary_key: bool,
    /// 默认值
    pub default_value: Option<String>,
    /// 列注释
    pub comment: Option<String>,
}

/// 数据库索引信息
///
/// 包含表中索引的详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    /// 索引名称
    pub name: String,
    /// 索引包含的列名列表
    pub columns: Vec<String>,
    /// 是否是唯一索引
    pub is_unique: bool,
}

/// 数据库架构检查器
///
/// 提供数据库架构信息的检查和提取功能
///
/// # 功能
///
/// - 获取所有表名
/// - 获取表的完整结构信息
/// - 获取索引信息
///
/// # 支持
///
/// 当前版本主要支持 PostgreSQL
pub struct SchemaInspector {
    /// 数据库连接实例
    database: Database,
}

impl SchemaInspector {
    /// 创建新的架构检查器
    ///
    /// # 参数
    ///
    /// * `database` - 数据库连接实例
    ///
    /// # 返回
    ///
    /// 返回 SchemaInspector 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// let database = Database::new_postgres("postgresql://localhost/db").await?;
    /// let inspector = SchemaInspector::new(database);
    /// ```
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// 获取所有表名
    ///
    /// 查询数据库中的所有表名，支持指定 schema
    ///
    /// # 参数
    ///
    /// * `schema` - Schema 名称（PostgreSQL），如果为 None 则使用 "public"
    ///
    /// # 返回
    ///
    /// 返回按表名排序的表名列表
    ///
    /// # 错误
    ///
    /// - 如果数据库不支持（当前仅支持 PostgreSQL）
    /// - 如果查询失败
    ///
    /// # 示例
    ///
    /// ```rust
    /// // 获取 public schema 中的所有表
    /// let tables = inspector.get_tables(None).await?;
    ///
    /// // 获取指定 schema 中的所有表
    /// let tables = inspector.get_tables(Some("my_schema")).await?;
    /// ```
    pub async fn get_tables(&self, schema: Option<&str>) -> Result<Vec<String>> {
        // PostgreSQL implementation
        if let Some(pool) = self.database.as_postgres() {
            let sql = if let Some(schema) = schema {
                format!(
                    "SELECT table_name FROM information_schema.tables WHERE table_schema = '{}' AND table_type = 'BASE TABLE' ORDER BY table_name",
                    schema
                )
            } else {
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE' ORDER BY table_name".to_string()
            };

            use sqlx::Row;
            let rows: Vec<sqlx::postgres::PgRow> = sqlx::query(&sql)
                .fetch_all(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Failed to query tables: {}", e)))?;

            let mut tables = Vec::new();
            for row in rows {
                let name: String = row.get(0);
                tables.push(name);
            }
            Ok(tables)
        } else {
            Err(rf_errors::RfError::Database(
                "Schema inspection currently only supports PostgreSQL".to_string()
            ))
        }
    }

    /// 获取表信息
    ///
    /// 获取指定表的完整结构信息，包括列、主键、索引等
    ///
    /// # 参数
    ///
    /// * `table_name` - 表名
    /// * `schema` - Schema 名称，如果为 None 则使用 "public"
    ///
    /// # 返回
    ///
    /// 返回包含表完整信息的 TableInfo 结构体
    ///
    /// # 错误
    ///
    /// - 如果数据库不支持（当前仅支持 PostgreSQL）
    /// - 如果查询失败
    ///
    /// # 示例
    ///
    /// ```rust
    /// let table_info = inspector.get_table_info("users", None).await?;
    /// println!("Table: {}", table_info.name);
    /// for column in &table_info.columns {
    ///     println!("  - {}: {}", column.name, column.data_type);
    /// }
    /// ```
    pub async fn get_table_info(&self, table_name: &str, schema: Option<&str>) -> Result<TableInfo> {
        // PostgreSQL implementation
        if let Some(pool) = self.database.as_postgres() {
            let _full_table = if let Some(_schema) = schema {
                format!("{}.{}", _schema, table_name)
            } else {
                table_name.to_string()
            };

            // Get columns
            let columns_sql = format!(
                r#"
                SELECT 
                    column_name,
                    data_type,
                    is_nullable,
                    column_default,
                    col_description(pgc.oid, ordinal_position) as comment
                FROM information_schema.columns c
                LEFT JOIN pg_class pgc ON pgc.relname = c.table_name
                WHERE table_name = '{}' AND table_schema = COALESCE('{}', 'public')
                ORDER BY ordinal_position
                "#,
                table_name,
                schema.unwrap_or("public")
            );

            use sqlx::Row;
            let column_rows: Vec<sqlx::postgres::PgRow> = sqlx::query(&columns_sql)
                .fetch_all(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Failed to query columns: {}", e)))?;

            // Get primary key
            let pk_sql = format!(
                r#"
                SELECT column_name
                FROM information_schema.table_constraints tc
                JOIN information_schema.key_column_usage kcu
                    ON tc.constraint_name = kcu.constraint_name
                WHERE tc.table_name = '{}' AND tc.constraint_type = 'PRIMARY KEY'
                LIMIT 1
                "#,
                table_name
            );

            let pk_row = sqlx::query(&pk_sql)
                .fetch_optional(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Failed to query primary key: {}", e)))?;

            let primary_key = pk_row.and_then(|row: sqlx::postgres::PgRow| {
                let name: Option<String> = row.get(0);
                name
            });

            // Build columns
            let mut columns = Vec::new();
            for row in column_rows {
                let name: String = row.get(0);
                let data_type: String = row.get(1);
                let is_nullable: String = row.get(2);
                let default_value: Option<String> = row.get(3);
                let comment: Option<String> = row.get(4);

                let is_pk = primary_key.as_ref().map(|pk| pk == &name).unwrap_or(false);

                columns.push(ColumnInfo {
                    name,
                    data_type,
                    is_nullable: is_nullable == "YES",
                    is_primary_key: is_pk,
                    default_value,
                    comment,
                });
            }

            // Get indexes (simplified)
            let indexes = self.get_indexes(table_name, schema).await.unwrap_or_default();

            Ok(TableInfo {
                name: table_name.to_string(),
                schema: schema.map(|s| s.to_string()),
                columns,
                primary_key,
                indexes,
            })
        } else {
            Err(rf_errors::RfError::Database(
                "Schema inspection currently only supports PostgreSQL".to_string()
            ))
        }
    }

    /// 获取表的索引信息
    ///
    /// 查询指定表的所有索引，包括索引列和唯一性约束
    ///
    /// # 参数
    ///
    /// * `table_name` - 表名
    /// * `_schema` - Schema 名称（当前未使用，保留用于未来扩展）
    ///
    /// # 返回
    ///
    /// 返回索引信息列表
    ///
    /// # 错误
    ///
    /// - 如果查询失败
    /// - 非 PostgreSQL 数据库返回空列表
    async fn get_indexes(&self, table_name: &str, _schema: Option<&str>) -> Result<Vec<IndexInfo>> {
        if let Some(pool) = self.database.as_postgres() {
            let sql = format!(
                r#"
                SELECT
                    i.relname as index_name,
                    a.attname as column_name,
                    ix.indisunique as is_unique
                FROM pg_class t
                JOIN pg_index ix ON t.oid = ix.indrelid
                JOIN pg_class i ON i.oid = ix.indexrelid
                JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = ANY(ix.indkey)
                WHERE t.relname = '{}' AND t.relkind = 'r'
                ORDER BY i.relname, a.attnum
                "#,
                table_name
            );

            use sqlx::Row;
            let rows = sqlx::query(&sql)
                .fetch_all(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Failed to query indexes: {}", e)))?;

            let mut index_map: std::collections::HashMap<String, IndexInfo> = std::collections::HashMap::new();
            for row in rows {
                let index_name: String = row.get(0);
                let column_name: String = row.get(1);
                let is_unique: bool = row.get(2);

                let index = index_map.entry(index_name.clone()).or_insert_with(|| IndexInfo {
                    name: index_name,
                    columns: Vec::new(),
                    is_unique,
                });
                index.columns.push(column_name);
            }

            Ok(index_map.into_values().collect())
        } else {
            Ok(Vec::new())
        }
    }
}

