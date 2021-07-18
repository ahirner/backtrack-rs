//! Helper functions for unit testing problems
#![cfg(test)]

use crate::problem::{CheckInc, Scope};
use std::fmt::Debug;

fn sat<'a, T, P, S>(problem: &P, solution: S) -> bool
where
    T: 'a,
    P: Scope<'a, T> + CheckInc<T>,
    S: IntoIterator<Item = T>,
{
    let mut acc = None;
    for (i, x_l) in solution.into_iter().enumerate() {
        let acc1 = problem.fold_acc(acc, &x_l, i);
        if !problem.accu_sat(&acc1, &x_l, i) {
            return false;
        }
        acc = Some(acc1);
    }
    true
}

/// collect all solutions and assert their size is below n and within
/// domain, then return `sat`
pub(crate) fn sat_safe<'a, T, P, S>(problem: &'a P, solution: S) -> bool
where
    T: 'a + PartialEq + Debug,
    P: Scope<'a, T> + CheckInc<T>,
    S: IntoIterator<Item = T>,
    S::Item: std::fmt::Debug,
{
    let solution_all: Vec<T> = solution.into_iter().collect();
    assert!(solution_all.len() <= problem.size(), "number of solutions cannot be beyond n");
    let domain: Vec<_> = problem.iter_values().collect();
    for candidate in solution_all.iter() {
        assert!(
            domain.contains(candidate),
            "candidate '{:?}' not part of domain {:?}",
            candidate,
            domain
        );
    }
    sat(problem, solution_all)
}
