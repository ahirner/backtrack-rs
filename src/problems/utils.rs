//! Helper functions for unit testing problems
#![cfg(test)]

use crate::problem::{CheckInc, Scope};
use std::fmt::Debug;

fn sat<'a, T, P>(problem: &P, solution: Vec<T>) -> bool
where
    T: 'a + Clone,
    P: Scope<'a, T> + CheckInc<T>,
    P::Accumulator: Clone,
{
    let mut acc0 = None;
    for (i, x_l) in solution.iter().enumerate() {
        let acc = problem.fold_acc(acc0.clone(), &x_l, i);
        if !problem.accu_sat(&acc, &x_l, i) {
            return false;
        }
        acc0 = Some(acc);
    }
    true
}

/// collect all solutions and assert their size is below n and within
/// domain, then return `sat`
pub(crate) fn sat_safe<'a, T, P, S>(problem: &'a P, solution: S) -> bool
where
    T: 'a + Clone + Eq + Debug,
    P: Scope<'a, T> + CheckInc<T>,
    P::Accumulator: Clone,
    S: IntoIterator<Item = T>,
    S::Item: std::fmt::Debug,
{
    let all: Vec<T> = solution.into_iter().collect();
    assert!(all.len() <= problem.size(), "number of solutions cannot be beyond n");
    let domain: Vec<_> = problem.iter_values().collect();
    for candidate in all.iter() {
        let in_ = domain.contains(candidate);
        assert!(in_, "candidate '{:?}' not part of domain {:?}", candidate, domain);
    }
    sat(problem, all)
}
