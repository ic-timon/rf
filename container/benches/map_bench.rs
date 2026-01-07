//! # Map Benchmark Tests
//!
//! Benchmark tests for Map containers

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_container::{HashMap, OrderedMap};
use std::sync::Arc;
use std::thread;

fn bench_hashmap_insert(c: &mut Criterion) {
    c.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let map = HashMap::new();
            for i in 0..1000 {
                map.insert(black_box(i.to_string()), black_box(i));
            }
        });
    });
}

fn bench_hashmap_get(c: &mut Criterion) {
    c.bench_function("hashmap_get", |b| {
        let map = HashMap::new();
        for i in 0..1000 {
            map.insert(i.to_string(), i);
        }
        b.iter(|| {
            for i in 0..1000 {
                black_box(map.get(&(i % 1000).to_string()));
            }
        });
    });
}

fn bench_hashmap_remove(c: &mut Criterion) {
    c.bench_function("hashmap_remove", |b| {
        b.iter(|| {
            let map = HashMap::new();
            for i in 0..1000 {
                map.insert(i.to_string(), i);
            }
            for i in 0..1000 {
                black_box(map.remove(&i.to_string()));
            }
        });
    });
}

fn bench_hashmap_concurrent_insert(c: &mut Criterion) {
    c.bench_function("hashmap_concurrent_insert", |b| {
        b.iter(|| {
            let map = Arc::new(HashMap::new());
            let handles: Vec<_> = (0..4)
                .map(|i| {
                    let map = Arc::clone(&map);
                    thread::spawn(move || {
                        for j in 0..250 {
                            map.insert(black_box((i * 250 + j).to_string()), black_box(i * 250 + j));
                        }
                    })
                })
                .collect();
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
}

fn bench_orderedmap_insert(c: &mut Criterion) {
    c.bench_function("orderedmap_insert", |b| {
        b.iter(|| {
            let mut map = OrderedMap::new();
            for i in 0..1000 {
                map.insert(black_box(i.to_string()), black_box(i));
            }
        });
    });
}

fn bench_orderedmap_get(c: &mut Criterion) {
    c.bench_function("orderedmap_get", |b| {
        let mut map = OrderedMap::new();
        for i in 0..1000 {
            map.insert(i.to_string(), i);
        }
        b.iter(|| {
            for i in 0..1000 {
                black_box(map.get(&(i % 1000).to_string()));
            }
        });
    });
}

criterion_group!(
    benches,
    bench_hashmap_insert,
    bench_hashmap_get,
    bench_hashmap_remove,
    bench_hashmap_concurrent_insert,
    bench_orderedmap_insert,
    bench_orderedmap_get
);
criterion_main!(benches);

