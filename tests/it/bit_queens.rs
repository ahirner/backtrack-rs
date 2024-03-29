use backtrack::problems::{BitQueens, NQueens};
use backtrack::solvers::{IterSolveCached, IterSolveNaive};
use std::cmp::min;

#[test]
fn tiny_bit_at_least() {
    assert!(BitQueens::max_n() >= 8, "you should buy a larger computer")
}

#[test]
fn equivalent_n_queens() {
    for n in 0..=min(9, BitQueens::max_n()) {
        let n_queens = NQueens::new(n);
        let n_solver = IterSolveNaive::new(&n_queens);

        let b_queens = BitQueens::new(n);
        let b_solver = IterSolveCached::new(&b_queens);

        let both = n_solver.into_iter().zip(b_solver.into_iter());

        for (i, (n_solution, b_solution)) in both.enumerate() {
            assert_eq!(n_solution, b_solution, "No match at n: {}, i: {}", n, i);
        }
    }
}
