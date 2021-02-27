use backtrack_rs::problems::CountUp;
use backtrack_rs::solve::IterSolveExt;
use backtrack_rs::solvers::IterSolveNaive;

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
fn count_up_search_nosat() {
    let asc = CountUp::new(4, 0..3);
    let solver = IterSolveNaive::new(&asc);

    let unsats: Vec<_> = solver.solution_iter().collect();

    assert_eq!(unsats.len(), 17);
}
