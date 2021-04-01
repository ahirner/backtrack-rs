use backtrack::problem::{Check, Scope};
use backtrack::solve::IterSolveExt;
use backtrack::solvers::IterSolveNaive;

struct VarSizedProblem {
    size: usize,
    len: usize,
}

impl Scope<'_> for VarSizedProblem {
    fn size(&self) -> usize {
        self.size
    }

    fn value(&self, _index: usize) -> usize {
        if !self.is_empty() {
            0
        } else {
            unreachable!()
        }
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl Check for VarSizedProblem {
    fn extends_sat(&self, _solution: &[usize], _x: &usize) -> bool {
        true
    }
}

#[test]
fn zero_sized_problem() {
    let prob = VarSizedProblem { size: 0, len: 4 };
    let solver = IterSolveNaive::new(&prob);

    let mut sats = solver.sat_iter();
    assert_eq!(sats.next(), None);
}

#[test]
#[should_panic]
fn zero_len_problem() {
    let prob = VarSizedProblem { size: 4, len: 0 };
    let solver = IterSolveNaive::new(&prob);

    let mut sats = solver.sat_iter();
    sats.next();
}

#[test]
fn zero_sized_len_problem() {
    let prob = VarSizedProblem { size: 0, len: 0 };
    let solver = IterSolveNaive::new(&prob);

    let mut sats = solver.sat_iter();
    assert_eq!(sats.next(), None);
}
