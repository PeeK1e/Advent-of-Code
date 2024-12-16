use criterion::{criterion_group, criterion_main, Criterion};
use solution::solve_1::solve1;
use solution::solve_2::solve2;

fn bench_part_1(c: &mut Criterion) {
    let content = std::fs::read_to_string("./input").unwrap();
    c.bench_function("part_1", |b| b.iter(|| solve1(&content)));
}

fn bench_part_2(c: &mut Criterion) {
    let content = std::fs::read_to_string("./input").unwrap();
    c.bench_function("part_2", |b| b.iter(|| solve2(&content)));
}

criterion_group!(benches, bench_part_1, bench_part_2);
criterion_main!(benches);
