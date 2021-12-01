use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2021_day1::*;

fn criterion_benchmark(c: &mut Criterion) {
    let numbers = load_input("../input");

    c.bench_function("part1", |b| {
        b.iter(|| count_increases_part1(black_box(&numbers)))
    });

    c.bench_function("part2", |b| {
        b.iter(|| count_increases_part2(black_box(&numbers)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
