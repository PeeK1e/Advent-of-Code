use criterion::{criterion_group, criterion_main, Criterion};
use solution::solve::{solve_t1, solve_t2};

fn bench_part_1(c: &mut Criterion) {
    let content = std::fs::read_to_string("./input").unwrap();
    c.bench_function("part_1", |b| b.iter(|| solve_t1(&content)));
    print!("{}", solve_t1(&content).unwrap());
}

fn bench_part_2(c: &mut Criterion) {
    let content = std::fs::read_to_string("./input").unwrap();
    c.bench_function("part_2", |b| b.iter(|| solve_t2(&content)));
}

criterion_group!(benches, bench_part_1, bench_part_2);
criterion_main!(benches);
