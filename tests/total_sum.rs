use back_rs::problems::TotalSum;
use back_rs::solve::{IterSolve, Solution::Sat};
use back_rs::solvers::IterSolveNaive;

#[test]
fn total_sum_search_sat() {
    let asc = TotalSum::new(3, &[2, 0], 3);
    let solver = IterSolveNaive::new(&asc);

    let mut sats = solver.solution_iter().filter_map(|s| match s {
        Sat(sol) => Some(sol),
        _ => None,
    });

    assert_eq!(sats.next(), Some(vec![2, 2, 0]));
    assert_eq!(sats.next(), Some(vec![2, 0, 2]));
    assert_eq!(sats.next(), Some(vec![0, 2, 2]));
    assert_eq!(sats.next(), None);
}
