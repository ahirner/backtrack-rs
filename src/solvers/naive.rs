use crate::problem::{Check, Scope};
use crate::solve::CandidateSolution;

pub struct IterSolveNaive<'a, P: Scope + Check> {
    problem: &'a P,
    index: usize,         // current index into solution domain
    solution: Vec<usize>, // scratch pad, length is current level of solution
    domain: Vec<usize>,   // cached domain values from problem
    terminated: bool,     // whether all solutions have been checked
}

impl<'a, P: Scope + Check> IterSolveNaive<'a, P> {
    pub fn new(problem: &'a P) -> Self {
        let solution = Vec::with_capacity(problem.size());
        let domain = problem.domain();
        IterSolveNaive { problem, index: 0, solution, domain, terminated: false }
    }
}

impl<P: Scope + Check> Iterator for IterSolveNaive<'_, P> {
    type Item = CandidateSolution;

    fn next(&mut self) -> Option<CandidateSolution> {
        if self.terminated {
            return None;
        }

        let mut index = self.index;
        let candidate = self.domain[index];
        let sat = self.problem.extends_sat(self.solution.as_ref(), candidate);

        // increment search pointer and solution
        let solution = if sat {
            let complete = self.solution.len() + 1 == self.problem.size();
            if complete {
                // breadth-next
                index += 1;
                // todo: can we borrow until next candidate solution?
                let mut sat_solution = self.solution.clone();
                sat_solution.push(candidate);
                Some(CandidateSolution::Sat(sat_solution))
            } else {
                // depth-first
                self.solution.push(candidate);
                index = 0;
                Some(CandidateSolution::Incomplete)
            }
        } else {
            // breadth-next
            index += 1;

            // todo: can we borrow until next candidate solution?
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
