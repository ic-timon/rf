//! # JSON Benchmark Tests
//!
//! Benchmark tests for JSON encoding/decoding

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rf_encoding::{json_decode, json_encode, json_encode_pretty, json_parse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct TestStruct {
    name: String,
    age: u32,
    email: String,
    active: bool,
    tags: Vec<String>,
}

fn bench_json_encode(c: &mut Criterion) {
    let data = TestStruct {
        name: "John Doe".to_string(),
        age: 30,
        email: "john@example.com".to_string(),
        active: true,
        tags: vec!["rust".to_string(), "benchmark".to_string()],
    };
    c.bench_function("json_encode", |b| {
        b.iter(|| {
            black_box(json_encode(black_box(&data)).unwrap());
        });
    });
}

fn bench_json_encode_pretty(c: &mut Criterion) {
    let data = TestStruct {
        name: "John Doe".to_string(),
        age: 30,
        email: "john@example.com".to_string(),
        active: true,
        tags: vec!["rust".to_string(), "benchmark".to_string()],
    };
    c.bench_function("json_encode_pretty", |b| {
        b.iter(|| {
            black_box(json_encode_pretty(black_box(&data)).unwrap());
        });
    });
}

fn bench_json_decode(c: &mut Criterion) {
    let json_str = r#"{"name":"John Doe","age":30,"email":"john@example.com","active":true,"tags":["rust","benchmark"]}"#;
    c.bench_function("json_decode", |b| {
        b.iter(|| {
            let _: TestStruct = black_box(json_decode(black_box(json_str)).unwrap());
        });
    });
}

fn bench_json_parse(c: &mut Criterion) {
    let json_str = r#"{"name":"John Doe","age":30,"email":"john@example.com","active":true,"tags":["rust","benchmark"]}"#;
    c.bench_function("json_parse", |b| {
        b.iter(|| {
            black_box(json_parse(black_box(json_str)).unwrap());
        });
    });
}

criterion_group!(
    benches,
    bench_json_encode,
    bench_json_encode_pretty,
    bench_json_decode,
    bench_json_parse
);
criterion_main!(benches);

