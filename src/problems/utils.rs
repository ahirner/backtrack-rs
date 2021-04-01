//! Helper functions for unit testing problems

#[cfg(test)]
use crate::problem::{Check, Scope};
use std::fmt::Debug;

#[cfg(test)]
fn sat<'a, T: 'a + Clone, P: Scope<'a, T> + Check<T>>(problem: &P, solution: Vec<T>) -> bool {
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
#[cfg(test)]
pub(crate) fn sat_safe<'a, T: 'a + Eq + Clone + Debug, P: Scope<'a, T> + Check<T>>(
    problem: &'a P,
    solution: impl IntoIterator<Item = T>,
) -> bool {
    let all: Vec<_> = solution.into_iter().collect();
    assert!(all.len() <= problem.size(), "number of solutions cannot be beyond n");
    let domain: Vec<_> = problem.iter_values().collect();
    for candidate in all.iter() {
        let in_ = domain.contains(candidate);
        assert!(in_, format!("candidate not part of domain {:?}", domain));
    }
    sat(problem, all)
}
