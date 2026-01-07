//! # Random Benchmark Tests
//!
//! Benchmark tests for random number generation

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_util::rand;

fn bench_rand_int(c: &mut Criterion) {
    c.bench_function("rand_int", |b| {
        b.iter(|| {
            black_box(rand::int_range(0, 1000));
        });
    });
}

fn bench_rand_float(c: &mut Criterion) {
    c.bench_function("rand_float", |b| {
        b.iter(|| {
            black_box(rand::float());
        });
    });
}

fn bench_rand_string(c: &mut Criterion) {
    c.bench_function("rand_string", |b| {
        b.iter(|| {
            black_box(rf_util::rand_string(32));
        });
    });
}

criterion_group!(benches, bench_rand_int, bench_rand_float, bench_rand_string);
criterion_main!(benches);

