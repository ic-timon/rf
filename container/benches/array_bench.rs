//! # Array Benchmark Tests
//!
//! Benchmark tests for Array container

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_container::Array;

fn bench_array_push(c: &mut Criterion) {
    c.bench_function("array_push", |b| {
        b.iter(|| {
            let mut arr = Array::new();
            for i in 0..1000 {
                arr.push(black_box(i));
            }
        });
    });
}

fn bench_array_pop(c: &mut Criterion) {
    c.bench_function("array_pop", |b| {
        let mut arr = Array::new();
        for i in 0..1000 {
            arr.push(i);
        }
        b.iter(|| {
            let mut arr = arr.clone();
            while arr.pop().is_some() {
                black_box(arr.pop());
            }
        });
    });
}

fn bench_array_get(c: &mut Criterion) {
    c.bench_function("array_get", |b| {
        let mut arr = Array::new();
        for i in 0..1000 {
            arr.push(i);
        }
        b.iter(|| {
            for i in 0..1000 {
                black_box(arr.get(i % arr.len()));
            }
        });
    });
}

fn bench_array_len(c: &mut Criterion) {
    c.bench_function("array_len", |b| {
        let arr = {
            let mut arr = Array::new();
            for i in 0..1000 {
                arr.push(i);
            }
            arr
        };
        b.iter(|| {
            black_box(arr.len());
        });
    });
}

criterion_group!(benches, bench_array_push, bench_array_pop, bench_array_get, bench_array_len);
criterion_main!(benches);

