//! # GUID Benchmark Tests
//!
//! Benchmark tests for GUID generation

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_util::guid;

fn bench_guid_new(c: &mut Criterion) {
    c.bench_function("guid_new", |b| {
        b.iter(|| {
            black_box(guid::new());
        });
    });
}

fn bench_guid_new_simple(c: &mut Criterion) {
    c.bench_function("guid_new_simple", |b| {
        b.iter(|| {
            black_box(guid::new_simple());
        });
    });
}

criterion_group!(benches, bench_guid_new, bench_guid_new_simple);
criterion_main!(benches);

