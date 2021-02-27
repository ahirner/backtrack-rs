//! Helper functions for unit testing problems

#[cfg(test)]
use crate::problem::{Check, Scope};

#[cfg(test)]
fn sat<P: Scope + Check>(problem: &P, solution: impl IntoIterator<Item = usize>) -> bool {
    // todo: use ArrayVector or similar
    let mut all = Vec::with_capacity(problem.size());
    for x_l in solution.into_iter() {
        if !problem.extends_sat(&all, x_l) {
            return false;
        }
        all.push(x_l);
    }
    true
}

/// collect all solutions and assert their size is below n and within
/// domain, then return `sat`
#[cfg(test)]
pub(crate) fn sat_safe<P: Scope + Check>(
    problem: &P,
    solution: impl IntoIterator<Item = usize>,
) -> bool {
    let all: Vec<_> = solution.into_iter().collect();
    assert!(all.len() <= problem.size(), "number of solutions cannot be beyond n");
    let domain = problem.domain();
    for solution in all.iter() {
        let in_ = domain.contains(&solution);
        assert!(in_, "solution is not part of domain")
    }
    sat(problem, all)
}
