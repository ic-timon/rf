//! # Pool Benchmark Tests
//!
//! Benchmark tests for Pool container
//!
//! 注意：ObjectPool 是一个占位符实现，此 benchmark 仅用于测试基本功能。

use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[allow(deprecated)]
use rf_container::ObjectPool;
use tokio::runtime::Runtime;

fn bench_pool_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("pool_get", |b| {
        #[allow(deprecated)]
        let pool = ObjectPool::<Vec<i32>>::new(100);
        b.iter(|| {
            rt.block_on(async {
                let _obj = black_box(pool.get().await);
            });
        });
    });
}

criterion_group!(benches, bench_pool_get);
criterion_main!(benches);

