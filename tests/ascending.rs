use back_rs::problems::Ascending;
use back_rs::solve::{IterSolve, Solution::Sat};
use back_rs::solvers::IterSolveNaive;

#[test]
fn ascending_search_sat() {
    let asc = Ascending::new(3, 0..4);
    let solver = IterSolveNaive::new(&asc);

    let mut sats = solver.solution_iter().filter_map(|s| match s {
        Sat(sol) => Some(sol),
        _ => None,
    });

    assert_eq!(sats.next(), Some(vec![0, 1, 2]));
    assert_eq!(sats.next(), Some(vec![0, 1, 3]));
    assert_eq!(sats.next(), Some(vec![0, 2, 3]));
    assert_eq!(sats.next(), Some(vec![1, 2, 3]));
    assert_eq!(sats.next(), None);
}

#[test]
fn ascending_search_nosat() {
    let asc = Ascending::new(4, 0..3);
    let solver = IterSolveNaive::new(&asc);

    let unsats: Vec<_> = solver.solution_iter().collect();

    assert_eq!(unsats.len(), 17);
}
