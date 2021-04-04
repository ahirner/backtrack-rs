//! Types defining solutions and help working with them

use std::iter::FilterMap;

/// Items of some intermediate solution.
///
/// This type is used as item in iterative solvers.
/// Each enum is a test of the next best candidate solution.
/// The iterator is exhausted when no more candidates can be tried.
#[derive(Debug)]
pub enum CandidateSolution<T = usize> {
    /// Satisfactory solution (but too short)
    Incomplete,
    /// Satisfying and complete solution including its values
    Sat(Vec<T>),
    /// Unsatisfying (and maybe too short) solution including its values
    Unsat(Vec<T>),
}

/// Items of completed solutions.
#[derive(Debug)]
pub enum Solution<T = usize> {
    /// Satisfying and complete solution including its values
    Sat(Vec<T>),
    /// Unsatisfying (and maybe too short) solution including its values
    Unsat(Vec<T>),
}

type MaybeSolution<T> = Option<Solution<T>>;
type MaybeFilteredSolution<T> = Option<Vec<T>>;

type SolutionMap<S, T> = FilterMap<S, fn(CandidateSolution<T>) -> MaybeSolution<T>>;
type SpecificSolutionMap<S, T> = FilterMap<S, fn(CandidateSolution<T>) -> MaybeFilteredSolution<T>>;

/// Filter interesting solution candidates
///
pub trait IterSolveExt<T>: Iterator<Item = CandidateSolution<T>> + Sized {
    /// Only yield satisfying and unsatisfying solutions
    fn solution_iter(self) -> SolutionMap<Self, T> {
        self.filter_map(|s| match s {
            CandidateSolution::<T>::Sat(sat) => Some(Solution::Sat(sat)),
            CandidateSolution::<T>::Unsat(unsat) => Some(Solution::Unsat(unsat)),
            CandidateSolution::<T>::Incomplete => None,
        })
    }
    // Only yield satisfying solutions
    fn sat_iter(self) -> SpecificSolutionMap<Self, T> {
        self.filter_map(|s| match s {
            CandidateSolution::<T>::Sat(sat) => Some(sat),
            _ => None,
        })
    }
    // Only yield unsatisfying solutions
    fn unsat_iter(self) -> SpecificSolutionMap<Self, T> {
        self.filter_map(|s| match s {
            CandidateSolution::<T>::Unsat(sat) => Some(sat),
            _ => None,
        })
    }
}

impl<T, I: Iterator<Item = CandidateSolution<T>>> IterSolveExt<T> for I {}
