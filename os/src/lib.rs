//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF 操作系统模块
//!
//! 本模块提供与操作系统相关的功能，包括文件操作、配置管理、进程管理、会话管理、时间处理等。
//!
//! ## 主要功能模块
//!
//! - **build**: 构建信息
//! - **cache**: 缓存系统
//! - **cfg**: 配置管理
//! - **cmd**: 命令行解析
//! - **cmd_exec**: 命令执行
//! - **cron**: 定时任务调度
//! - **ctx**: 上下文管理
//! - **env**: 环境变量操作
//! - **file**: 文件操作
//! - **fpool**: 文件池管理
//! - **fsnotify**: 文件系统通知
//! - **log**: 日志系统
//! - **metric**: 指标收集
//! - **metric_otel**: OpenTelemetry 指标
//! - **mlock**: 内存锁
//! - **mutex**: 互斥锁封装
//! - **proc**: 进程管理
//! - **res**: 资源管理
//! - **rpool**: 运行时池（任务池）
//! - **session**: 会话管理
//! - **spath**: 路径工具
//! - **structs**: 结构体工具
//! - **time**: 时间处理
//! - **timer**: 定时器工具
//! - **view**: 视图模板引擎
//!
//! @author TimonQWQ
//! @date 2026-01-06

// ========== 模块声明 ==========

pub mod build;
pub mod cache;
pub mod cfg;
pub mod cmd;
pub mod cmd_exec;
pub mod cron;
pub mod ctx;
pub mod env;
pub mod file;
pub mod fpool;
pub mod fsnotify;
pub mod log;
pub mod metric;
pub mod metric_otel;
pub mod mlock;
pub mod mutex;
pub mod proc;
pub mod res;
pub mod rpool;
// ========== Session 模块（包含内联定义）==========

/// Session 模块
///
/// 提供会话管理功能，包括会话存储、会话管理器等。
pub mod session {
    /// Session 存储子模块
    pub mod storage;

    // 导出 storage 模块的所有公共内容
    pub use storage::*;

    use axum_sessions::async_session::Session;
    use rf_errors::Result;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// Session 存储接口
    ///
    /// 定义了 Session 存储的基本操作，包括获取、存储和删除会话。
    ///
    /// # 方法
    ///
    /// - `get`: 根据 ID 获取会话
    /// - `store`: 存储会话
    /// - `delete`: 删除会话
    pub trait SessionStorage: Send + Sync {
        /// 根据 ID 获取会话
        ///
        /// # 参数
        ///
        /// - `id`: 会话 ID
        ///
        /// # 返回值
        ///
        /// 返回 `Result<Option<Session>>`，如果会话存在则返回 Some，否则返回 None
        fn get(&self, id: &str) -> Result<Option<Session>>;

        /// 存储会话
        ///
        /// # 参数
        ///
        /// - `session`: 要存储的会话对象
        ///
        /// # 返回值
        ///
        /// 返回 `Result<()>`，表示操作成功或失败
        fn store(&self, session: Session) -> Result<()>;

        /// 删除会话
        ///
        /// # 参数
        ///
        /// - `id`: 要删除的会话 ID
        ///
        /// # 返回值
        ///
        /// 返回 `Result<()>`，表示操作成功或失败
        fn delete(&self, id: &str) -> Result<()>;
    }

    /// 基于内存的 Session 存储
    ///
    /// 使用内存中的 HashMap 存储 Session，适用于单机应用和测试环境。
    ///
    /// # 字段
    ///
    /// - `sessions`: 存储会话的 HashMap
    pub struct MemorySessionStorage {
        sessions: Arc<RwLock<HashMap<String, Session>>>,
    }

    impl MemorySessionStorage {
        /// 创建新的内存 Session 存储
        ///
        /// # 返回值
        ///
        /// 返回一个 `MemorySessionStorage` 实例
        pub fn new() -> Self {
            Self {
                sessions: Arc::new(RwLock::new(HashMap::new())),
            }
        }
    }

    impl SessionStorage for MemorySessionStorage {
        fn get(&self, id: &str) -> Result<Option<Session>> {
            let sessions = futures::executor::block_on(self.sessions.read());
            Ok(sessions.get(id).cloned())
        }

        fn store(&self, session: Session) -> Result<()> {
            let mut sessions = futures::executor::block_on(self.sessions.write());
            let session_id = session.id().to_string();
            sessions.insert(session_id, session);
            Ok(())
        }

        fn delete(&self, id: &str) -> Result<()> {
            let mut sessions = futures::executor::block_on(self.sessions.write());
            sessions.remove(id);
            Ok(())
        }
    }

    impl Default for MemorySessionStorage {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Session 管理器
    ///
    /// 管理 Session 的生命周期，提供统一的 Session 操作接口。
    ///
    /// # 字段
    ///
    /// - `storage`: Session 存储后端
    pub struct SessionManager {
        storage: Box<dyn SessionStorage>,
    }

    impl SessionManager {
        /// 创建使用内存存储的 Session 管理器
        ///
        /// # 返回值
        ///
        /// 返回一个使用 `MemorySessionStorage` 的 `SessionManager` 实例
        pub fn new() -> Self {
            Self {
                storage: Box::new(MemorySessionStorage::new()),
            }
        }

        /// 创建使用自定义存储的 Session 管理器
        ///
        /// # 参数
        ///
        /// - `storage`: 自定义的 Session 存储实现
        ///
        /// # 返回值
        ///
        /// 返回一个使用指定存储的 `SessionManager` 实例
        pub fn with_storage(storage: Box<dyn SessionStorage>) -> Self {
            Self { storage }
        }

        /// 根据 ID 获取会话
        ///
        /// # 参数
        ///
        /// - `id`: 会话 ID
        ///
        /// # 返回值
        ///
        /// 返回 `Result<Option<Session>>`，如果会话存在则返回 Some，否则返回 None
        pub fn get(&self, id: &str) -> Result<Option<Session>> {
            self.storage.get(id)
        }

        /// 存储会话
        ///
        /// # 参数
        ///
        /// - `session`: 要存储的会话对象
        ///
        /// # 返回值
        ///
        /// 返回 `Result<()>`，表示操作成功或失败
        pub fn store(&self, session: Session) -> Result<()> {
            self.storage.store(session)
        }

        /// 删除会话
        ///
        /// # 参数
        ///
        /// - `id`: 要删除的会话 ID
        ///
        /// # 返回值
        ///
        /// 返回 `Result<()>`，表示操作成功或失败
        pub fn delete(&self, id: &str) -> Result<()> {
            self.storage.delete(id)
        }
    }

    impl Default for SessionManager {
        fn default() -> Self {
            Self::new()
        }
    }
}
pub mod spath;
pub mod structs;
pub mod time;
pub mod timer;
pub mod view;

// ========== 导出公共接口 ==========
//
// 使用特定名称重新导出，避免命名冲突

pub use build::{info as build_info};
pub use cache::*;
pub use cfg::*;
pub use cmd::*;
pub use cron::*;
pub use ctx::*;
pub use env::*;
pub use file::*;
pub use fpool::*;
pub use fsnotify::*;
pub use log::*;
pub use metric::*;
pub use mlock::*;
pub use mutex::*;
pub use proc::*;
pub use res::*;
pub use rpool::*;
pub use session::*;
// 使用特定导入避免与 file 模块冲突
pub use spath::{join as path_join, abs as path_abs};
pub use structs::*;
pub use time::*;
pub use timer::*;
pub use view::*;
