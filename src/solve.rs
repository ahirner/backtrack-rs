use crate::problem::Problem;
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

pub struct IterSolveNaive<'a, P: Problem> {
    problem: &'a P,
    index: usize,         // current index into solution domain
    solution: Vec<usize>, // scratch pad, length is current level of solution
    domain: Vec<usize>,   // cached domain values from problem
    terminated: bool,     // whether all solutions have been checked
}

impl<'a, P: Problem> IterSolveNaive<'a, P> {
    pub fn new(problem: &'a P) -> Self {
        let solution = Vec::with_capacity(problem.get_n());
        let domain = problem.get_domain();
        IterSolveNaive { problem, index: 0, solution, domain, terminated: false }
    }
}

// todo: how to blanket impl?
impl<P: Problem> IterSolve for IterSolveNaive<'_, P> {}

impl<P: Problem> Iterator for IterSolveNaive<'_, P> {
    type Item = CandidateSolution;

    fn next(&mut self) -> Option<CandidateSolution> {
        if self.terminated {
            return None;
        }

        let mut index = self.index;
        let candidate = self.domain[index];
        let sat = self.problem.inc_sat(self.solution.as_ref(), candidate);

        // increment search pointer and solution
        let solution = if sat {
            self.solution.push(candidate);

            let complete = self.solution.len() == self.problem.get_n();
            if complete {
                // breadth-next
                index += 1;
                // todo: can we borrow until the next candidate solution?
                Some(CandidateSolution::Sat(self.solution.clone()))
            } else {
                // depth-first
                index = 0;
                Some(CandidateSolution::Incomplete)
            }
        } else {
            // breadth-next
            index += 1;

            // todo: turn into borrow + copy of index if borrow would
            // also work for `Sat`
            let mut unsat_solution = self.solution.clone();
            unsat_solution.push(candidate);
            Some(CandidateSolution::Unsat(unsat_solution))
        };

        // back-track or terminate
        while index == self.domain.len() {
            let last_candidate = self.solution.pop();
            if let Some(last) = last_candidate {
                // todo: cache indices
                index = self.domain.iter().position(|i| *i == last).unwrap() + 1;
            } else {
                self.terminated = true;
                break;
            }
        }

        self.index = index;
        solution
    }
}
