//! # Cache Benchmark Tests
//!
//! Benchmark tests for Cache container

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_os::cache::CacheContainer;
use tokio::runtime::Runtime;

fn bench_cache_insert(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("cache_insert", |b| {
        b.iter(|| {
            rt.block_on(async {
                let cache = CacheContainer::new(10000);
                for i in 0..1000 {
                    cache.insert(black_box(i), black_box(i * 2)).await;
                }
            });
        });
    });
}

fn bench_cache_get(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let cache = rt.block_on(async {
        let cache = CacheContainer::new(10000);
        for i in 0..1000 {
            cache.insert(i, i * 2).await;
        }
        cache
    });
    c.bench_function("cache_get", |b| {
        b.iter(|| {
            rt.block_on(async {
                for i in 0..1000 {
                    black_box(cache.get(&(i % 1000)).await);
                }
            });
        });
    });
}

fn bench_cache_remove(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    c.bench_function("cache_remove", |b| {
        b.iter(|| {
            rt.block_on(async {
                let cache = CacheContainer::new(10000);
                for i in 0..1000 {
                    cache.insert(i, i * 2).await;
                }
                for i in 0..1000 {
                    cache.remove(&i).await;
                }
            });
        });
    });
}

criterion_group!(benches, bench_cache_insert, bench_cache_get, bench_cache_remove);
criterion_main!(benches);

