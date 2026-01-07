//! # Set Benchmark Tests
//!
//! Benchmark tests for Set container

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_container::Set;

fn bench_set_insert(c: &mut Criterion) {
    c.bench_function("set_insert", |b| {
        b.iter(|| {
            let mut set = Set::new();
            for i in 0..1000 {
                set.insert(black_box(i));
            }
        });
    });
}

fn bench_set_contains(c: &mut Criterion) {
    c.bench_function("set_contains", |b| {
        let mut set = Set::new();
        for i in 0..1000 {
            set.insert(i);
        }
        b.iter(|| {
            for i in 0..1000 {
                black_box(set.contains(&(i % 1000)));
            }
        });
    });
}

fn bench_set_remove(c: &mut Criterion) {
    c.bench_function("set_remove", |b| {
        b.iter(|| {
            let mut set = Set::new();
            for i in 0..1000 {
                set.insert(i);
            }
            for i in 0..1000 {
                black_box(set.remove(&i));
            }
        });
    });
}

fn bench_set_len(c: &mut Criterion) {
    c.bench_function("set_len", |b| {
        let set = {
            let mut set = Set::new();
            for i in 0..1000 {
                set.insert(i);
            }
            set
        };
        b.iter(|| {
            black_box(set.len());
        });
    });
}

criterion_group!(benches, bench_set_insert, bench_set_contains, bench_set_remove, bench_set_len);
criterion_main!(benches);

