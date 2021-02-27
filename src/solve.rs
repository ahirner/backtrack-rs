use std::iter::FilterMap;

/// Items of some intermediate solution.
///
/// This type is used as item in iterative solvers.
/// Each enum is a test of the next best candidate solution.
/// The iterator is exhausted when no more candidates can be tried.
#[derive(Debug)]
pub enum CandidateSolution {
    /// Satisfactory solution (but too short)
    Incomplete,
    /// Satisfying and complete solution including its values
    Sat(Vec<usize>),
    /// Unsatisfying (and maybe too short) solution including its values
    Unsat(Vec<usize>),
}

/// Items of completed solutions.
#[derive(Debug)]
pub enum Solution {
    /// Satisfying and complete solution including its values
    Sat(Vec<usize>),
    /// Unsatisfying (and maybe too short) solution including its values
    Unsat(Vec<usize>),
}

/// Filter interesting solution candidates
///
pub trait IterSolveExt: Iterator<Item = CandidateSolution> + Sized {
    /// Only yield satisfying and unsatisfying solutions
    fn solution_iter(self) -> FilterMap<Self, fn(CandidateSolution) -> Option<Solution>> {
        self.filter_map(|s| match s {
            CandidateSolution::Sat(sat) => Some(Solution::Sat(sat)),
            CandidateSolution::Unsat(unsat) => Some(Solution::Unsat(unsat)),
            CandidateSolution::Incomplete => None,
        })
    }
    // Only yield satisfying solutions
    fn sat_iter(self) -> FilterMap<Self, fn(CandidateSolution) -> Option<Vec<usize>>> {
        self.filter_map(|s| match s {
            CandidateSolution::Sat(sat) => Some(sat),
            _ => None,
        })
    }
}

impl<T: Iterator<Item = CandidateSolution>> IterSolveExt for T {}
