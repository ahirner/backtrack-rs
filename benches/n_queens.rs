use criterion::{black_box, criterion_group, criterion_main, Criterion};

use backtrack::problems::NQueens;
use backtrack::solvers::IterSolveNaive;

fn bench_n_queens(n: usize) -> usize {
    let n_queens = NQueens::new(n);
    let solver = IterSolveNaive::new(&n_queens);
    solver.into_iter().count()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("n-queens 5", |b| b.iter(|| bench_n_queens(black_box(5))));
    c.bench_function("n-queens 9", |b| b.iter(|| bench_n_queens(black_box(9))));

    let mut g_small = c.benchmark_group("smaller-sample-size");
    g_small.sample_size(50);
    g_small
        .bench_function("n-queens 10", |b| b.iter(|| bench_n_queens(black_box(10))))
        .sample_size(50);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
