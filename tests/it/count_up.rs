use crate::common::assert_unsat_unique;
use backtrack::problems::CountUp;
use backtrack::solve::IterSolveExt;
use backtrack::solvers::IterSolveNaive;

#[test]
fn count_up_search_sat() {
    let asc = CountUp::new(3, 0..4);
    let solver = IterSolveNaive::new(&asc);

    let mut sats = solver.sat_iter();

    assert_eq!(sats.next(), Some(vec![0, 1, 2]));
    assert_eq!(sats.next(), Some(vec![0, 1, 3]));
    assert_eq!(sats.next(), Some(vec![0, 2, 3]));
    assert_eq!(sats.next(), Some(vec![1, 2, 3]));
    assert_eq!(sats.next(), None);
}

#[test]
fn total_sum_unsat_unique() {
    let asc = CountUp::new(3, 0..4);
    let unsats = IterSolveNaive::new(&asc).unsat_iter();
    assert_unsat_unique(unsats);
}

#[test]
fn count_up_search_nosat() {
    let asc = CountUp::new(4, 0..3);
    let solver = IterSolveNaive::new(&asc);

    assert_eq!(solver.solution_iter().count(), 17);
}
