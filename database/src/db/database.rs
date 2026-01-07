//! # database
//!
//! database 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Database Connection and Pool Management
//!
//! 数据库连接和连接池管理模块。
//!
//! ## 功能特性
//!
//! - 支持多种数据库类型：PostgreSQL、MySQL、SQLite
//! - 自动管理连接池，默认最大连接数为 10
//! - 提供原始 SQL 查询执行接口
//! - 类型安全的连接池访问
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use rf_database::db::Database;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 创建 PostgreSQL 连接
//! let db = Database::new_postgres("postgresql://user:pass@localhost/db").await?;
//!
//! // 创建 MySQL 连接
//! let db = Database::new_mysql("mysql://user:pass@localhost/db").await?;
//!
//! // 创建 SQLite 连接
//! let db = Database::new_sqlite("sqlite://path/to/db.sqlite").await?;
//! # Ok(())
//! # }
//! ```

use rf_errors::{Result, RfError};
use sqlx::{Pool, Postgres, MySql, Sqlite};
use sqlx::postgres::PgPoolOptions;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;

/// 数据库类型枚举
///
/// 支持的数据库类型：
/// - `Postgres`: PostgreSQL 数据库
/// - `MySql`: MySQL 数据库
/// - `Sqlite`: SQLite 数据库
#[derive(Debug, Clone, Copy)]
pub enum DatabaseType {
    Postgres,
    MySql,
    Sqlite,
}

/// 数据库连接池包装器
///
/// 封装了底层连接池，提供统一的数据库操作接口。
///
/// ## 字段说明
///
/// - `pool`: 内部连接池枚举，存储实际的数据库连接池
/// - `db_type`: 数据库类型标识
pub struct Database {
    pool: DatabasePool,
    db_type: DatabaseType,
}

/// 内部连接池枚举
///
/// 存储不同类型的数据库连接池实例。
enum DatabasePool {
    Postgres(Pool<Postgres>),
    MySql(Pool<MySql>),
    Sqlite(Pool<Sqlite>),
}

impl Database {
    /// 创建一个新的 PostgreSQL 数据库连接
    ///
    /// ## 参数
    ///
    /// - `url`: PostgreSQL 数据库连接字符串，格式：`postgresql://user:password@host/database`
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<Database>`，成功时包含数据库连接实例，失败时返回错误信息。
    ///
    /// ## 连接池配置
    ///
    /// 默认最大连接数：10
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// use rf_database::db::Database;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let db = Database::new_postgres("postgresql://user:pass@localhost/mydb").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_postgres(url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await
            .map_err(|e| RfError::Database(format!("Failed to connect to PostgreSQL: {}", e)))?;
        Ok(Self {
            pool: DatabasePool::Postgres(pool),
            db_type: DatabaseType::Postgres,
        })
    }

    /// 创建一个新的 MySQL 数据库连接
    ///
    /// ## 参数
    ///
    /// - `url`: MySQL 数据库连接字符串，格式：`mysql://user:password@host/database`
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<Database>`，成功时包含数据库连接实例，失败时返回错误信息。
    ///
    /// ## 连接池配置
    ///
    /// 默认最大连接数：10
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// use rf_database::db::Database;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let db = Database::new_mysql("mysql://user:pass@localhost/mydb").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_mysql(url: &str) -> Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await
            .map_err(|e| RfError::Database(format!("Failed to connect to MySQL: {}", e)))?;
        Ok(Self {
            pool: DatabasePool::MySql(pool),
            db_type: DatabaseType::MySql,
        })
    }

    /// 创建一个新的 SQLite 数据库连接
    ///
    /// ## 参数
    ///
    /// - `url`: SQLite 数据库连接字符串，格式：`sqlite://path/to/database.sqlite`
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<Database>`，成功时包含数据库连接实例，失败时返回错误信息。
    ///
    /// ## 连接池配置
    ///
    /// 默认最大连接数：10
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// use rf_database::db::Database;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let db = Database::new_sqlite("sqlite://path/to/mydb.sqlite").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new_sqlite(url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await
            .map_err(|e| RfError::Database(format!("Failed to connect to SQLite: {}", e)))?;
        Ok(Self {
            pool: DatabasePool::Sqlite(pool),
            db_type: DatabaseType::Sqlite,
        })
    }

    /// 为指定表创建一个 ORM 模型
    ///
    /// ## 参数
    ///
    /// - `table`: 表名
    ///
    /// ## 返回值
    ///
    /// 返回一个 `Model` 实例，用于对该表进行 ORM 操作。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// use rf_database::db::Database;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let db = Database::new_postgres("postgresql://user:pass@localhost/db").await?;
    /// let user_model = db.model("users");
    /// # Ok(())
    /// # }
    /// ```
    pub fn model(&self, table: &str) -> crate::db::model::Model {
        crate::db::model::Model::new(self, table.to_string())
    }

    /// 获取数据库类型
    ///
    /// ## 返回值
    ///
    /// 返回 `DatabaseType` 枚举值，标识当前数据库的类型。
    pub fn db_type(&self) -> &DatabaseType {
        &self.db_type
    }
}

impl Database {
    /// 获取 PostgreSQL 连接池
    ///
    /// ## 返回值
    ///
    /// - `Some(&Pool<Postgres>)`: 如果是 PostgreSQL 数据库，返回连接池的引用
    /// - `None`: 如果不是 PostgreSQL 数据库
    pub fn as_postgres(&self) -> Option<&Pool<Postgres>> {
        match &self.pool {
            DatabasePool::Postgres(pool) => Some(pool),
            _ => None,
        }
    }

    /// 获取 MySQL 连接池
    ///
    /// ## 返回值
    ///
    /// - `Some(&Pool<MySql>)`: 如果是 MySQL 数据库，返回连接池的引用
    /// - `None`: 如果不是 MySQL 数据库
    pub fn as_mysql(&self) -> Option<&Pool<MySql>> {
        match &self.pool {
            DatabasePool::MySql(pool) => Some(pool),
            _ => None,
        }
    }

    /// 获取 SQLite 连接池
    ///
    /// ## 返回值
    ///
    /// - `Some(&Pool<Sqlite>)`: 如果是 SQLite 数据库，返回连接池的引用
    /// - `None`: 如果不是 SQLite 数据库
    pub fn as_sqlite(&self) -> Option<&Pool<Sqlite>> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => Some(pool),
            _ => None,
        }
    }

    /// 执行原始 SQL 查询（仅支持 PostgreSQL）
    ///
    /// ## 参数
    ///
    /// - `sql`: SQL 查询语句
    ///
    /// ## 泛型参数
    ///
    /// - `T`: 返回的行数据类型，必须实现 `FromRow` trait
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<Vec<T>>`，成功时包含查询结果集，失败时返回错误信息。
    ///
    /// ## 注意事项
    ///
    /// 此方法当前仅支持 PostgreSQL，其他数据库请使用对应的专用方法。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::db::Database;
    /// # use sqlx::FromRow;
    /// #
    /// # #[derive(Debug, FromRow)]
    /// # struct User {
    /// #     id: i32,
    /// #     name: String,
    /// # }
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let db = Database::new_postgres("postgresql://user:pass@localhost/db").await?;
    /// let users: Vec<User> = db.raw_query("SELECT * FROM users WHERE age > 18").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn raw_query<T>(&self, sql: &str) -> Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        if let Some(pool) = self.as_postgres() {
            let rows = sqlx::query_as::<_, T>(sql)
                .fetch_all(pool)
                .await
                .map_err(|e| RfError::Database(format!("Raw query failed: {}", e)))?;
            Ok(rows)
        } else {
            Err(RfError::Database("Raw query is currently only supported for PostgreSQL".to_string()))
        }
    }

    /// 执行原始 SQL 查询并返回单个结果（仅支持 PostgreSQL）
    ///
    /// ## 参数
    ///
    /// - `sql`: SQL 查询语句
    ///
    /// ## 泛型参数
    ///
    /// - `T`: 返回的行数据类型，必须实现 `FromRow` trait
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<Option<T>>`：
    /// - `Some(T)`: 查询到结果
    /// - `None`: 未查询到结果
    /// - `Err(...)`: 查询失败
    ///
    /// ## 注意事项
    ///
    /// 此方法当前仅支持 PostgreSQL，其他数据库请使用对应的专用方法。
    pub async fn raw_query_one<T>(&self, sql: &str) -> Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        if let Some(pool) = self.as_postgres() {
            let row = sqlx::query_as::<_, T>(sql)
                .fetch_optional(pool)
                .await
                .map_err(|e| RfError::Database(format!("Raw query failed: {}", e)))?;
            Ok(row)
        } else {
            Err(RfError::Database("Raw query is currently only supported for PostgreSQL".to_string()))
        }
    }

    /// 执行原始 SQL（INSERT/UPDATE/DELETE）
    ///
    /// ## 参数
    ///
    /// - `sql`: SQL 执行语句（INSERT、UPDATE 或 DELETE）
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<u64>`，成功时返回受影响的行数，失败时返回错误信息。
    ///
    /// ## 支持的数据库
    ///
    /// 此方法支持 PostgreSQL、MySQL 和 SQLite。
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::db::Database;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let db = Database::new_postgres("postgresql://user:pass@localhost/db").await?;
    /// let affected = db.raw_execute("UPDATE users SET age = age + 1 WHERE id = 1").await?;
    /// println!("Affected rows: {}", affected);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn raw_execute(&self, sql: &str) -> Result<u64> {
        if let Some(pool) = self.as_postgres() {
            let result = sqlx::query(sql)
                .execute(pool)
                .await
                .map_err(|e| RfError::Database(format!("Raw execute failed: {}", e)))?;
            Ok(result.rows_affected())
        } else if let Some(pool) = self.as_mysql() {
            let result = sqlx::query(sql)
                .execute(pool)
                .await
                .map_err(|e| RfError::Database(format!("Raw execute failed: {}", e)))?;
            Ok(result.rows_affected())
        } else if let Some(pool) = self.as_sqlite() {
            let result = sqlx::query(sql)
                .execute(pool)
                .await
                .map_err(|e| RfError::Database(format!("Raw execute failed: {}", e)))?;
            Ok(result.rows_affected())
        } else {
            Err(RfError::Database("Unsupported database type".to_string()))
        }
    }
}
