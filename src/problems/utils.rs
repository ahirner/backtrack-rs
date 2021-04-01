//! Helper functions for unit testing problems
#![cfg(test)]

use crate::problem::{Check, Scope};
use std::fmt::Debug;

fn sat<'a, T, P>(problem: &P, solution: Vec<T>) -> bool
where
    T: 'a + Clone,
    P: Scope<'a, T> + Check<T>,
{
    let mut all = Vec::with_capacity(problem.size());
    for x_l in solution.into_iter() {
        if !problem.extends_sat(&all, &x_l) {
            return false;
        }
        all.push(x_l);
    }
    true
}

/// collect all solutions and assert their size is below n and within
/// domain, then return `sat`
pub(crate) fn sat_safe<'a, T, P, S>(problem: &'a P, solution: S) -> bool
where
    T: 'a + Clone + Eq + Debug,
    P: Scope<'a, T> + Check<T>,
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
