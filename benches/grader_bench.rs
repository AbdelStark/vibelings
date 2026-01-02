//! Benchmarks for the grader module.
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde_json::json;

/// Benchmark JSON schema validation performance.
fn bench_schema_validation(c: &mut Criterion) {
    // Create a sample schema
    let schema = json!({
        "type": "object",
        "properties": {
            "name": { "type": "string", "minLength": 1 },
            "age": { "type": "integer", "minimum": 0 },
            "email": { "type": "string", "format": "email" },
            "tags": {
                "type": "array",
                "items": { "type": "string" }
            }
        },
        "required": ["name", "age"]
    });

    let compiled = jsonschema::validator_for(&schema).expect("Invalid schema");

    // Valid instance
    let valid_instance = json!({
        "name": "Test User",
        "age": 25,
        "email": "test@example.com",
        "tags": ["rust", "ai", "agents"]
    });

    // Invalid instance (missing required field)
    let invalid_instance = json!({
        "name": "Test User"
    });

    c.bench_function("schema_validation_valid", |b| {
        b.iter(|| compiled.is_valid(black_box(&valid_instance)))
    });

    c.bench_function("schema_validation_invalid", |b| {
        b.iter(|| compiled.is_valid(black_box(&invalid_instance)))
    });

    c.bench_function("schema_compile", |b| {
        b.iter(|| jsonschema::validator_for(black_box(&schema)))
    });
}

/// Benchmark JSON parsing performance.
fn bench_json_parsing(c: &mut Criterion) {
    let json_str = r#"{
        "name": "Test User",
        "age": 25,
        "email": "test@example.com",
        "tags": ["rust", "ai", "agents"],
        "metadata": {
            "created": "2024-01-01",
            "updated": "2024-01-02",
            "version": 1
        }
    }"#;

    c.bench_function("json_parse_small", |b| {
        b.iter(|| serde_json::from_str::<serde_json::Value>(black_box(json_str)))
    });

    // Larger JSON for stress testing
    let large_json = serde_json::to_string(&json!({
        "items": (0..100).map(|i| {
            json!({
                "id": i,
                "name": format!("Item {}", i),
                "value": i * 10,
                "tags": ["a", "b", "c"]
            })
        }).collect::<Vec<_>>()
    }))
    .unwrap();

    c.bench_function("json_parse_large", |b| {
        b.iter(|| serde_json::from_str::<serde_json::Value>(black_box(&large_json)))
    });
}

/// Benchmark complex nested schema validation.
fn bench_nested_schema(c: &mut Criterion) {
    let schema = json!({
        "type": "object",
        "properties": {
            "team": {
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "members": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "role": { "type": "string", "enum": ["admin", "member", "guest"] },
                                "email": { "type": "string" }
                            },
                            "required": ["name", "role"]
                        },
                        "minItems": 1
                    }
                },
                "required": ["name", "members"]
            }
        },
        "required": ["team"]
    });

    let compiled = jsonschema::validator_for(&schema).expect("Invalid schema");

    let valid_nested = json!({
        "team": {
            "name": "Engineering",
            "members": [
                { "name": "Alice", "role": "admin", "email": "alice@test.com" },
                { "name": "Bob", "role": "member", "email": "bob@test.com" },
                { "name": "Charlie", "role": "guest" }
            ]
        }
    });

    c.bench_function("nested_schema_validation", |b| {
        b.iter(|| compiled.is_valid(black_box(&valid_nested)))
    });
}

criterion_group!(
    benches,
    bench_schema_validation,
    bench_json_parsing,
    bench_nested_schema
);
criterion_main!(benches);
