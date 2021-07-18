use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::hash_map::DefaultHasher;

use backtrack::problems::{BitQueens, NQueens, TotalSum};
use backtrack::solvers::{IterSolveCached, IterSolveNaive};
use std::hash::{Hash, Hasher};

fn bench_n_queens(n: usize) -> usize {
    let n_queens = NQueens::new(n);
    let solver = IterSolveNaive::new(&n_queens);
    solver.into_iter().count()
}

fn bench_bit_queens(n: usize) -> usize {
    let n_queens = BitQueens::new(n);
    let solver = IterSolveCached::new(&n_queens);
    solver.into_iter().count()
}

fn bench_total_sum(n: usize) -> usize {
    let mut domain = Vec::default();
    let mut hasher = DefaultHasher::new();
    for i in 0..3 {
        i.hash(&mut hasher);
        let v = hasher.finish() as usize % (n / 2);
        domain.push(v);
    }
    let total_sum = TotalSum::new(n, &domain, n * 4);
    let solver = IterSolveCached::new(&total_sum);
    solver.into_iter().count()
}

fn criterion_bench_n_queens(c: &mut Criterion) {
    c.bench_function("n-queens 5", |b| b.iter(|| bench_n_queens(black_box(5))));
    c.bench_function("n-queens 9", |b| b.iter(|| bench_n_queens(black_box(9))));

    let mut g_small = c.benchmark_group("smaller-sample-size");
    g_small.sample_size(10);
    g_small.bench_function("n-queens 10", |b| b.iter(|| bench_n_queens(black_box(10))));
    g_small.bench_function("n-queens 11", |b| b.iter(|| bench_n_queens(black_box(11))));
}

fn criterion_bench_bit_queens(c: &mut Criterion) {
    c.bench_function("bit-queens 9", |b| b.iter(|| bench_bit_queens(black_box(9))));

    let mut g_small = c.benchmark_group("smaller-sample-size");
    g_small.sample_size(10);
    g_small.bench_function("bit-queens 10", |b| b.iter(|| bench_bit_queens(black_box(10))));
    g_small.bench_function("bit-queens 11", |b| b.iter(|| bench_bit_queens(black_box(11))));
}

fn criterion_bench_total_sum(c: &mut Criterion) {
    c.bench_function("total-sum 3^3", |b| b.iter(|| bench_total_sum(black_box(3))));
    let mut g_small = c.benchmark_group("smaller-sample-size");
    g_small.sample_size(25);
    g_small.bench_function("total-sum 3^42", |b| b.iter(|| bench_total_sum(black_box(42))));
}

criterion_group!(
    benches,
    criterion_bench_n_queens,
    criterion_bench_bit_queens,
    criterion_bench_total_sum
);
criterion_main!(benches);
