use crate::common::assert_unsat_unique;
use backtrack::problems::TotalSum;
use backtrack::solve::IterSolveExt;
use backtrack::solvers::IterSolveCached;

#[test]
fn total_sum_search_sat() {
    let sum = TotalSum::new(3, &[2, 0], 3);
    let solver = IterSolveCached::new(&sum);

    let mut sats = solver.sat_iter();

    assert_eq!(sats.next(), Some(vec![2, 2, 0]));
    assert_eq!(sats.next(), Some(vec![2, 0, 2]));
    assert_eq!(sats.next(), Some(vec![0, 2, 2]));
    assert_eq!(sats.next(), None);
}

#[test]
fn total_sum_unsat_unique() {
    let sum = TotalSum::new(3, &[2, 0], 3);
    let unsats = IterSolveCached::new(&sum).unsat_iter();
    assert_unsat_unique(unsats);
}

#[test]
fn total_sum_search_sat_cached() {
    let sum = TotalSum::new(3, &[2, 0], 3);
    let solver = IterSolveCached::new(&sum);

    let mut sats = solver.sat_iter();

    assert_eq!(sats.next(), Some(vec![2, 2, 0]));
    assert_eq!(sats.next(), Some(vec![2, 0, 2]));
    assert_eq!(sats.next(), Some(vec![0, 2, 2]));
    assert_eq!(sats.next(), None);
}

#[test]
fn total_sum_unsat_unique_cached() {
    let sum = TotalSum::new(3, &[2, 0], 3);
    let unsats = IterSolveCached::new(&sum).unsat_iter();
    assert_unsat_unique(unsats);
}
