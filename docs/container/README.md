# Container 模块教程

Container 模块提供了各种数据结构容器，包括数组、列表、映射、集合、队列、树等。

## 模块概述

Container 模块包含以下容器类型：

- **原子类型包装器（type）**：线程安全的基本类型包装
- **通用变量类型（var）**：类似 GoFrame 的 gvar.Var
- **映射容器（map）**：HashMap 和 OrderedMap
- **数组容器（array）**：基于 SmallVec 的小向量数组
- **链表容器（list）**：双向链表
- **集合容器（set）**：有序集合
- **队列容器（queue）**：线程安全队列
- **环形缓冲区（ring）**：固定大小的环形缓冲区
- **对象池（pool）**：对象池管理器
- **树容器（tree）**：通用树结构

## 快速开始

### 添加依赖

```toml
[dependencies]
rf-container = { path = "../rf/container" }
```

### 基本导入

```rust
use rf_container::{Var, Queue, HashMap, Set};
```

## 核心功能

### Var - 通用变量类型

Var 可以存储任意类型的数据，并提供类型转换方法。

```rust
use rf_container::Var;

// 从各种类型创建 Var
let var1 = Var::new("Hello");
let var2 = Var::new(42i64);
let var3 = Var::new(3.14f64);
let var4 = Var::new(true);

// 类型转换
assert_eq!(var1.string(), "Hello");
assert_eq!(var2.int64(), 42);
assert_eq!(var3.float64(), 3.14);
assert_eq!(var4.bool(), true);

// 检查是否为空
let empty = Var::new(());
assert!(empty.is_empty());
```

### Queue - 线程安全队列

```rust
use rf_container::Queue;

let queue = Queue::new();

// 添加元素
queue.push(1);
queue.push(2);
queue.push(3);

// 获取元素
if let Some(value) = queue.pop() {
    println!("{}", value);
}

// 检查是否为空
assert!(!queue.is_empty());
```

### HashMap - 线程安全映射

```rust
use rf_container::HashMap;

let map = HashMap::new();

// 插入键值对
map.insert("key1".to_string(), "value1".to_string());
map.insert("key2".to_string(), "value2".to_string());

// 获取值
if let Some(value) = map.get("key1") {
    println!("{}", value);
}

// 检查键是否存在
assert!(map.contains_key("key1"));
```

### Set - 有序集合

```rust
use rf_container::Set;

let mut set = Set::new();

// 添加元素
set.insert("apple".to_string());
set.insert("banana".to_string());
set.insert("orange".to_string());

// 检查元素是否存在
assert!(set.contains("apple"));

// 移除元素
set.remove("banana");

// 获取集合大小
println!("集合大小: {}", set.len());
```

### Array - 小向量数组

```rust
use rf_container::Array;

let mut array = Array::new();

// 添加元素
array.push(1);
array.push(2);
array.push(3);

// 访问元素
if let Some(value) = array.get(0) {
    println!("第一个元素: {}", value);
}

// 迭代
for item in array.iter() {
    println!("{}", item);
}
```

### List - 双向链表

```rust
use rf_container::List;

let mut list = List::new();

// 在头部添加
list.push_front(1);
list.push_front(2);

// 在尾部添加
list.push_back(3);
list.push_back(4);

// 从头部移除
if let Some(value) = list.pop_front() {
    println!("头部元素: {}", value);
}

// 从尾部移除
if let Some(value) = list.pop_back() {
    println!("尾部元素: {}", value);
}
```

### Ring - 环形缓冲区

```rust
use rf_container::Ring;

// 创建固定大小的环形缓冲区
let mut ring = Ring::new(5);

// 添加元素（当缓冲区满时，会覆盖最旧的元素）
ring.push(1);
ring.push(2);
ring.push(3);
ring.push(4);
ring.push(5);
ring.push(6); // 覆盖 1

// 获取所有元素
let items: Vec<_> = ring.iter().collect();
println!("{:?}", items);
```

### Pool - 对象池

```rust
use rf_container::Pool;
use std::sync::Arc;

// 创建对象池
let pool = Pool::new(|| {
    // 创建新对象的函数
    String::new()
}, 10); // 最大容量 10

// 从池中获取对象
let obj = pool.get();
// 使用对象...
// 对象会在作用域结束时自动返回池中
```

### Tree - 树结构

```rust
use rf_container::Tree;

let mut tree = Tree::new("root".to_string());

// 添加子节点
let child1 = tree.add_child("child1".to_string());
let child2 = tree.add_child("child2".to_string());

// 在子节点下添加节点
child1.add_child("grandchild".to_string());

// 遍历树
tree.traverse(|node| {
    println!("节点: {}", node.value());
});
```

## 高级用法

### Var 类型转换

```rust
use rf_container::Var;

let var = Var::new("123");

// 转换为整数
if let Ok(num) = var.int64() {
    println!("数字: {}", num);
}

// 转换为浮点数
if let Ok(num) = var.float64() {
    println!("浮点数: {}", num);
}

// 转换为布尔值
let bool_var = Var::new("true");
if let Ok(b) = bool_var.bool() {
    println!("布尔值: {}", b);
}
```

### 并发安全使用

```rust
use rf_container::{Queue, HashMap};
use std::thread;

let queue = Queue::new();
let map = HashMap::new();

// 多线程写入
let handles: Vec<_> = (0..10).map(|i| {
    let queue = queue.clone();
    let map = map.clone();
    thread::spawn(move || {
        queue.push(i);
        map.insert(format!("key{}", i), format!("value{}", i));
    })
}).collect();

// 等待所有线程完成
for handle in handles {
    handle.join().unwrap();
}

// 读取结果
while let Some(value) = queue.pop() {
    println!("{}", value);
}
```

## API 参考

### Var 方法

- `new<T: Serialize>(value: T) -> Self` - 创建新的 Var
- `string() -> String` - 转换为字符串
- `int64() -> Result<i64>` - 转换为 i64
- `float64() -> Result<f64>` - 转换为 f64
- `bool() -> Result<bool>` - 转换为布尔值
- `is_empty() -> bool` - 检查是否为空

### Queue 方法

- `new() -> Self` - 创建新队列
- `push(item: T)` - 添加元素
- `pop() -> Option<T>` - 移除并返回元素
- `is_empty() -> bool` - 检查是否为空
- `len() -> usize` - 获取队列长度

### HashMap 方法

- `new() -> Self` - 创建新映射
- `insert(key: K, value: V)` - 插入键值对
- `get(key: &K) -> Option<V>` - 获取值
- `contains_key(key: &K) -> bool` - 检查键是否存在
- `remove(key: &K) -> Option<V>` - 移除键值对

## 常见问题

### Q: Var 和 serde_json::Value 有什么区别？

A: Var 是对 serde_json::Value 的封装，提供了更便捷的类型转换方法。

### Q: Queue 是线程安全的吗？

A: 是的，Queue 使用内部同步机制，可以在多线程环境中安全使用。

### Q: 什么时候使用 Ring 而不是 Vec？

A: Ring 适用于需要固定大小缓冲区的场景，当缓冲区满时会自动覆盖最旧的元素。

## 相关链接

- [core 模块](../core/README.md) - 核心类型
- [util 模块](../util/README.md) - 工具函数

