use backtrack::problems::NQueens;
use backtrack::solve::{CandidateSolution, IterSolveExt};
use backtrack::solvers::IterSolveNaive;

#[test]
fn four_queens_search_sat() {
    let n_queens = NQueens::new(4);
    let solver = IterSolveNaive::new(&n_queens);

    let mut sats = solver.sat_iter();

    assert_eq!(sats.next(), Some(vec![1, 3, 0, 2]));
    assert_eq!(sats.next(), Some(vec![2, 0, 3, 1]));
    assert_eq!(sats.next(), None);
}

#[test]
fn four_queens_search_nodes() {
    let n_queens = NQueens::new(4);
    let solver = IterSolveNaive::new(&n_queens);

    let nodes_not_unsat = solver
        .into_iter()
        .filter_map(|sol| match sol {
            CandidateSolution::Unsat(_) => None,
            _ => Some(()),
        })
        .count();
    assert_eq!(nodes_not_unsat, 16);
}
