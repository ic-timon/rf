//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 数据库驱动扩展模块
//!
//! 提供标准 PostgreSQL、MySQL 和 SQLite 之外的额外数据库驱动：
//! - ClickHouse：用于联机分析处理（OLAP）的列式数据库
//! - Oracle：企业级关系型数据库
//! - SQL Server：微软的关系型数据库管理系统
//! - TiDB：兼容 MySQL 的分布式数据库
//! - OceanBase：兼容 MySQL/Oracle 的分布式关系数据库
//! - Dameng（达梦）：国产关系型数据库
//! - GaussDB：华为高斯数据库，兼容 PostgreSQL

pub mod clickhouse;
pub mod oracle;
pub mod sqlserver;
pub mod tidb;
pub mod oceanbase;
pub mod dameng;
pub mod gaussdb;

pub use clickhouse::*;
pub use oracle::*;
pub use sqlserver::*;
pub use tidb::*;
pub use oceanbase::*;
pub use dameng::*;
pub use gaussdb::*;

use rf_errors::Result;

/// 可扩展的数据库驱动特征
///
/// 该 trait 定义了数据库驱动必须实现的基本接口，用于支持多种数据库系统。
///
/// # 要求
/// - Send：可以在多线程间传递所有权
/// - Sync：可以在多线程间共享引用
pub trait DatabaseDriver: Send + Sync {
    /// 获取驱动名称
    ///
    /// 返回驱动的标识符，用于标识和调试。
    ///
    /// # 返回值
    ///
    /// 返回驱动名称的字符串切片，例如 "mysql"、"postgresql"、"clickhouse" 等
    fn name(&self) -> &str;

    /// 连接到数据库
    ///
    /// 使用给定的连接 URL 建立数据库连接。
    ///
    /// # 参数
    ///
    /// * `url` - 数据库连接字符串，格式因数据库类型而异
    ///   - MySQL: `mysql://user:password@host:port/database`
    ///   - PostgreSQL: `postgresql://user:password@host:port/database`
    ///   - ClickHouse: `clickhouse://user:password@host:port/database`
    ///
    /// # 返回值
    ///
    /// 成功时返回包装在 Box 中的数据库连接对象
    ///
    /// # 错误
    ///
    /// 当连接失败时返回错误，可能的原因包括：
    /// - 网络连接失败
    /// - 认证失败
    /// - 数据库不存在
    /// - 连接字符串格式错误
    fn connect(&self, url: &str) -> Result<Box<dyn DatabaseConnection>>;

    /// 执行查询
    ///
    /// 执行不返回结果集的 SQL 语句（如 INSERT、UPDATE、DELETE）。
    ///
    /// # 参数
    ///
    /// * `query` - SQL 查询语句
    /// * `params` - 查询参数数组，支持任意类型（通过 `Any` trait 对象）
    ///
    /// # 返回值
    ///
    /// 成功时返回空 `Ok(())`
    ///
    /// # 错误
    ///
    /// 当执行失败时返回错误，可能的原因包括：
    /// - SQL 语法错误
    /// - 参数类型不匹配
    /// - 约束违反
    /// - 权限不足
    fn execute(&self, query: &str, params: &[&dyn std::any::Any]) -> Result<()>;
}

/// 数据库连接特征
///
/// 定义了数据库连接的基本操作，包括查询、执行和事务管理。
///
/// # 要求
/// - Send：可以在多线程间传递所有权
/// - Sync：可以在多线程间共享引用
pub trait DatabaseConnection: Send + Sync {
    /// 执行查询并返回结果
    ///
    /// 执行 SELECT 等 SQL 查询语句并返回结果集。
    ///
    /// # 参数
    ///
    /// * `sql` - SQL 查询语句
    /// * `params` - 查询参数数组，支持任意类型（通过 `Any` trait 对象）
    ///
    /// # 返回值
    ///
    /// 成功时返回二维字符串数组，每个内部向量表示一行数据
    /// - 外层 Vec：表示所有行
    /// - 内层 Vec：表示一行中的所有列
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use rf_contrib_drivers::DatabaseConnection;
    /// # fn example(connection: &dyn DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    /// let results = connection.query("SELECT name, age FROM users", &[])?;
    /// for row in results {
    ///     println!("Name: {}, Age: {}", row[0], row[1]);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # 错误
    ///
    /// 当查询失败时返回错误
    fn query(&self, sql: &str, params: &[&dyn std::any::Any]) -> Result<Vec<Vec<String>>>;

    /// 执行 SQL 语句（INSERT、UPDATE、DELETE）
    ///
    /// 执行不返回结果集的 SQL 语句，返回受影响的行数。
    ///
    /// # 参数
    ///
    /// * `sql` - SQL 语句
    /// * `params` - 语句参数数组，支持任意类型
    ///
    /// # 返回值
    ///
    /// 返回受影响的行数：
    /// - INSERT：返回插入的行数
    /// - UPDATE：返回更新的行数
    /// - DELETE：返回删除的行数
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use rf_contrib_drivers::DatabaseConnection;
    /// # fn example(connection: &dyn DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    /// let affected = connection.execute(
    ///     "UPDATE users SET age = ? WHERE id = ?",
    ///     &[&30, &1]
    /// )?;
    /// println!("Updated {} rows", affected);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # 错误
    ///
    /// 当执行失败时返回错误
    fn execute(&self, sql: &str, params: &[&dyn std::any::Any]) -> Result<u64>;

    /// 开始事务
    ///
    /// 创建一个新的事务对象，用于执行一组原子性的数据库操作。
    ///
    /// # 事务特性
    /// - 原子性（Atomicity）：事务中的操作要么全部成功，要么全部失败
    /// - 一致性（Consistency）：事务执行前后数据库保持一致状态
    /// - 隔离性（Isolation）：并发事务之间相互隔离
    /// - 持久性（Durability）：事务提交后，修改永久保存
    ///
    /// # 返回值
    ///
    /// 成功时返回事务对象的 Box 智能指针
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use rf_contrib_drivers::DatabaseConnection;
    /// # fn example(connection: &dyn DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    /// let mut tx = connection.begin_transaction()?;
    /// // 执行多个操作（通过 connection 执行，在事务中）
    /// connection.execute("INSERT INTO accounts ...", &[])?;
    /// connection.execute("UPDATE accounts SET ...", &[])?;
    /// // 提交事务
    /// tx.commit()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # 错误
    ///
    /// 当开始事务失败时返回错误
    fn begin_transaction(&self) -> Result<Box<dyn DatabaseTransaction>>;
}

/// 数据库事务特征
///
/// 定义了事务的基本操作：提交和回滚。
///
/// # 要求
/// - Send：可以在多线程间传递所有权
/// - Sync：可以在多线程间共享引用
pub trait DatabaseTransaction: Send + Sync {
    /// 提交事务
    ///
    /// 将事务中的所有操作永久保存到数据库。
    ///
    /// # 行为
    ///
    /// - 成功提交：所有修改被永久保存，事务结束
    /// - 事务提交后，连接的自动提交模式恢复
    /// - 释放事务持有的所有锁和资源
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`
    ///
    /// # 错误
    ///
    /// 当提交失败时返回错误，可能的原因包括：
    /// - 网络故障
    /// - 死锁
    /// - 约束违反
    fn commit(self: Box<Self>) -> Result<()>;

    /// 回滚事务
    ///
    /// 撤销事务中的所有操作，恢复到事务开始前的状态。
    ///
    /// # 行为
    ///
    /// - 所有修改被撤销，数据库恢复到事务前的状态
    /// - 事务结束
    /// - 释放事务持有的所有锁和资源
    ///
    /// # 使用场景
    ///
    /// - 遇到错误时撤销操作
    /// - 业务逻辑验证失败时回滚
    /// - 用户取消操作时回滚
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Ok(())`
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// # use rf_contrib_drivers::DatabaseTransaction;
    /// # fn example(mut transaction: Box<dyn DatabaseTransaction>) -> Result<(), Box<dyn std::error::Error>> {
    /// // 如果操作成功，提交事务
    /// // transaction.commit()?;
    /// // 如果操作失败，回滚事务
    /// transaction.rollback()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # 错误
    ///
    /// 当回滚失败时返回错误（较少见）
    fn rollback(self: Box<Self>) -> Result<()>;
}

