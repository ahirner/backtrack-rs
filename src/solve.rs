use crate::problem::Problem;

#[derive(Debug)]
pub enum Solution {
    Sat(Vec<usize>),
    Unsat(Vec<usize>),
}

// todo: solve generically over Problem
pub struct IterSolver {
    problem: Box<dyn Problem>,
    index: usize,         // current index into solution domain
    solution: Vec<usize>, // scratch pad, length is current level of solution
    domain: Vec<usize>,   // cached domain values from problem
    terminated: bool,     // whether all solutions have been checked
}

impl IterSolver {
    pub fn new(problem: Box<dyn Problem>) -> IterSolver {
        let solution = Vec::with_capacity(problem.get_n());
        let domain = problem.get_domain();
        IterSolver { problem, index: 0, solution, domain, terminated: false }
    }

    /// Advance solver with next candidate solution
    ///
    /// If the next solution is proved false return Unsat,
    /// if true and complete return Sat, otherwise None.
    /// Once all solutions are tried, the solver's terminated flag is set
    /// and subsequent calls panic.
    fn push_sat(&mut self) -> Option<Solution> {
        assert!(!self.terminated, "Tried to push solution whereas all candidates are exhausted");
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
                Some(Solution::Sat(self.solution.clone()))
            } else {
                // depth-first
                index = 0;
                None
            }
        } else {
            // breadth-next
            index += 1;

            // todo: can we move cloning to caller and hence make optional?
            let mut unsat_solution = self.solution.clone();
            unsat_solution.push(candidate);
            Some(Solution::Unsat(unsat_solution))
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

impl Iterator for IterSolver {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.terminated {
            if let Some(solution) = self.push_sat() {
                return Some(solution);
            }
        }
        None
    }
}
