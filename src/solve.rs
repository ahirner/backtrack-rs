use std::iter::FilterMap;

#[derive(Debug)]
pub enum CandidateSolution {
    Incomplete,
    Sat(Vec<usize>),
    Unsat(Vec<usize>),
}

#[derive(Debug)]
pub enum Solution {
    Sat(Vec<usize>),
    Unsat(Vec<usize>),
}

/// Yield candidate solutions
///
/// Each iterator item tests the next best candidate solution.
/// If the next solution is proved false return Unsat,
/// if proved true and complete return Sat, otherwise Incomplete.
/// The iterator is exhausted when no more candidates can be tried.
pub trait IterSolve: Iterator<Item = CandidateSolution> + Sized {
    /// Only yield satisfying and unsatisfying solutions
    fn solution_iter(self) -> FilterMap<Self, fn(CandidateSolution) -> Option<Solution>> {
        self.filter_map(|s| match s {
            CandidateSolution::Sat(sat) => Some(Solution::Sat(sat)),
            CandidateSolution::Unsat(unsat) => Some(Solution::Unsat(unsat)),
            CandidateSolution::Incomplete => None,
        })
    }
}
