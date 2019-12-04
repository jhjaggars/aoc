#[macro_use]
extern crate criterion;
use std::fs;
use three::{counter, trace};

use criterion::Criterion;
// use criterion::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("part1.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let w1 = lines[0].to_string();
    let wire1 = trace(w1.clone());
    // c.bench_function("shortest", move |b| b.iter(|| shortest(&wire1, &wire2)));
    c.bench_function("trace", |b| b.iter(|| trace(w1.clone())));
    c.bench_function("counter", |b| b.iter(|| counter(&wire1.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
