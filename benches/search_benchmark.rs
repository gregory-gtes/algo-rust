use criterion::{criterion_group, criterion_main, Criterion};
use algo_rust::algorithms::searching::binary_search;
use std::hint::black_box;

fn bench_binary_search(c: &mut Criterion) {
    println!("Starting benchmark");
    let arr: Vec<i32> = (0..10000).collect();
    c.bench_function("binary_search", |b| {
        b.iter(|| binary_search(black_box(&arr), black_box(&9876)));
    });
}

criterion_group!(benches, bench_binary_search);
criterion_main!(benches);