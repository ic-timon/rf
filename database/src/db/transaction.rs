//! # transaction
//!
//! transaction 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Transaction Management
//!
//! 数据库事务管理模块，支持 ACID 事务操作。
//!
//! ## 功能特性
//!
//! - 原子性（Atomicity）：事务中的操作要么全部成功，要么全部失败
//! - 一致性（Consistency）：事务执行前后数据库保持一致状态
//! - 隔离性（Isolation）：并发事务之间相互隔离
//! - 持久性（Durability）：事务一旦提交，其结果永久保存
//!
//! ## 使用场景
//!
//! - 需要保证数据一致性的多个操作
//! - 银行转账、订单创建等业务场景
//! - 批量数据更新
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use rf_database::db::transaction::TransactionWrapper;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 开始事务
//! // let mut tx = database.begin().await?;
//!
//! // 执行多个操作
//! // tx.execute("UPDATE accounts SET balance = balance - 100 WHERE id = 1").await?;
//! // tx.execute("UPDATE accounts SET balance = balance + 100 WHERE id = 2").await?;
//!
//! // 提交事务
//! // tx.commit().await?;
//! # Ok(())
//! # }
//! ```

use rf_errors::{Result, RfError};
use sqlx::{Transaction, Postgres};

/// 事务包装器
///
/// 封装 PostgreSQL 事务，提供安全的交易操作接口。
///
/// ## 字段说明
///
/// - `transaction`: 底层 PostgreSQL 事务实例
pub struct TransactionWrapper {
    transaction: Transaction<'static, Postgres>,
}

impl TransactionWrapper {
    /// 创建一个新的事务包装器
    ///
    /// ## 参数
    ///
    /// - `transaction`: PostgreSQL 事务实例
    ///
    /// ## 返回值
    ///
    /// 返回 `TransactionWrapper` 实例。
    pub fn new(transaction: Transaction<'static, Postgres>) -> Self {
        Self { transaction }
    }

    /// 在事务中执行查询
    ///
    /// ## 参数
    ///
    /// - `query`: SQL 查询语句（INSERT、UPDATE、DELETE）
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<u64>`，成功时返回受影响的行数，失败时返回错误信息。
    ///
    /// ## 注意事项
    ///
    /// - 此操作不会自动提交，需要显式调用 `commit()` 或 `rollback()`
    /// - 如果事务中的任何操作失败，应调用 `rollback()` 回滚事务
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::db::transaction::TransactionWrapper;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut tx = unsafe { std::mem::zeroed() };
    /// // 更新用户余额
    /// tx.execute("UPDATE users SET balance = balance - 100 WHERE id = 1").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(&mut self, query: &str) -> Result<u64> {
        sqlx::query(query)
            .execute(&mut *self.transaction)
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| RfError::Database(format!("Transaction query failed: {}", e)))
    }

    /// 提交事务
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<()>`，成功时返回空，失败时返回错误信息。
    ///
    /// ## 注意事项
    ///
    /// - 提交后，事务中的所有操作将永久生效
    /// - 提交后事务实例不能再使用
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::db::transaction::TransactionWrapper;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut tx = unsafe { std::mem::zeroed() };
    /// // 所有操作成功后提交
    /// tx.commit().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn commit(self) -> Result<()> {
        self.transaction
            .commit()
            .await
            .map_err(|e| RfError::Database(format!("Transaction commit failed: {}", e)))
    }

    /// 回滚事务
    ///
    /// ## 返回值
    ///
    /// 返回 `Result<()>`，成功时返回空，失败时返回错误信息。
    ///
    /// ## 注意事项
    ///
    /// - 回滚后，事务中的所有操作都将被撤销
    /// - 回滚后事务实例不能再使用
    /// - 如果事务中的任何操作失败，应调用此方法
    ///
    /// ## 使用示例
    ///
    /// ```rust,no_run
    /// # use rf_database::db::transaction::TransactionWrapper;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut tx = unsafe { std::mem::zeroed() };
    /// // 发生错误时回滚
    /// tx.rollback().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rollback(self) -> Result<()> {
        self.transaction
            .rollback()
            .await
            .map_err(|e| RfError::Database(format!("Transaction rollback failed: {}", e)))
    }
}
