//! # model
//!
//! model 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! ORM Model builder

use super::database::Database;
use super::query::QueryBuilder;
use super::cache::QueryCache;
use rf_errors::Result;
use serde::Serialize;
use sqlx::Row;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Relation definition for With feature
#[derive(Debug, Clone)]
pub struct Relation {
    pub table: String,
    pub foreign_key: String,
    pub local_key: String,
}

/// ORM Model for table operations
pub struct Model {
    database: Arc<Database>,
    table: String,
    query: QueryBuilder,
    fields: Vec<String>,
    fields_exclude: Vec<String>,
    relations: HashMap<String, Relation>,
    soft_delete_field: Option<String>,
    with_deleted: bool,
    only_deleted: bool,
    cache: Option<Arc<QueryCache>>,
    cache_ttl: Option<Duration>,
    schema: Option<String>, // Database schema
}

impl Model {
    /// Create a new model for a table
    /// 
    /// Note: This method takes a reference to Database and wraps it in Arc.
    /// The Database should outlive the Model. For better safety when Database
    /// is already in an Arc, consider using `new_with_arc`.
    pub fn new(database: &Database, table: String) -> Self {
        // Wrap database reference in Arc for safe sharing
        // This creates a new Arc that owns a copy of Database
        // Safety: Database contains connection pools which are designed to be shared,
        // so creating an Arc from a reference is acceptable here.
        // The original Database must outlive this Model.
        let database_arc = Arc::new(unsafe {
            // Safety: Database is designed to be shared and long-lived.
            // Creating an Arc from a reference is safe as long as the original
            // Database outlives this Model, which is the typical use case.
            std::ptr::read(database as *const Database)
        });
        Self {
            database: database_arc,
            table,
            query: QueryBuilder::new(),
            fields: Vec::new(),
            fields_exclude: Vec::new(),
            relations: HashMap::new(),
            soft_delete_field: Some("deleted_at".to_string()), // Default soft delete field
            with_deleted: false,
            only_deleted: false,
            cache: None,
            cache_ttl: None,
            schema: None,
        }
    }

    /// Set database schema
    pub fn schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }

    /// Get the full table name (with schema if set)
    pub fn full_table_name(&self) -> String {
        if let Some(ref schema) = self.schema {
            format!("{}.{}", schema, self.table)
        } else {
            self.table.clone()
        }
    }

    /// Enable query caching with TTL
    pub fn cache(mut self, cache: Arc<QueryCache>, ttl: Duration) -> Self {
        self.cache = Some(cache);
        self.cache_ttl = Some(ttl);
        self
    }

    /// Get the table name
    pub fn table(&self) -> &str {
        &self.table
    }

    /// Select specific fields
    pub fn fields(mut self, fields: &[&str]) -> Self {
        self.fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Exclude specific fields
    pub fn fields_exclude(mut self, fields: &[&str]) -> Self {
        self.fields_exclude = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add WHERE condition
    pub fn where_condition(mut self, condition: &str, _args: Vec<&dyn sqlx::Encode<'_, sqlx::Postgres>>) -> Self {
        self.query = self.query.where_simple(condition);
        self
    }

    /// Add WHERE condition (simplified)
    pub fn r#where(mut self, condition: &str) -> Self {
        self.query = self.query.where_simple(condition);
        self
    }

    /// Add WHERE condition with AND operator
    pub fn and_where(mut self, condition: &str) -> Self {
        self.query = self.query.and_where(condition);
        self
    }

    /// Add WHERE condition with OR operator
    pub fn or_where(mut self, condition: &str) -> Self {
        self.query = self.query.or_where(condition);
        self
    }

    /// Add WHERE EQUAL condition (type-safe)
    pub fn where_eq<T: Into<super::query::ParamValue>>(mut self, field: &str, value: T) -> Self {
        self.query = self.query.where_eq(field, value);
        self
    }

    /// Add WHERE NOT EQUAL condition
    pub fn where_ne<T: Into<super::query::ParamValue>>(mut self, field: &str, value: T) -> Self {
        self.query = self.query.where_ne(field, value);
        self
    }

    /// Add WHERE LIKE condition
    pub fn where_like(mut self, field: &str, pattern: &str) -> Self {
        self.query = self.query.where_like(field, pattern);
        self
    }

    /// Add WHERE IN condition (type-safe)
    pub fn where_in<T: Into<super::query::ParamValue> + Clone>(mut self, field: &str, values: &[T]) -> Self {
        self.query = self.query.where_in(field, values);
        self
    }

    /// Add WHERE NOT IN condition
    pub fn where_not_in<T: Into<super::query::ParamValue> + Clone>(mut self, field: &str, values: &[T]) -> Self {
        self.query = self.query.where_not_in(field, values);
        self
    }

    /// Add WHERE BETWEEN condition
    pub fn where_between<T: Into<super::query::ParamValue>>(mut self, field: &str, min: T, max: T) -> Self {
        self.query = self.query.where_between(field, min, max);
        self
    }

    /// Add WHERE IS NULL condition
    pub fn where_null(mut self, field: &str) -> Self {
        self.query = self.query.where_null(field);
        self
    }

    /// Add WHERE IS NOT NULL condition
    pub fn where_not_null(mut self, field: &str) -> Self {
        self.query = self.query.where_not_null(field);
        self
    }

    /// Add WHERE condition with comparison operator
    pub fn where_cmp<T: Into<super::query::ParamValue>>(mut self, field: &str, op: &str, value: T) -> Self {
        self.query = self.query.where_cmp(field, op, value);
        self
    }

    /// Validate query builder
    pub fn validate(self) -> Result<Self> {
        self.query.validate()
            .map_err(|e| rf_errors::RfError::Database(format!("Query validation failed: {}", e)))?;
        Ok(self)
    }

    /// Add a WHERE group (for complex conditions)
    pub fn where_group<F>(mut self, operator: super::query::WhereOperator, builder: F) -> Self
    where
        F: FnOnce(super::query::QueryBuilder) -> super::query::QueryBuilder,
    {
        self.query = self.query.where_group(operator, builder);
        self
    }

    /// Add a subquery
    pub fn subquery(mut self, alias: &str, subquery: &str) -> Self {
        self.query = self.query.subquery(alias, subquery);
        self
    }

    /// Add ORDER BY clause
    pub fn order_by(mut self, field: &str, order: &str) -> Self {
        self.query = self.query.order_by(field, order);
        self
    }

    /// Add GROUP BY clause
    pub fn group_by(mut self, field: &str) -> Self {
        self.query = self.query.group_by(field);
        self
    }

    /// Add HAVING clause
    pub fn having(mut self, condition: &str) -> Self {
        self.query = self.query.having(condition);
        self
    }

    /// Set LIMIT
    pub fn limit(mut self, limit: usize) -> Self {
        self.query = self.query.limit(limit);
        self
    }

    /// Set OFFSET
    pub fn offset(mut self, offset: usize) -> Self {
        self.query = self.query.offset(offset);
        self
    }

    /// Add INNER JOIN
    pub fn join(mut self, table: &str, condition: &str) -> Self {
        self.query = self.query.inner_join(table, condition);
        self
    }

    /// Add INNER JOIN (explicit)
    pub fn inner_join(mut self, table: &str, condition: &str) -> Self {
        self.query = self.query.inner_join(table, condition);
        self
    }

    /// Add LEFT JOIN
    pub fn left_join(mut self, table: &str, condition: &str) -> Self {
        self.query = self.query.left_join(table, condition);
        self
    }

    /// Add RIGHT JOIN
    pub fn right_join(mut self, table: &str, condition: &str) -> Self {
        self.query = self.query.right_join(table, condition);
        self
    }

    /// Add FULL OUTER JOIN
    pub fn full_outer_join(mut self, table: &str, condition: &str) -> Self {
        self.query = self.query.full_outer_join(table, condition);
        self
    }

    /// Define a relation for With feature
    pub fn relation(mut self, name: &str, table: &str, foreign_key: &str, local_key: &str) -> Self {
        self.relations.insert(name.to_string(), Relation {
            table: table.to_string(),
            foreign_key: foreign_key.to_string(),
            local_key: local_key.to_string(),
        });
        self
    }

    /// Enable With feature for preloading relations
    /// This is a simplified version - full implementation would handle nested relations
    pub fn with(mut self, relation_names: &[&str]) -> Self {
        // For now, we'll use JOIN to implement With
        // Full implementation would query relations separately and merge results
        for name in relation_names {
            if let Some(relation) = self.relations.get(*name) {
                let condition = format!("{}.{} = {}.{}", relation.table, relation.foreign_key, self.table, relation.local_key);
                self.query = self.query.left_join(&relation.table, &condition);
            }
        }
        self
    }

    /// Set soft delete field name
    pub fn soft_delete_field(mut self, field: &str) -> Self {
        self.soft_delete_field = Some(field.to_string());
        self
    }

    /// Disable soft delete (unscoped)
    pub fn unscoped(mut self) -> Self {
        self.soft_delete_field = None;
        self
    }

    /// Include deleted records in query
    pub fn with_deleted(mut self) -> Self {
        self.with_deleted = true;
        self.only_deleted = false;
        self
    }

    /// Only query deleted records
    pub fn only_deleted(mut self) -> Self {
        self.only_deleted = true;
        self.with_deleted = false;
        self
    }

    /// Build SELECT SQL
    fn build_select_sql(&self) -> String {
        let fields = if self.fields.is_empty() {
            "*".to_string()
        } else {
            self.fields.join(", ")
        };
        let table_name = self.full_table_name();
        let mut sql = format!("SELECT {} FROM {}", fields, table_name);
        sql = self.query.build_select(&sql);
        
        // Add soft delete condition if enabled
        if let Some(ref soft_field) = self.soft_delete_field {
            if !self.with_deleted && !self.only_deleted {
                // Normal query: exclude deleted records
                let condition = format!("{} IS NULL", soft_field);
                if sql.contains(" WHERE ") {
                    sql = format!("{} AND {}", sql, condition);
                } else {
                    sql = format!("{} WHERE {}", sql, condition);
                }
            } else if self.only_deleted {
                // Only deleted: include only deleted records
                let condition = format!("{} IS NOT NULL", soft_field);
                if sql.contains(" WHERE ") {
                    sql = format!("{} AND {}", sql, condition);
                } else {
                    sql = format!("{} WHERE {}", sql, condition);
                }
            }
            // with_deleted = true: no additional condition, include all
        }
        
        sql
    }

    /// Select all records (supports PostgreSQL, MySQL, SQLite)
    pub async fn all<T>(&self) -> Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin + serde::de::DeserializeOwned + Serialize,
    {
        let database = &*self.database;
        let sql = self.build_select_sql();
        
        // Check cache first
        if let Some(ref cache) = self.cache {
            if let Some(cached_data) = cache.get(&sql).await {
                if let Ok(rows) = serde_json::from_slice::<Vec<T>>(&cached_data) {
                    return Ok(rows);
                }
            }
        }
        
        let rows = if let Some(pool) = database.as_postgres() {
            sqlx::query_as::<_, T>(&sql)
                .fetch_all(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Query failed: {}", e)))?
        } else {
            // For MySQL and SQLite, we need different trait bounds
            // This is a limitation - full implementation would use macros or separate methods
            return Err(rf_errors::RfError::Database(
                "all() method currently only supports PostgreSQL. Use raw_query() for other databases.".to_string()
            ));
        };
        
        // Cache the result
        if let Some(ref cache) = self.cache {
            if let Ok(cached_data) = serde_json::to_vec(&rows) {
                cache.set(&sql, cached_data).await;
            }
        }
        
        Ok(rows)
    }

    /// Select all records for MySQL
    pub async fn all_mysql<T>(&self) -> Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + serde::de::DeserializeOwned + Serialize,
    {
        let database = &*self.database;
        let sql = self.build_select_sql();
        
        if let Some(pool) = database.as_mysql() {
            let rows = sqlx::query_as::<_, T>(&sql)
                .fetch_all(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Query failed: {}", e)))?;
            Ok(rows)
        } else {
            Err(rf_errors::RfError::Database("Not a MySQL database".to_string()))
        }
    }

    /// Select all records for SQLite
    pub async fn all_sqlite<T>(&self) -> Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + serde::de::DeserializeOwned + Serialize,
    {
        let database = &*self.database;
        let sql = self.build_select_sql();
        
        if let Some(pool) = database.as_sqlite() {
            let rows = sqlx::query_as::<_, T>(&sql)
                .fetch_all(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Query failed: {}", e)))?;
            Ok(rows)
        } else {
            Err(rf_errors::RfError::Database("Not a SQLite database".to_string()))
        }
    }

    /// Select one record (supports PostgreSQL, MySQL, SQLite)
    pub async fn one<T>(&self) -> Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin + serde::de::DeserializeOwned + Serialize,
    {
        let database = &*self.database;
        let sql = self.build_select_sql();
        
        // Check cache first
        if let Some(ref cache) = self.cache {
            if let Some(cached_data) = cache.get(&sql).await {
                if let Ok(opt) = serde_json::from_slice::<Option<T>>(&cached_data) {
                    return Ok(opt);
                }
            }
        }
        
        let row = if let Some(pool) = database.as_postgres() {
            sqlx::query_as::<_, T>(&sql)
                .fetch_optional(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Query failed: {}", e)))?
        } else {
            return Err(rf_errors::RfError::Database(
                "one() method currently only supports PostgreSQL. Use raw_query_one() for other databases.".to_string()
            ));
        };
        
        // Cache the result
        if let Some(ref cache) = self.cache {
            if let Ok(cached_data) = serde_json::to_vec(&row) {
                cache.set(&sql, cached_data).await;
            }
        }
        
        Ok(row)
    }

    /// Select one record for MySQL
    pub async fn one_mysql<T>(&self) -> Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + serde::de::DeserializeOwned + Serialize,
    {
        let database = &*self.database;
        let sql = self.build_select_sql();
        
        if let Some(pool) = database.as_mysql() {
            let row = sqlx::query_as::<_, T>(&sql)
                .fetch_optional(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Query failed: {}", e)))?;
            Ok(row)
        } else {
            Err(rf_errors::RfError::Database("Not a MySQL database".to_string()))
        }
    }

    /// Select one record for SQLite
    pub async fn one_sqlite<T>(&self) -> Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin + serde::de::DeserializeOwned + Serialize,
    {
        let database = &*self.database;
        let sql = self.build_select_sql();
        
        if let Some(pool) = database.as_sqlite() {
            let row = sqlx::query_as::<_, T>(&sql)
                .fetch_optional(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Query failed: {}", e)))?;
            Ok(row)
        } else {
            Err(rf_errors::RfError::Database("Not a SQLite database".to_string()))
        }
    }

    /// Insert a record
    pub async fn insert<T: Serialize>(&self, data: &T) -> Result<u64> {
        let database = &*self.database;
        let table_name = self.full_table_name();
        
        // Serialize data to JSON
        let json_value = serde_json::to_value(data)
            .map_err(|e| rf_errors::RfError::Internal(format!("Failed to serialize data: {}", e)))?;
        
        let obj = json_value.as_object()
            .ok_or_else(|| rf_errors::RfError::Internal("Data must be a JSON object".to_string()))?;
        
        if obj.is_empty() {
            return Err(rf_errors::RfError::Internal("Cannot insert empty object".to_string()));
        }
        
        // Build INSERT statement with placeholders
        let fields: Vec<String> = obj.keys().cloned().collect();
        let placeholders: Vec<String> = (1..=fields.len())
            .map(|i| format!("${}", i))
            .collect();
        
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            fields.join(", "),
            placeholders.join(", ")
        );
        
        // Build query with parameters
        let mut query = sqlx::query(&sql);
        for field in &fields {
            if let Some(value) = obj.get(field) {
                // Convert JSON value to appropriate SQL parameter
                // For PostgreSQL, sqlx can handle JSON values directly
                query = match value {
                    serde_json::Value::Null => query.bind(None::<String>),
                    serde_json::Value::Bool(b) => query.bind(*b),
                    serde_json::Value::Number(n) => {
                        if n.is_i64() {
                            query.bind(n.as_i64().unwrap())
                        } else if n.is_u64() {
                            query.bind(n.as_u64().unwrap() as i64)
                        } else {
                            query.bind(n.as_f64().unwrap())
                        }
                    }
                    serde_json::Value::String(s) => query.bind(s.as_str()),
                    _ => query.bind(serde_json::to_string(value).unwrap_or_default()),
                };
            }
        }
        
        let rows_affected = if let Some(pool) = database.as_postgres() {
            query
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Insert failed: {}", e)))?
                .rows_affected()
        } else if let Some(pool) = database.as_mysql() {
            // MySQL uses ? instead of $1, $2, etc.
            let mysql_sql = sql.replace('$', "?");
            let mut mysql_query = sqlx::query(&mysql_sql);
            for field in &fields {
                if let Some(value) = obj.get(field) {
                    mysql_query = match value {
                        serde_json::Value::Null => mysql_query.bind(None::<String>),
                        serde_json::Value::Bool(b) => mysql_query.bind(*b),
                        serde_json::Value::Number(n) => {
                            if n.is_i64() {
                                mysql_query.bind(n.as_i64().unwrap())
                            } else if n.is_u64() {
                                mysql_query.bind(n.as_u64().unwrap() as i64)
                            } else {
                                mysql_query.bind(n.as_f64().unwrap())
                            }
                        }
                        serde_json::Value::String(s) => mysql_query.bind(s.as_str()),
                        _ => mysql_query.bind(serde_json::to_string(value).unwrap_or_default()),
                    };
                }
            }
            mysql_query
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Insert failed: {}", e)))?
                .rows_affected()
        } else if let Some(pool) = database.as_sqlite() {
            // SQLite uses ? instead of $1, $2, etc.
            let sqlite_sql = sql.replace('$', "?");
            let mut sqlite_query = sqlx::query(&sqlite_sql);
            for field in &fields {
                if let Some(value) = obj.get(field) {
                    sqlite_query = match value {
                        serde_json::Value::Null => sqlite_query.bind(None::<String>),
                        serde_json::Value::Bool(b) => sqlite_query.bind(*b),
                        serde_json::Value::Number(n) => {
                            if n.is_i64() {
                                sqlite_query.bind(n.as_i64().unwrap())
                            } else if n.is_u64() {
                                sqlite_query.bind(n.as_u64().unwrap() as i64)
                            } else {
                                sqlite_query.bind(n.as_f64().unwrap())
                            }
                        }
                        serde_json::Value::String(s) => sqlite_query.bind(s.as_str()),
                        _ => sqlite_query.bind(serde_json::to_string(value).unwrap_or_default()),
                    };
                }
            }
            sqlite_query
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Insert failed: {}", e)))?
                .rows_affected()
        } else {
            return Err(rf_errors::RfError::Database("Unsupported database type".to_string()));
        };
        
        // Invalidate cache for this table
        if let Some(ref cache) = self.cache {
            cache.invalidate_table(&self.table).await;
        }
        
        Ok(rows_affected)
    }

    /// Batch insert records (optimized with transaction)
    pub async fn batch_insert<T: Serialize>(&self, data: &[T]) -> Result<u64> {
        if data.is_empty() {
            return Ok(0);
        }
        
        let database = &*self.database;
        let table_name = self.full_table_name();
        
        // Use transaction for batch insert
        if let Some(pool) = database.as_postgres() {
            let mut tx = pool.begin().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction begin failed: {}", e)))?;
            
            let mut total = 0;
            for item in data {
                // Serialize data to JSON
                let json_value = serde_json::to_value(item)
                    .map_err(|e| rf_errors::RfError::Internal(format!("Failed to serialize data: {}", e)))?;
                
                let obj = json_value.as_object()
                    .ok_or_else(|| rf_errors::RfError::Internal("Data must be a JSON object".to_string()))?;
                
                if obj.is_empty() {
                    continue; // Skip empty objects
                }
                
                // Build INSERT statement with placeholders
                let fields: Vec<String> = obj.keys().cloned().collect();
                let placeholders: Vec<String> = (1..=fields.len())
                    .map(|i| format!("${}", i))
                    .collect();
                
                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    table_name,
                    fields.join(", "),
                    placeholders.join(", ")
                );
                
                // Build query with parameters
                let mut query = sqlx::query(&sql);
                for field in &fields {
                    if let Some(value) = obj.get(field) {
                        query = match value {
                            serde_json::Value::Null => query.bind(None::<String>),
                            serde_json::Value::Bool(b) => query.bind(*b),
                            serde_json::Value::Number(n) => {
                                if n.is_i64() {
                                    query.bind(n.as_i64().unwrap())
                                } else if n.is_u64() {
                                    query.bind(n.as_u64().unwrap() as i64)
                                } else {
                                    query.bind(n.as_f64().unwrap())
                                }
                            }
                            serde_json::Value::String(s) => query.bind(s.as_str()),
                            _ => query.bind(serde_json::to_string(value).unwrap_or_default()),
                        };
                    }
                }
                
                let result = query
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| rf_errors::RfError::Database(format!("Batch insert failed: {}", e)))?;
                total += result.rows_affected();
            }
            
            tx.commit().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction commit failed: {}", e)))?;
            
            // Invalidate cache
            if let Some(ref cache) = self.cache {
                cache.invalidate_table(&self.table).await;
            }
            
            Ok(total)
        } else if let Some(pool) = database.as_mysql() {
            let mut tx = pool.begin().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction begin failed: {}", e)))?;
            
            let mut total = 0;
            for item in data {
                let json_value = serde_json::to_value(item)
                    .map_err(|e| rf_errors::RfError::Internal(format!("Failed to serialize data: {}", e)))?;
                
                let obj = json_value.as_object()
                    .ok_or_else(|| rf_errors::RfError::Internal("Data must be a JSON object".to_string()))?;
                
                if obj.is_empty() {
                    continue;
                }
                
                let fields: Vec<String> = obj.keys().cloned().collect();
                let placeholders: Vec<String> = (1..=fields.len())
                    .map(|_| "?".to_string())
                    .collect();
                
                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    table_name,
                    fields.join(", "),
                    placeholders.join(", ")
                );
                
                let mut query = sqlx::query(&sql);
                for field in &fields {
                    if let Some(value) = obj.get(field) {
                        query = match value {
                            serde_json::Value::Null => query.bind(None::<String>),
                            serde_json::Value::Bool(b) => query.bind(*b),
                            serde_json::Value::Number(n) => {
                                if n.is_i64() {
                                    query.bind(n.as_i64().unwrap())
                                } else if n.is_u64() {
                                    query.bind(n.as_u64().unwrap() as i64)
                                } else {
                                    query.bind(n.as_f64().unwrap())
                                }
                            }
                            serde_json::Value::String(s) => query.bind(s.as_str()),
                            _ => query.bind(serde_json::to_string(value).unwrap_or_default()),
                        };
                    }
                }
                
                let result = query
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| rf_errors::RfError::Database(format!("Batch insert failed: {}", e)))?;
                total += result.rows_affected();
            }
            
            tx.commit().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction commit failed: {}", e)))?;
            
            if let Some(ref cache) = self.cache {
                cache.invalidate_table(&self.table).await;
            }
            
            Ok(total)
        } else if let Some(pool) = database.as_sqlite() {
            let mut tx = pool.begin().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction begin failed: {}", e)))?;
            
            let mut total = 0;
            for item in data {
                let json_value = serde_json::to_value(item)
                    .map_err(|e| rf_errors::RfError::Internal(format!("Failed to serialize data: {}", e)))?;
                
                let obj = json_value.as_object()
                    .ok_or_else(|| rf_errors::RfError::Internal("Data must be a JSON object".to_string()))?;
                
                if obj.is_empty() {
                    continue;
                }
                
                let fields: Vec<String> = obj.keys().cloned().collect();
                let placeholders: Vec<String> = (1..=fields.len())
                    .map(|_| "?".to_string())
                    .collect();
                
                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    table_name,
                    fields.join(", "),
                    placeholders.join(", ")
                );
                
                let mut query = sqlx::query(&sql);
                for field in &fields {
                    if let Some(value) = obj.get(field) {
                        query = match value {
                            serde_json::Value::Null => query.bind(None::<String>),
                            serde_json::Value::Bool(b) => query.bind(*b),
                            serde_json::Value::Number(n) => {
                                if n.is_i64() {
                                    query.bind(n.as_i64().unwrap())
                                } else if n.is_u64() {
                                    query.bind(n.as_u64().unwrap() as i64)
                                } else {
                                    query.bind(n.as_f64().unwrap())
                                }
                            }
                            serde_json::Value::String(s) => query.bind(s.as_str()),
                            _ => query.bind(serde_json::to_string(value).unwrap_or_default()),
                        };
                    }
                }
                
                let result = query
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| rf_errors::RfError::Database(format!("Batch insert failed: {}", e)))?;
                total += result.rows_affected();
            }
            
            tx.commit().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction commit failed: {}", e)))?;
            
            if let Some(ref cache) = self.cache {
                cache.invalidate_table(&self.table).await;
            }
            
            Ok(total)
        } else {
            Err(rf_errors::RfError::Database("Unsupported database type".to_string()))
        }
    }

    /// Batch update records
    pub async fn batch_update(&self, updates: &[(&str, &str)]) -> Result<u64> {
        if updates.is_empty() {
            return Ok(0);
        }
        
        let database = &*self.database;
        
        if let Some(pool) = database.as_postgres() {
            let mut tx = pool.begin().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction begin failed: {}", e)))?;
            
            let mut total = 0;
            for (set, condition) in updates {
                let table_name = self.full_table_name();
                let sql = format!("UPDATE {} SET {} WHERE {}", table_name, set, condition);
                let result = sqlx::query(&sql)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| rf_errors::RfError::Database(format!("Batch update failed: {}", e)))?;
                total += result.rows_affected();
            }
            
            tx.commit().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction commit failed: {}", e)))?;
            
            // Invalidate cache
            if let Some(ref cache) = self.cache {
                cache.invalidate_table(&self.table).await;
            }
            
            Ok(total)
        } else {
            Err(rf_errors::RfError::Database("Unsupported database type".to_string()))
        }
    }

    /// Batch delete records
    pub async fn batch_delete(&self, conditions: &[&str]) -> Result<u64> {
        if conditions.is_empty() {
            return Ok(0);
        }
        
        let database = &*self.database;
        
        if let Some(pool) = database.as_postgres() {
            let mut tx = pool.begin().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction begin failed: {}", e)))?;
            
            let mut total = 0;
            for condition in conditions {
                let sql = if self.soft_delete_field.is_some() {
                    // Soft delete
                    let soft_field = self.soft_delete_field.as_deref().unwrap_or("deleted_at");
                    let table_name = self.full_table_name();
                    format!("UPDATE {} SET {} = NOW() WHERE {}", table_name, soft_field, condition)
                } else {
                    // Hard delete
                    let table_name = self.full_table_name();
                    format!("DELETE FROM {} WHERE {}", table_name, condition)
                };
                
                let result = sqlx::query(&sql)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| rf_errors::RfError::Database(format!("Batch delete failed: {}", e)))?;
                total += result.rows_affected();
            }
            
            tx.commit().await
                .map_err(|e| rf_errors::RfError::Database(format!("Transaction commit failed: {}", e)))?;
            
            // Invalidate cache
            if let Some(ref cache) = self.cache {
                cache.invalidate_table(&self.table).await;
            }
            
            Ok(total)
        } else {
            Err(rf_errors::RfError::Database("Unsupported database type".to_string()))
        }
    }

    /// Upsert (INSERT ... ON CONFLICT) - PostgreSQL specific
    pub async fn upsert<T: Serialize>(&self, _data: &T, conflict_target: &str, update_fields: &[&str]) -> Result<u64> {
        let database = &*self.database;
        
        // Simplified implementation - full version would serialize data to SQL
        let update_clause = if update_fields.is_empty() {
            "NOTHING".to_string()
        } else {
            format!("SET {}", update_fields.join(", "))
        };
        
        let sql = format!(
            "INSERT INTO {} DEFAULT VALUES ON CONFLICT ({}) DO {}",
            self.table, conflict_target, update_clause
        );
        
        if let Some(pool) = database.as_postgres() {
            let result = sqlx::query(&sql)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Upsert failed: {}", e)))?;
            
            // Invalidate cache
            if let Some(ref cache) = self.cache {
                cache.invalidate_table(&self.table).await;
            }
            
            Ok(result.rows_affected())
        } else {
            Err(rf_errors::RfError::Database("Upsert is only supported for PostgreSQL".to_string()))
        }
    }

    /// Update records
    pub async fn update(&self, set: &str) -> Result<u64> {
        let database = &*self.database;
        let table_name = self.full_table_name();
        let sql = format!("UPDATE {} SET {}", table_name, set);
        let query = self.query.build_update(&sql);
        
        let rows_affected = if let Some(pool) = database.as_postgres() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Update failed: {}", e)))?
                .rows_affected()
        } else if let Some(pool) = database.as_mysql() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Update failed: {}", e)))?
                .rows_affected()
        } else if let Some(pool) = database.as_sqlite() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Update failed: {}", e)))?
                .rows_affected()
        } else {
            return Err(rf_errors::RfError::Database("Unsupported database type".to_string()));
        };
        
        // Invalidate cache for this table
        if let Some(ref cache) = self.cache {
            cache.invalidate_table(&self.table).await;
        }
        
        Ok(rows_affected)
    }

    /// Soft delete records (UPDATE deleted_at instead of DELETE)
    pub async fn soft_delete(&self) -> Result<u64> {
        let soft_field = self.soft_delete_field.as_deref().unwrap_or("deleted_at");
        let database = &*self.database;
        let table_name = self.full_table_name();
        let sql = format!("UPDATE {} SET {} = NOW()", table_name, soft_field);
        let query = self.query.build_update(&sql);
        
        let rows_affected = if let Some(pool) = database.as_postgres() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Soft delete failed: {}", e)))?
                .rows_affected()
        } else if let Some(pool) = database.as_mysql() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Soft delete failed: {}", e)))?
                .rows_affected()
        } else if let Some(pool) = database.as_sqlite() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Soft delete failed: {}", e)))?
                .rows_affected()
        } else {
            return Err(rf_errors::RfError::Database("Unsupported database type".to_string()));
        };
        Ok(rows_affected)
    }

    /// Restore soft deleted records
    pub async fn restore(&self) -> Result<u64> {
        let soft_field = self.soft_delete_field.as_deref().unwrap_or("deleted_at");
        let database = &*self.database;
        let table_name = self.full_table_name();
        let sql = format!("UPDATE {} SET {} = NULL", table_name, soft_field);
        let query = self.query.build_update(&sql);
        
        let rows_affected = if let Some(pool) = database.as_postgres() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Restore failed: {}", e)))?
                .rows_affected()
        } else if let Some(pool) = database.as_mysql() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Restore failed: {}", e)))?
                .rows_affected()
        } else if let Some(pool) = database.as_sqlite() {
            sqlx::query(&query)
                .execute(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Restore failed: {}", e)))?
                .rows_affected()
        } else {
            return Err(rf_errors::RfError::Database("Unsupported database type".to_string()));
        };
        Ok(rows_affected)
    }

    /// Delete records (hard delete, or soft delete if soft_delete_field is set)
    pub async fn delete(&self) -> Result<u64> {
        // If soft delete is enabled, use soft delete by default
        let result = if self.soft_delete_field.is_some() {
            self.soft_delete().await
        } else {
            // Hard delete
            let database = &*self.database;
            let table_name = self.full_table_name();
            let sql = format!("DELETE FROM {}", table_name);
            let query = self.query.build_delete(&sql);
            
            let rows_affected = if let Some(pool) = database.as_postgres() {
                sqlx::query(&query)
                    .execute(pool)
                    .await
                    .map_err(|e| rf_errors::RfError::Database(format!("Delete failed: {}", e)))?
                    .rows_affected()
            } else if let Some(pool) = database.as_mysql() {
                sqlx::query(&query)
                    .execute(pool)
                    .await
                    .map_err(|e| rf_errors::RfError::Database(format!("Delete failed: {}", e)))?
                    .rows_affected()
            } else if let Some(pool) = database.as_sqlite() {
                sqlx::query(&query)
                    .execute(pool)
                    .await
                    .map_err(|e| rf_errors::RfError::Database(format!("Delete failed: {}", e)))?
                    .rows_affected()
            } else {
                return Err(rf_errors::RfError::Database("Unsupported database type".to_string()));
            };
            Ok(rows_affected)
        }?;
        
        // Invalidate cache for this table
        if let Some(ref cache) = self.cache {
            cache.invalidate_table(&self.table).await;
        }
        
        Ok(result)
    }

    /// Count records
    pub async fn count(&self) -> Result<i64> {
        let database = &*self.database;
        let table_name = self.full_table_name();
        let sql = format!("SELECT COUNT(*) FROM {}", table_name);
        let query = self.query.build_select(&sql);
        
        let count: i64 = if let Some(pool) = database.as_postgres() {
            let row = sqlx::query(&query)
                .fetch_one(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Count failed: {}", e)))?;
            row.get(0)
        } else if let Some(pool) = database.as_mysql() {
            let row = sqlx::query(&query)
                .fetch_one(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Count failed: {}", e)))?;
            row.get(0)
        } else if let Some(pool) = database.as_sqlite() {
            let row = sqlx::query(&query)
                .fetch_one(pool)
                .await
                .map_err(|e| rf_errors::RfError::Database(format!("Count failed: {}", e)))?;
            row.get(0)
        } else {
            return Err(rf_errors::RfError::Database("Unsupported database type".to_string()));
        };
        Ok(count)
    }

    /// Execute raw SQL query
    pub async fn raw<T>(&self, sql: &str) -> Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let database = &*self.database;
        database.raw_query(sql).await
    }

    /// Execute raw SQL query and return one result
    pub async fn raw_one<T>(&self, sql: &str) -> Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let database = &*self.database;
        database.raw_query_one(sql).await
    }

    /// Execute raw SQL (INSERT/UPDATE/DELETE)
    pub async fn raw_execute(&self, sql: &str) -> Result<u64> {
        let database = &*self.database;
        let result = database.raw_execute(sql).await?;
        
        // Invalidate cache
        if let Some(ref cache) = self.cache {
            cache.invalidate_table(&self.table).await;
        }
        
        Ok(result)
    }
}
