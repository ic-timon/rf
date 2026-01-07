//! # List Benchmark Tests
//!
//! Benchmark tests for List container

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_container::List;
use std::thread;

fn bench_list_push_back(c: &mut Criterion) {
    c.bench_function("list_push_back", |b| {
        b.iter(|| {
            let mut list = List::new();
            for i in 0..1000 {
                list.push_back(black_box(i));
            }
        });
    });
}

fn bench_list_push_front(c: &mut Criterion) {
    c.bench_function("list_push_front", |b| {
        b.iter(|| {
            let mut list = List::new();
            for i in 0..1000 {
                list.push_front(black_box(i));
            }
        });
    });
}

fn bench_list_pop_back(c: &mut Criterion) {
    c.bench_function("list_pop_back", |b| {
        let mut list = List::new();
        for i in 0..1000 {
            list.push_back(i);
        }
        b.iter(|| {
            let mut list = list.clone();
            while list.pop_back().is_some() {
                black_box(list.pop_back());
            }
        });
    });
}

fn bench_list_pop_front(c: &mut Criterion) {
    c.bench_function("list_pop_front", |b| {
        let mut list = List::new();
        for i in 0..1000 {
            list.push_back(i);
        }
        b.iter(|| {
            let mut list = list.clone();
            while list.pop_front().is_some() {
                black_box(list.pop_front());
            }
        });
    });
}

fn bench_list_len(c: &mut Criterion) {
    c.bench_function("list_len", |b| {
        let list = {
            let mut list = List::new();
            for i in 0..1000 {
                list.push_back(i);
            }
            list
        };
        b.iter(|| {
            black_box(list.len());
        });
    });
}

fn bench_list_concurrent_push_back(c: &mut Criterion) {
    c.bench_function("list_concurrent_push_back", |b| {
        b.iter(|| {
            let mut list = List::new();
            let handles: Vec<_> = (0..4)
                .map(|i| {
                    thread::spawn(move || {
                        let mut local_list = List::new();
                        for j in 0..250 {
                            local_list.push_back(black_box(i * 250 + j));
                        }
                        local_list
                    })
                })
                .collect();
            for handle in handles {
                let mut local_list = handle.join().unwrap();
                // Merge lists (simulate concurrent access)
                while let Some(item) = local_list.pop_front() {
                    list.push_back(item);
                }
            }
        });
    });
}

criterion_group!(
    benches,
    bench_list_push_back,
    bench_list_push_front,
    bench_list_pop_back,
    bench_list_pop_front,
    bench_list_len,
    bench_list_concurrent_push_back
);
criterion_main!(benches);

