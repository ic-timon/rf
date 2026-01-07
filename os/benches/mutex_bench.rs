//! # Mutex Benchmark Tests
//!
//! Benchmark tests for Mutex

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_os::mutex::Mutex;
use std::sync::{Arc, Mutex as StdMutex};
use std::thread;

fn bench_std_mutex_lock_unlock(c: &mut Criterion) {
    c.bench_function("std_mutex_lock_unlock", |b| {
        let mutex = StdMutex::new(0);
        b.iter(|| {
            let _guard = mutex.lock().unwrap();
            black_box(*_guard);
        });
    });
}

fn bench_parking_lot_mutex_lock_unlock(c: &mut Criterion) {
    c.bench_function("parking_lot_mutex_lock_unlock", |b| {
        let mutex = Mutex::new(0);
        b.iter(|| {
            let _guard = mutex.lock();
            black_box(*_guard);
        });
    });
}

fn bench_std_mutex_concurrent(c: &mut Criterion) {
    c.bench_function("std_mutex_concurrent", |b| {
        b.iter(|| {
            let mutex = Arc::new(StdMutex::new(0));
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let mutex = Arc::clone(&mutex);
                    thread::spawn(move || {
                        for _ in 0..250 {
                            let mut guard = mutex.lock().unwrap();
                            *guard += 1;
                            black_box(*guard);
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

fn bench_parking_lot_mutex_concurrent(c: &mut Criterion) {
    c.bench_function("parking_lot_mutex_concurrent", |b| {
        b.iter(|| {
            let mutex = Arc::new(Mutex::new(0));
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let mutex = Arc::clone(&mutex);
                    thread::spawn(move || {
                        for _ in 0..250 {
                            let mut guard = mutex.lock();
                            *guard += 1;
                            black_box(*guard);
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
    bench_std_mutex_lock_unlock,
    bench_parking_lot_mutex_lock_unlock,
    bench_std_mutex_concurrent,
    bench_parking_lot_mutex_concurrent
);
criterion_main!(benches);

