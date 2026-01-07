//! # Hash Benchmark Tests
//!
//! Benchmark tests for hash functions

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_encoding::{hash, xxhash};

fn bench_xxhash_small(c: &mut Criterion) {
    let data = b"Hello, World!";
    c.bench_function("xxhash_small", |b| {
        b.iter(|| {
            black_box(xxhash(black_box(data)));
        });
    });
}

fn bench_xxhash_large(c: &mut Criterion) {
    let data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    c.bench_function("xxhash_large", |b| {
        b.iter(|| {
            black_box(xxhash(black_box(&data)));
        });
    });
}

fn bench_hash_small(c: &mut Criterion) {
    let data = "Hello, World!";
    c.bench_function("hash_small", |b| {
        b.iter(|| {
            black_box(hash(black_box(&data)));
        });
    });
}

fn bench_hash_large(c: &mut Criterion) {
    let data: String = (0..10000).map(|i| (i % 256) as u8 as char).collect();
    c.bench_function("hash_large", |b| {
        b.iter(|| {
            black_box(hash(black_box(&data)));
        });
    });
}

criterion_group!(
    benches,
    bench_xxhash_small,
    bench_xxhash_large,
    bench_hash_small,
    bench_hash_large
);
criterion_main!(benches);

