//! # Ring Benchmark Tests
//!
//! Benchmark tests for Ring container

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_container::Ring;

fn bench_ring_push(c: &mut Criterion) {
    c.bench_function("ring_push", |b| {
        b.iter(|| {
            let mut ring = Ring::new(1000);
            for i in 0..1000 {
                let _ = ring.push(black_box(i));
            }
        });
    });
}

fn bench_ring_pop(c: &mut Criterion) {
    c.bench_function("ring_pop", |b| {
        let mut ring = Ring::new(1000);
        for i in 0..1000 {
            let _ = ring.push(i);
        }
        b.iter(|| {
            let mut ring = {
                let mut new_ring = Ring::new(1000);
                for i in 0..1000 {
                    let _ = new_ring.push(i);
                }
                new_ring
            };
            while ring.pop().is_some() {
                black_box(ring.pop());
            }
        });
    });
}

fn bench_ring_get(c: &mut Criterion) {
    c.bench_function("ring_get", |b| {
        let mut ring = Ring::new(1000);
        for i in 0..1000 {
            let _ = ring.push(i);
        }
        b.iter(|| {
            let len = ring.len();
            for i in 0..1000 {
                black_box(ring.slice(i % len, (i % len + 1).min(len)));
            }
        });
    });
}

fn bench_ring_len(c: &mut Criterion) {
    c.bench_function("ring_len", |b| {
        let ring = {
            let mut ring = Ring::new(1000);
            for i in 0..1000 {
                let _ = ring.push(i);
            }
            ring
        };
        b.iter(|| {
            black_box(ring.len());
        });
    });
}

criterion_group!(benches, bench_ring_push, bench_ring_pop, bench_ring_get, bench_ring_len);
criterion_main!(benches);

