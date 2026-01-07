//! # Queue Benchmark Tests
//!
//! Benchmark tests for Queue container

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_container::Queue;
use std::thread;

fn bench_queue_push(c: &mut Criterion) {
    c.bench_function("queue_push", |b| {
        b.iter(|| {
            let queue = Queue::new();
            for i in 0..1000 {
                queue.push(black_box(i));
            }
        });
    });
}

fn bench_queue_pop(c: &mut Criterion) {
    c.bench_function("queue_pop", |b| {
        let queue = Queue::new();
        for i in 0..1000 {
            queue.push(i);
        }
        b.iter(|| {
            let queue = queue.clone();
            while let Some(val) = queue.pop() {
                black_box(val);
            }
        });
    });
}

fn bench_queue_concurrent_push(c: &mut Criterion) {
    c.bench_function("queue_concurrent_push", |b| {
        b.iter(|| {
            let queue = Queue::new();
            let handles: Vec<_> = (0..4)
                .map(|i| {
                    let queue = queue.clone();
                    thread::spawn(move || {
                        for j in 0..250 {
                            queue.push(black_box(i * 250 + j));
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

fn bench_queue_concurrent_pop(c: &mut Criterion) {
    c.bench_function("queue_concurrent_pop", |b| {
        let queue = Queue::new();
        for i in 0..1000 {
            queue.push(i);
        }
        b.iter(|| {
            let queue = queue.clone();
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let queue = queue.clone();
                    thread::spawn(move || {
                        for _ in 0..250 {
                            black_box(queue.pop());
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

criterion_group!(
    benches,
    bench_queue_push,
    bench_queue_pop,
    bench_queue_concurrent_push,
    bench_queue_concurrent_pop
);
criterion_main!(benches);

