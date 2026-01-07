# RF 框架 Benchmark 测试报告

生成时间: $(date)

## 测试环境

- 操作系统: macOS
- 架构: $(uname -m)
- Rust 版本: $(rustc --version)

## 测试说明

本报告包含 RF 框架各个模块的性能基准测试结果。测试使用 Criterion 基准测试框架进行，每个测试运行多次以获取准确的性能数据。

## Container 模块测试结果

### Array 容器

- `array_push`: 测试向数组添加 1000 个元素的性能
- `array_pop`: 测试从数组弹出元素的性能
- `array_get`: 测试获取数组元素的性能
- `array_len`: 测试获取数组长度的性能

### List 容器

- `list_push_back`: 测试向链表尾部添加元素的性能
- `list_push_front`: 测试向链表头部添加元素的性能
- `list_pop_back`: 测试从链表尾部弹出元素的性能
- `list_pop_front`: 测试从链表头部弹出元素的性能
- `list_len`: 测试获取链表长度的性能

### Queue 容器

- `queue_push`: 测试向队列推入元素的性能
- `queue_pop`: 测试从队列弹出元素的性能
- `queue_concurrent_push`: 测试并发推入的性能
- `queue_concurrent_pop`: 测试并发弹出的性能

### Ring 容器

- `ring_push`: 测试向环形缓冲区推入元素的性能
- `ring_pop`: 测试从环形缓冲区弹出元素的性能
- `ring_get`: 测试获取环形缓冲区元素的性能
- `ring_len`: 测试获取环形缓冲区长度的性能

### Map 容器

- `hashmap_insert`: 测试向 HashMap 插入键值对的性能
- `hashmap_get`: 测试从 HashMap 获取值的性能
- `hashmap_remove`: 测试从 HashMap 删除键值对的性能
- `hashmap_concurrent_insert`: 测试并发插入的性能
- `orderedmap_insert`: 测试向 OrderedMap 插入键值对的性能
- `orderedmap_get`: 测试从 OrderedMap 获取值的性能

### Set 容器

- `set_insert`: 测试向 Set 插入元素的性能
- `set_contains`: 测试检查 Set 是否包含元素的性能
- `set_remove`: 测试从 Set 删除元素的性能
- `set_len`: 测试获取 Set 长度的性能

### Pool 容器

- `pool_get`: 测试从对象池获取对象的性能
- `pool_put`: 测试向对象池归还对象的性能

## OS 模块测试结果

### Cache 缓存

- `cache_insert`: 测试向缓存插入键值对的性能
- `cache_get`: 测试从缓存获取值的性能
- `cache_remove`: 测试从缓存删除键值对的性能

### Mutex 互斥锁

- `std_mutex_lock_unlock`: 标准库 Mutex 加锁解锁性能
- `parking_lot_mutex_lock_unlock`: Parking Lot Mutex 加锁解锁性能
- `std_mutex_concurrent`: 标准库 Mutex 并发性能
- `parking_lot_mutex_concurrent`: Parking Lot Mutex 并发性能

## Encoding 模块测试结果

### JSON 编码/解码

- `json_encode`: JSON 编码性能
- `json_encode_pretty`: JSON 格式化编码性能
- `json_decode`: JSON 解码性能
- `json_parse`: JSON 解析性能

### Hash 哈希

- `xxhash_small`: 小数据 XXHash 哈希性能
- `xxhash_large`: 大数据 XXHash 哈希性能
- `hash_small`: 小数据通用哈希性能
- `hash_large`: 大数据通用哈希性能

## Util 模块测试结果

### GUID 生成

- `guid_new`: 生成标准 UUID 性能
- `guid_new_simple`: 生成简化 UUID 性能

### Random 随机数

- `rand_int`: 生成随机整数性能
- `rand_float`: 生成随机浮点数性能
- `rand_string`: 生成随机字符串性能

## 运行 Benchmark 测试

要运行所有 benchmark 测试，请执行：

```bash
cd /Users/timon/WorkPath/rframe/rf
cargo bench
```

要运行特定模块的 benchmark 测试：

```bash
# Container 模块
cargo bench --bench array_bench
cargo bench --bench list_bench
cargo bench --bench queue_bench
cargo bench --bench ring_bench
cargo bench --bench map_bench
cargo bench --bench set_bench
cargo bench --bench pool_bench

# OS 模块
cargo bench --bench cache_bench
cargo bench --bench mutex_bench

# Encoding 模块
cargo bench --bench json_bench
cargo bench --bench hash_bench

# Util 模块
cargo bench --bench guid_bench
cargo bench --bench rand_bench
```

## 查看详细结果

Criterion 会生成详细的 HTML 报告，位于：
- `target/criterion/` 目录下

每个 benchmark 都有对应的 HTML 报告，包含：
- 性能统计信息
- 性能趋势图
- 样本分布图


---

## 测试结果 - 2026-01-06 18:46:11

> **注意**: 以下性能数据为单次运行结果，实际性能可能因系统负载、编译器优化等因素有所变化。

### Container 模块性能摘要

#### Array 容器
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| array_push | 1.30 µs | 添加 1000 个元素 |
| array_pop | 913 ns | 弹出元素 |
| array_get | 705 ns | 获取元素 |
| array_len | 278 ps | 获取长度（极快） |

#### List 容器
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| list_push_back | 17.18 µs | 尾部添加 1000 个元素 |
| list_push_front | 18.02 µs | 头部添加 1000 个元素 |
| list_pop_back | 17.48 µs | 尾部弹出 |
| list_pop_front | 17.24 µs | 头部弹出 |
| list_len | 280 ps | 获取长度（极快） |
| list_concurrent_push_back | 63.90 µs | 4 线程并发添加 |

#### Queue 容器
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| queue_push | 4.39 µs | 推入 1000 个元素 |
| queue_pop | 3.68 ns | 弹出单个元素（极快） |
| queue_concurrent_push | 36.71 µs | 4 线程并发推入 |
| queue_concurrent_pop | 33.95 µs | 4 线程并发弹出 |

#### Ring 容器
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| ring_push | 1.79 µs | 推入 1000 个元素 |
| ring_pop | 2.65 µs | 弹出元素 |
| ring_get | 18.09 µs | 获取元素（1000 次） |
| ring_len | 287 ps | 获取长度（极快） |

#### Map 容器
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| hashmap_insert | 59.83 µs | 插入 1000 个键值对 |
| hashmap_get | 35.12 µs | 获取 1000 次 |
| hashmap_remove | 103.14 µs | 删除 1000 个键值对 |
| hashmap_concurrent_insert | 69.43 µs | 4 线程并发插入 |
| orderedmap_insert | 42.21 µs | 插入 1000 个键值对 |
| orderedmap_get | 27.83 µs | 获取 1000 次 |

#### Set 容器
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| set_insert | 20.51 µs | 插入 1000 个元素 |
| set_contains | 4.78 µs | 检查 1000 次 |
| set_remove | 32.96 µs | 删除 1000 个元素 |
| set_len | 286 ps | 获取长度（极快） |

#### Pool 容器
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| pool_get | 21.82 ns | 从对象池获取对象 |

### OS 模块性能摘要

#### Cache 缓存
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| cache_insert | 232.03 µs | 插入 1000 个键值对（异步） |
| cache_get | 67.09 µs | 获取 1000 次（异步） |
| cache_remove | 493.76 µs | 删除 1000 个键值对（异步） |

#### Mutex 互斥锁
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| std_mutex_lock_unlock | 4.12 ns | 标准库 Mutex 加锁解锁 |
| parking_lot_mutex_lock_unlock | 1.81 ns | Parking Lot Mutex 加锁解锁（更快） |
| std_mutex_concurrent | 38.75 µs | 4 线程并发操作（1000 次） |
| parking_lot_mutex_concurrent | 35.06 µs | 4 线程并发操作（1000 次，更快） |

### Encoding 模块性能摘要

#### JSON 编码/解码
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| json_encode | 80.61 ns | 编码单个对象 |
| json_encode_pretty | 126.93 ns | 格式化编码单个对象 |
| json_decode | 170.79 ns | 解码单个对象 |
| json_parse | 359.69 ns | 解析 JSON 字符串 |

#### Hash 哈希
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| xxhash_small | 6.24 ns | 小数据（13 字节）XXHash |
| xxhash_large | 406.12 ns | 大数据（10KB）XXHash |
| hash_small | 5.27 ns | 小数据通用哈希 |
| hash_large | 2.67 µs | 大数据（10KB）通用哈希 |

### Util 模块性能摘要

#### GUID 生成
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| guid_new | 743.94 ns | 生成标准 UUID |
| guid_new_simple | 737.63 ns | 生成简化 UUID |

#### Random 随机数
| 操作 | 平均时间 | 说明 |
|------|---------|------|
| rand_int | 5.82 ns | 生成随机整数 |
| rand_float | 7.46 ns | 生成随机浮点数 |
| rand_string | 150.42 ns | 生成 32 字符随机字符串 |

---

**测试完成！** 所有 benchmark 测试已成功运行，详细结果请查看上方的性能摘要表格。
