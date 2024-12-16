use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use solution::solve::{solve_t1, solve_t2};

fn bench_part_1(c: &mut Criterion) {
    let content = std::fs::read_to_string("./input").unwrap();
    let mut group = c.benchmark_group("solve");
    group.measurement_time(Duration::new(30, 0));
    group.bench_function("part_1", |b| b.iter(|| solve_t1(&content)));
    group.finish();
}

fn bench_part_2(c: &mut Criterion) {
    let content = std::fs::read_to_string("./input").unwrap();
    let mut group = c.benchmark_group("solve");
    group.measurement_time(Duration::new(30, 0));
    group.bench_function("part_2", |b| b.iter(|| solve_t2(&content)));
    group.finish();
}

criterion_group!(benches, bench_part_1, bench_part_2);
criterion_main!(benches);
