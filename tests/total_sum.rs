use backtrack_rs::problems::TotalSum;
use backtrack_rs::solve::IterSolve;
use backtrack_rs::solvers::IterSolveNaive;

#[test]
fn total_sum_search_sat() {
    let asc = TotalSum::new(3, &[2, 0], 3);
    let solver = IterSolveNaive::new(&asc);

    let mut sats = solver.sat_iter();

    assert_eq!(sats.next(), Some(vec![2, 2, 0]));
    assert_eq!(sats.next(), Some(vec![2, 0, 2]));
    assert_eq!(sats.next(), Some(vec![0, 2, 2]));
    assert_eq!(sats.next(), None);
}
