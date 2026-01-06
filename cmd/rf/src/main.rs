//! # main
//!
//! main 模块 - RF 框架的命令行工具主入口
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF CLI 工具
//!
//! 这是 RF 框架的命令行接口，提供以下功能：
//! - 项目初始化 (Init)
//! - 代码生成 (Gen)
//! - 数据库迁移 (Migrate)
//! - 服务管理 (Service)

mod migration {
    pub mod engine;
}

mod migration_rs;

mod gen {
    pub mod database;
    pub mod generator;
    pub mod templates;
}

use clap::{Parser, Subcommand};

/// CLI 命令行参数结构体
///
/// 这是 RF CLI 工具的根命令结构，使用 clap 库进行命令行参数解析。
#[derive(Parser)]
#[command(name = "rf")]
#[command(about = "RF Framework CLI Tool", version = "0.1.0")]
struct Cli {
    /// 子命令
    #[command(subcommand)]
    command: Commands,
}

/// CLI 子命令枚举
///
/// 定义了所有可用的顶层命令：
/// - Init: 初始化新项目
/// - Gen: 代码生成
/// - Migrate: 数据库迁移管理
/// - Service: 服务管理
#[derive(Subcommand)]
enum Commands {
    /// 初始化一个新的 RF 项目
    ///
    /// 创建标准的项目目录结构，包括：
    /// - src/: 源代码目录
    /// - config/: 配置文件目录
    /// - Cargo.toml: Rust 项目配置文件
    /// - README.md: 项目说明文档
    Init {
        /// 项目名称
        /// 如果不指定，默认使用 "rf-project"
        name: Option<String>,
        /// 项目目录
        /// 如果不指定，默认使用项目名称作为目录名
        #[arg(short, long)]
        dir: Option<String>,
    },
    /// 从数据库生成代码
    ///
    /// 支持生成：
    /// - Model: 数据模型结构体
    /// - DAO: 数据访问对象
    Gen {
        #[command(subcommand)]
        command: GenCommands,
    },
    /// 数据库迁移命令
    ///
    /// 提供完整的数据库迁移管理功能：
    /// - Create: 创建新迁移文件
    /// - Up: 执行待处理的迁移
    /// - Down: 回滚最后一个迁移
    /// - Status: 查看迁移状态
    /// - Rollback: 回滚到指定版本
    Migrate {
        #[command(subcommand)]
        command: MigrateCommands,
    },
    /// 服务管理命令
    ///
    /// 用于管理 RF 应用的运行状态
    Service {
        #[command(subcommand)]
        command: ServiceCommands,
    },
}

/// 数据库迁移子命令
///
/// 定义了所有迁移相关的子命令
#[derive(Subcommand)]
enum MigrateCommands {
    /// 创建新的迁移文件
    ///
    /// 创建一个带时间戳的 Rust 迁移文件，包含 up() 和 down() 函数
    ///
    /// # 示例
    ///
    /// ```bash
    /// rf migrate create add_users_table
    /// rf migrate create add_users_table --dir custom_migrations
    /// ```
    Create {
        /// 迁移名称
        /// 将被用于生成文件名：VERSION_NAME.rs
        name: String,
        /// 迁移文件存储目录
        /// 默认为 "migrations"
        #[arg(short, long, default_value = "migrations")]
        dir: String,
    },
    /// 执行待处理的迁移
    ///
    /// 按顺序执行所有未应用的迁移脚本
    ///
    /// # 示例
    ///
    /// ```bash
    /// rf migrate up --db postgresql://user:pass@localhost/db
    /// rf migrate up --dir custom_migrations
    /// ```
    Up {
        /// 数据库连接 URL
        /// 格式：postgresql://user:password@host/database
        /// 如果不指定，默认使用 "postgresql://localhost/test"
        #[arg(short, long)]
        db: Option<String>,
        /// 迁移文件目录
        #[arg(short = 'd', long, default_value = "migrations")]
        dir: String,
    },
    /// 回滚最后一个迁移
    ///
    /// 执行最近一次迁移的 down() 函数，撤销该迁移
    ///
    /// # 示例
    ///
    /// ```bash
    /// rf migrate down --db postgresql://user:pass@localhost/db
    /// ```
    Down {
        /// 数据库连接 URL
        #[arg(short, long)]
        db: Option<String>,
        /// 迁移文件目录
        #[arg(short = 'd', long, default_value = "migrations")]
        dir: String,
    },
    /// 显示迁移状态
    ///
    /// 列出所有迁移及其应用状态，包括：
    /// - Version: 迁移版本号（时间戳）
    /// - Name: 迁移名称
    /// - Status: 状态（Applied/Pending）
    /// - Applied At: 应用时间
    ///
    /// # 示例
    ///
    /// ```bash
    /// rf migrate status --db postgresql://user:pass@localhost/db
    /// ```
    Status {
        /// 数据库连接 URL
        #[arg(short, long)]
        db: Option<String>,
        /// 迁移文件目录
        #[arg(short = 'd', long, default_value = "migrations")]
        dir: String,
    },
    /// 回滚到指定版本
    ///
    /// 回滚所有指定版本之后的迁移，将数据库恢复到该版本状态
    ///
    /// # 示例
    ///
    /// ```bash
    /// rf migrate rollback 20240101120000 --db postgresql://user:pass@localhost/db
    /// ```
    Rollback {
        /// 目标版本号
        /// 将回滚到此版本，格式为时间戳（YYYYMMDDHHMMSS）
        version: String,
        /// 数据库连接 URL
        #[arg(short, long)]
        db: Option<String>,
        /// 迁移文件目录
        #[arg(short = 'd', long, default_value = "migrations")]
        dir: String,
    },
}

/// 代码生成子命令
///
/// 定义了代码生成相关的子命令
#[derive(Subcommand)]
enum GenCommands {
    /// 生成 Model 代码
    ///
    /// 从数据库表结构生成 Rust 数据模型，包括：
    /// - 结构体定义（derive Debug, Clone, Serialize, Deserialize, FromRow）
    /// - 字段类型映射（自动转换数据库类型到 Rust 类型）
    /// - 字段注释（从数据库注释提取）
    ///
    /// # 示例
    ///
    /// ```bash
    /// # 生成所有表的模型
    /// rf gen model --db postgresql://localhost/test
    ///
    /// # 生成指定表的模型
    /// rf gen model --db postgresql://localhost/test --tables users,posts
    ///
    /// # 排除某些表
    /// rf gen model --db postgresql://localhost/test --exclude migrations,temp_*
    ///
    /// # 指定输出目录和表前缀
    /// rf gen model --db postgresql://localhost/test --output src/models --prefix app_
    /// ```
    Model {
        /// 数据库连接 URL
        #[arg(short, long)]
        db: Option<String>,
        /// 输出目录
        /// 生成的模型文件将保存在此目录下的 model/ 子目录中
        #[arg(short, long, default_value = "generated")]
        output: String,
        /// 要生成的表名列表（逗号分隔）
        /// 如果为空，则生成所有表
        #[arg(short, long)]
        tables: Option<String>,
        /// 要排除的表名列表（逗号分隔）
        #[arg(short = 'x', long)]
        exclude: Option<String>,
        /// 表名前缀过滤
        /// 只生成以此前缀开头的表
        #[arg(short, long)]
        prefix: Option<String>,
        /// Schema 名称
        /// 默认为 public
        #[arg(short = 's', long)]
        schema: Option<String>,
    },
    /// 生成 DAO 代码
    ///
    /// 从数据库表结构生成数据访问对象（DAO），提供：
    /// - DAO 结构体定义
    /// - 基础 CRUD 方法
    /// - 数据库访问辅助方法
    ///
    /// # 示例
    ///
    /// ```bash
    /// # 生成所有表的 DAO
    /// rf gen dao --db postgresql://localhost/test
    ///
    /// # 生成指定表的 DAO
    /// rf gen dao --db postgresql://localhost/test --tables users,posts
    /// ```
    Dao {
        /// 数据库连接 URL
        #[arg(short, long)]
        db: Option<String>,
        /// 输出目录
        /// 生成的 DAO 文件将保存在此目录下的 dao/ 子目录中
        #[arg(short, long, default_value = "generated")]
        output: String,
        /// 要生成的表名列表（逗号分隔）
        #[arg(short, long)]
        tables: Option<String>,
        /// 要排除的表名列表（逗号分隔）
        #[arg(short = 'x', long)]
        exclude: Option<String>,
        /// 表名前缀过滤
        #[arg(short, long)]
        prefix: Option<String>,
        /// Schema 名称
        #[arg(short = 's', long)]
        schema: Option<String>,
    },
}

/// 服务管理子命令
///
/// 定义了服务管理相关的子命令
#[derive(Subcommand)]
enum ServiceCommands {
    /// 启动服务
    ///
    /// 启动 RF 应用服务
    Start,
    /// 停止服务
    ///
    /// 停止正在运行的 RF 应用服务
    Stop,
    /// 重启服务
    ///
    /// 重启 RF 应用服务
    Restart,
    /// 显示服务状态
    ///
    /// 显示 RF 应用服务的运行状态
    Status,
}

/// 程序主入口
///
/// 使用 tokio 异步运行时，解析命令行参数并分发到相应的处理函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 根据子命令分发到相应的处理函数
    match cli.command {
        Commands::Init { name, dir } => {
            init_project(name, dir).await?;
        }
        Commands::Gen { command } => {
            handle_gen(command).await?;
        }
        Commands::Migrate { command } => {
            handle_migrate(command).await?;
        }
        Commands::Service { command } => {
            handle_service(command).await?;
        }
    }

    Ok(())
}

/// 初始化 RF 项目
///
/// 创建标准的项目目录结构和基础文件
///
/// # 参数
///
/// * `name` - 项目名称，如果为 None 则使用 "rf-project"
/// * `dir` - 项目目录路径，如果为 None 则使用项目名称作为目录
///
/// # 返回
///
/// 成功返回 Ok(())，失败返回错误
///
/// # 创建的文件结构
///
/// ```text
/// project_dir/
/// ├── Cargo.toml       # Rust 项目配置
/// ├── README.md        # 项目说明文档
/// └── src/
///     └── main.rs      # 主程序入口
/// ```
///
/// # 示例
///
/// ```rust
/// // 使用默认名称
/// init_project(None, None).await?;
///
/// // 指定名称和目录
/// init_project(Some("my_app".to_string()), Some("./my_app".to_string())).await?;
/// ```
async fn init_project(name: Option<String>, dir: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let project_name = name.unwrap_or_else(|| "rf-project".to_string());
    let project_dir = dir.unwrap_or_else(|| project_name.clone());
    
    println!("Initializing RF project: {} in {}", project_name, project_dir);
    
    // Create project structure
    std::fs::create_dir_all(&project_dir)?;
    std::fs::create_dir_all(format!("{}/src", project_dir))?;
    std::fs::create_dir_all(format!("{}/config", project_dir))?;
    
    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
rf-core = {{ path = "../rf/core" }}
rf-net = {{ path = "../rf/net" }}
rf-database = {{ path = "../rf/database" }}
rf-os = {{ path = "../rf/os" }}
rf-util = {{ path = "../rf/util" }}
rf-errors = {{ path = "../rf/errors" }}
tokio = {{ version = "1.35", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
"#, project_name);
    std::fs::write(format!("{}/Cargo.toml", project_dir), cargo_toml)?;
    
    // Create main.rs
    let main_rs = r#"use rf_net::http::HttpServer;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let server = HttpServer::new(addr);
    
    println!("Server starting on {}", addr);
    server.serve().await?;
    
    Ok(())
}
"#;
    std::fs::write(format!("{}/src/main.rs", project_dir), main_rs)?;
    
    // Create README.md
    let readme = format!(r#"# {}

RF Framework Project

## Getting Started

```bash
cargo run
```

Server will start on http://127.0.0.1:8080
"#, project_name);
    std::fs::write(format!("{}/README.md", project_dir), readme)?;
    
    println!("Project initialized successfully!");
    Ok(())
}

/// 处理代码生成命令
///
/// 根据子命令类型（Model 或 DAO）生成相应的代码
///
/// # 参数
///
/// * `command` - 代码生成子命令
///
/// # 返回
///
/// 成功返回 Ok(())，失败返回错误
///
/// # 功能说明
///
/// 该函数会：
/// 1. 连接到数据库
/// 2. 创建 SchemaInspector 实例
/// 3. 根据命令配置生成选项
/// 4. 调用 CodeGenerator 生成代码
async fn handle_gen(command: GenCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        GenCommands::Model { db, output, tables, exclude, prefix, schema } => {
            let db_url = db.unwrap_or_else(|| "postgresql://localhost/test".to_string());
            println!("Generating models from database: {}", db_url);
            
            // Connect to database
            let database = rf_database::db::Database::new_postgres(&db_url).await?;
            let inspector = gen::database::SchemaInspector::new(database);

            // Build options
            let options = gen::generator::GenOptions {
                output_dir: std::path::PathBuf::from(&output),
                model_dir: Some(std::path::PathBuf::from(&output).join("model")),
                dao_dir: None,
                table_prefix: prefix,
                table_suffix: None,
                only_tables: tables.as_ref().map(|s| s.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default(),
                exclude_tables: exclude.as_ref().map(|s| s.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default(),
                schema,
                naming_style: gen::generator::NamingStyle::PascalCase,
            };
            
            // Generate code
            let generator = gen::generator::CodeGenerator::new(inspector, options);
            generator.generate_all().await?;
            
            println!("Model generation completed!");
        }
        GenCommands::Dao { db, output, tables, exclude, prefix, schema } => {
            let db_url = db.unwrap_or_else(|| "postgresql://localhost/test".to_string());
            println!("Generating DAOs from database: {}", db_url);
            
            // Connect to database
            let database = rf_database::db::Database::new_postgres(&db_url).await?;
            let inspector = gen::database::SchemaInspector::new(database);

            // Build options
            let options = gen::generator::GenOptions {
                output_dir: std::path::PathBuf::from(&output),
                model_dir: None,
                dao_dir: Some(std::path::PathBuf::from(&output).join("dao")),
                table_prefix: prefix,
                table_suffix: None,
                only_tables: tables.as_ref().map(|s| s.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default(),
                exclude_tables: exclude.as_ref().map(|s| s.split(',').map(|s| s.trim().to_string()).collect()).unwrap_or_default(),
                schema,
                naming_style: gen::generator::NamingStyle::PascalCase,
            };
            
            // Generate code
            let generator = gen::generator::CodeGenerator::new(inspector, options);
            generator.generate_all().await?;
            
            println!("DAO generation completed!");
        }
    }
    Ok(())
}

/// 处理数据库迁移命令
///
/// 根据子命令类型执行相应的迁移操作
///
/// # 参数
///
/// * `command` - 迁移子命令
///
/// # 返回
///
/// 成功返回 Ok(())，失败返回错误
///
/// # 功能说明
///
/// 支持的迁移操作：
/// - Create: 创建新的迁移文件
/// - Up: 执行待处理的迁移
/// - Down: 回滚最后一个迁移
/// - Status: 显示迁移状态
/// - Rollback: 回滚到指定版本
async fn handle_migrate(command: MigrateCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        MigrateCommands::Create { name, dir } => {
            let manager = migration_rs::MigrationManager::new(&dir);
            let filepath = manager.create_migration(&name)?;
            println!("Created migration: {}", filepath.display());
        }
        MigrateCommands::Up { db, dir } => {
            let db_url = db.unwrap_or_else(|| "postgresql://localhost/test".to_string());
            println!("Running migrations from {}...", dir);
            
            // Connect to database
            let database = rf_database::db::Database::new_postgres(&db_url).await?;
            let engine = migration::engine::MigrationEngine::new(database);
            engine.init().await?;
            
            // Get all migrations
            let manager = migration_rs::MigrationManager::new(&dir);
            let migrations = manager.list_migrations()?;
            
            // Get applied migrations
            let applied = engine.get_applied_migrations().await?;
            let applied_versions: std::collections::HashSet<String> = applied
                .iter()
                .map(|m| m.version.clone())
                .collect();
            
            // Apply pending migrations
            let mut applied_count = 0;
            for migration in &migrations {
                if !applied_versions.contains(&migration.version) {
                    println!("Applying migration: {} ({})", migration.name, migration.version);
                    engine.apply_migration(migration).await?;
                    applied_count += 1;
                }
            }
            
            if applied_count == 0 {
                println!("No pending migrations");
            } else {
                println!("Applied {} migration(s)", applied_count);
            }
        }
        MigrateCommands::Down { db, dir } => {
            let db_url = db.unwrap_or_else(|| "postgresql://localhost/test".to_string());
            println!("Rolling back last migration from {}...", dir);
            
            // Connect to database
            let database = rf_database::db::Database::new_postgres(&db_url).await?;
            let engine = migration::engine::MigrationEngine::new(database);
            engine.init().await?;
            
            // Get all migrations
            let manager = migration_rs::MigrationManager::new(&dir);
            let migrations = manager.list_migrations()?;
            
            // Get applied migrations
            let applied = engine.get_applied_migrations().await?;
            if let Some(last_migration) = applied.last() {
                // Find the migration to rollback
                if let Some(migration) = migrations.iter().find(|m| m.version == last_migration.version) {
                    println!("Rolling back migration: {} ({})", migration.name, migration.version);
                    engine.rollback_migration(migration).await?;
                    println!("Rolled back successfully");
                } else {
                    println!("Migration {} not found in files", last_migration.version);
                }
            } else {
                println!("No migrations to rollback");
            }
        }
        MigrateCommands::Status { db, dir } => {
            let db_url = db.unwrap_or_else(|| "postgresql://localhost/test".to_string());
            println!("Migration status from {}:", dir);
            
            // Connect to database
            let database = rf_database::db::Database::new_postgres(&db_url).await?;
            let engine = migration::engine::MigrationEngine::new(database);
            engine.init().await?;
            
            // Get all migrations
            let manager = migration_rs::MigrationManager::new(&dir);
            let migrations = manager.list_migrations()?;
            
            // Get status
            let status = engine.get_status(&migrations).await?;
            
            println!("\n{:<20} {:<30} {:<10} {:<20}", "Version", "Name", "Status", "Applied At");
            println!("{}", "-".repeat(80));
            for s in status {
                let status_str = if s.applied { "Applied" } else { "Pending" };
                let applied_at_str = s.applied_at
                    .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                println!("{:<20} {:<30} {:<10} {:<20}", s.version, s.name, status_str, applied_at_str);
            }
        }
        MigrateCommands::Rollback { version, db, dir } => {
            let db_url = db.unwrap_or_else(|| "postgresql://localhost/test".to_string());
            println!("Rolling back to version {} from {}...", version, dir);
            
            // Connect to database
            let database = rf_database::db::Database::new_postgres(&db_url).await?;
            let engine = migration::engine::MigrationEngine::new(database);
            engine.init().await?;
            
            // Get all migrations
            let manager = migration_rs::MigrationManager::new(&dir);
            let migrations = manager.list_migrations()?;
            
            // Rollback to target version
            engine.rollback_to(&version, &migrations).await?;
            println!("Rolled back to version {}", version);
        }
    }
    Ok(())
}

/// 处理服务管理命令
///
/// 根据子命令类型执行相应的服务管理操作
///
/// # 参数
///
/// * `command` - 服务管理子命令
///
/// # 返回
///
/// 成功返回 Ok(())，失败返回错误
///
/// # 功能说明
///
/// 支持的服务操作：
/// - Start: 启动服务
/// - Stop: 停止服务
/// - Restart: 重启服务
/// - Status: 显示服务状态
async fn handle_service(command: ServiceCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ServiceCommands::Start => {
            println!("Starting service...");
        }
        ServiceCommands::Stop => {
            println!("Stopping service...");
        }
        ServiceCommands::Restart => {
            println!("Restarting service...");
        }
        ServiceCommands::Status => {
            println!("Service status: running");
        }
    }
    Ok(())
}

