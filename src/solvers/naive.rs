use crate::problem::{Check, Scope};
use crate::solve::CandidateSolution;

/// Backtracking solver without planning
pub struct IterSolveNaive<'p, P: Scope<'p, T> + Check<T>, T = usize>
where
    T: 'p,
{
    problem: &'p P,
    /// Scratch pad of indices into domain, length is current level of solution
    solution_indices: Vec<usize>,
    /// Scratch pad solution, length is current level of partially satisfied solution
    solution: Vec<T>,
}

impl<'p, P: Scope<'p, T> + Check<T>, T> IterSolveNaive<'p, P, T> {
    pub fn new(problem: &'p P) -> Self {
        let mut solution_index = Vec::with_capacity(problem.size());
        if !(problem.is_empty() || problem.size() == 0) {
            solution_index.push(0);
        }

        let solution = Vec::with_capacity(problem.size());

        IterSolveNaive { problem, solution_indices: solution_index, solution }
    }

    fn value(&self, index: usize) -> T {
        self.problem.value(index)
    }

    fn make_solution(&self) -> Vec<T> {
        let mut solution = Vec::with_capacity(self.solution_indices.len() + 1);
        for i in self.solution_indices.iter() {
            solution.push(self.value(*i));
        }
        solution
    }
}

impl<'p, P: Scope<'p, T> + Check<T>, T> Iterator for IterSolveNaive<'p, P, T> {
    type Item = CandidateSolution<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // todo: push/pop less
        let mut index = match self.solution_indices.pop() {
            Some(index) => index,
            // terminated through backtracking
            None => return None,
        };

        let candidate = self.value(index);
        let sat = self.problem.extends_sat(self.solution.as_slice(), &candidate);

        // increment search pointer and solution
        let solution = if sat {
            let complete = self.solution_indices.len() + 1 == self.problem.size();
            if complete {
                let mut sat_solution = self.make_solution();
                sat_solution.push(candidate);
                // breadth-next
                index += 1;
                Some(CandidateSolution::Sat(sat_solution))
            } else {
                self.solution.push(candidate);
                self.solution_indices.push(index);
                // depth-first
                index = 0;
                Some(CandidateSolution::Incomplete)
            }
        } else {
            let mut unsat_solution = self.make_solution();
            unsat_solution.push(candidate);
            // breadth-next
            index += 1;
            Some(CandidateSolution::Unsat(unsat_solution))
        };

        // backtrack
        let mut terminated = false;
        while index == self.problem.len() {
            if let Some(old_index) = self.solution_indices.pop() {
                index = old_index + 1;
            } else {
                terminated = true;
                break;
            }
            self.solution.truncate(self.solution_indices.len());
        }

        if !terminated {
            self.solution_indices.push(index);
        }
        solution
    }
}
