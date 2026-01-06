# Database 模块教程

Database 模块提供 ORM 和数据库操作功能，支持 PostgreSQL、MySQL、SQLite 和 Redis。

## 模块概述

Database 模块包含以下功能：

- **ORM 模型**：面向对象的数据建模和查询接口
- **查询构建器**：类型安全的 SQL 查询构建
- **事务管理**：数据库事务支持
- **连接池**：自动管理数据库连接池
- **查询缓存**：查询结果缓存
- **Redis 客户端**：Redis 操作封装
- **多数据库支持**：PostgreSQL、MySQL、SQLite

## 快速开始

### 添加依赖

```toml
[dependencies]
rf-database = { path = "../rf/database" }
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 基本导入

```rust
use rf_database::db::{Database, Model};
```

## 核心功能

### 数据库连接

```rust
use rf_database::db::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PostgreSQL
    let db = Database::new_postgres(
        "postgresql://user:pass@localhost/dbname"
    ).await?;
    
    // MySQL
    let db = Database::new_mysql(
        "mysql://user:pass@localhost/dbname"
    ).await?;
    
    // SQLite
    let db = Database::new_sqlite(
        "sqlite://path/to/db.sqlite"
    ).await?;
    
    Ok(())
}
```

### ORM 模型

#### 基本查询

```rust
use rf_database::db::{Database, Model};

let db = Database::new_postgres("postgresql://...").await?;
let user_model = Model::new(&db, "users".to_string());

// 查询所有记录
let users: Vec<serde_json::Value> = user_model.all().await?;

// 条件查询
let active_users = user_model
    .where_("status", "=", "active")
    .all()
    .await?;

// 排序和限制
let recent_users = user_model
    .order_by("created_at", "DESC")
    .limit(10)
    .all()
    .await?;

// 单条查询
let user = user_model
    .where_("id", "=", "1")
    .one()
    .await?;
```

#### 插入数据

```rust
use serde_json::json;

// 插入单条记录
let new_user = json!({
    "name": "Alice",
    "email": "alice@example.com",
    "age": 25
});

let id = user_model.insert(&new_user).await?;

// 批量插入
let users = vec![
    json!({"name": "Bob", "email": "bob@example.com"}),
    json!({"name": "Charlie", "email": "charlie@example.com"}),
];

user_model.insert_batch(&users).await?;
```

#### 更新数据

```rust
// 更新记录
let updated = json!({
    "name": "Alice Updated",
    "age": 26
});

user_model
    .where_("id", "=", "1")
    .update(&updated)
    .await?;

// 更新或插入（upsert）
user_model
    .where_("email", "=", "alice@example.com")
    .save(&updated)
    .await?;
```

#### 删除数据

```rust
// 删除记录
user_model
    .where_("id", "=", "1")
    .delete()
    .await?;

// 软删除（如果配置了软删除字段）
user_model
    .where_("id", "=", "1")
    .soft_delete()
    .await?;
```

### 查询构建器

```rust
// 复杂查询
let users = user_model
    .where_("age", ">", "18")
    .where_("status", "=", "active")
    .order_by("created_at", "DESC")
    .limit(10)
    .offset(0)
    .all()
    .await?;

// 关联查询
let users_with_posts = user_model
    .with("posts")
    .all()
    .await?;

// 聚合查询
let count = user_model.count().await?;
let avg_age = user_model.avg("age").await?;
```

### 事务管理

```rust
use rf_database::db::Transaction;

// 开始事务
let tx = db.begin().await?;

// 在事务中执行操作
let user_model = Model::new(&db, "users".to_string());
user_model.insert(&new_user).await?;

// 提交事务
tx.commit().await?;

// 或回滚
// tx.rollback().await?;
```

### Redis 客户端

```rust
use rf_database::redis::RedisClient;

// 创建 Redis 客户端
let redis = RedisClient::new("redis://localhost:6379").await?;

// 字符串操作
redis.set("key", "value").await?;
let value: Option<String> = redis.get("key").await?;

// 哈希操作
redis.hset("user:1", "name", "Alice").await?;
let name: Option<String> = redis.hget("user:1", "name").await?;

// 列表操作
redis.lpush("list", "item1").await?;
let item: Option<String> = redis.rpop("list").await?;

// 集合操作
redis.sadd("set", "member1").await?;
let members: Vec<String> = redis.smembers("set").await?;
```

### 查询缓存

```rust
use std::time::Duration;

// 启用查询缓存
let cached_users = user_model
    .cache(Duration::from_secs(60)) // 缓存 60 秒
    .where_("status", "=", "active")
    .all()
    .await?;
```

## 高级用法

### 关联查询

```rust
// 定义关联关系
user_model.with("posts", |query| {
    query.where_("published", "=", "true")
});

// 执行关联查询
let users_with_posts = user_model
    .with("posts")
    .all()
    .await?;
```

### 软删除

```rust
// 配置软删除字段
let user_model = Model::new(&db, "users".to_string())
    .soft_delete_field("deleted_at");

// 查询时排除已删除的记录（默认）
let active_users = user_model.all().await?;

// 包含已删除的记录
let all_users = user_model.with_deleted().all().await?;

// 只查询已删除的记录
let deleted_users = user_model.only_deleted().all().await?;
```

### 原始 SQL

```rust
// 执行原始 SQL
let users: Vec<serde_json::Value> = db
    .query("SELECT * FROM users WHERE age > $1", &[&18i64])
    .await?;
```

## API 参考

### Database

- `new_postgres(url: &str) -> Result<Database>` - 创建 PostgreSQL 连接
- `new_mysql(url: &str) -> Result<Database>` - 创建 MySQL 连接
- `new_sqlite(url: &str) -> Result<Database>` - 创建 SQLite 连接
- `begin() -> Result<Transaction>` - 开始事务

### Model

- `new(database: &Database, table: String) -> Self` - 创建模型
- `where_(field: &str, op: &str, value: &str) -> Self` - 添加条件
- `order_by(field: &str, direction: &str) -> Self` - 排序
- `limit(n: usize) -> Self` - 限制数量
- `all() -> Result<Vec<Value>>` - 查询所有
- `one() -> Result<Option<Value>>` - 查询单条
- `insert(data: &Value) -> Result<i64>` - 插入
- `update(data: &Value) -> Result<()>` - 更新
- `delete() -> Result<()>` - 删除

### RedisClient

- `new(url: &str) -> Result<RedisClient>` - 创建客户端
- `set(key: &str, value: &str) -> Result<()>` - 设置值
- `get(key: &str) -> Result<Option<String>>` - 获取值
- `hset(key: &str, field: &str, value: &str) -> Result<()>` - 设置哈希
- `hget(key: &str, field: &str) -> Result<Option<String>>` - 获取哈希

## 常见问题

### Q: 如何配置连接池大小？

A: 在创建 Database 时可以通过连接字符串参数配置，或使用环境变量。

### Q: 支持哪些数据库？

A: 目前支持 PostgreSQL、MySQL、SQLite。更多数据库请查看 [contrib/drivers](../contrib/drivers/README.md)。

### Q: 如何实现数据库迁移？

A: 使用 CLI 工具的迁移功能，或手动执行 SQL 脚本。

### Q: 查询缓存如何工作？

A: 查询缓存基于查询条件和结果进行哈希，相同查询在缓存有效期内直接返回缓存结果。

## 相关链接

- [net 模块](../net/README.md) - HTTP 服务器
- [os 模块](../os/README.md) - 配置管理
- [contrib/drivers 模块](../contrib/drivers/README.md) - 数据库驱动扩展

