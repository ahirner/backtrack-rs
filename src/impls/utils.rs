//! Helper functions for unit testing problems

#[cfg(test)]
use crate::problem::Problem;

#[cfg(test)]
fn sat<P: Problem>(problem: &P, solution: impl IntoIterator<Item = usize>) -> bool {
    // todo: use ArrayVector or similar
    let mut all = Vec::with_capacity(problem.get_n());
    for x_l in solution.into_iter() {
        if !problem.inc_sat(&all, x_l) {
            return false;
        }
        all.push(x_l);
    }
    true
}

/// collect all solutions and assert their size is below n and within
/// domain, then return `sat`
#[cfg(test)]
pub(crate) fn sat_safe<P: Problem>(problem: &P, solution: impl IntoIterator<Item = usize>) -> bool {
    let all: Vec<_> = solution.into_iter().collect();
    assert!(all.len() <= problem.get_n(), "number of solutions cannot be beyond n");
    let domain = problem.get_domain();
    for solution in all.iter() {
        let in_ = domain.contains(&solution);
        assert!(in_, "solution is not part of domain")
    }
    sat(problem, all)
}
