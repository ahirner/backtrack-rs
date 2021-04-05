use crate::problem::{CheckInc, Scope};
use crate::solve::CandidateSolution;

/// Backtracking solver without planning that caches solution state
pub struct IterSolveCached<'p, P: Scope<'p, T> + CheckInc<T>, T = usize>
where
    T: 'p,
{
    problem: &'p P,
    /// Scratch pad of indices into domain, length is current level of solution
    solution_indices: Vec<usize>,
    /// Scratch pad solution, length is current level of partially satisfied solution
    solution: Vec<T>,
    /// Scratch pad accumulations, length is current level of solution
    states: Vec<P::Accumulator>,
}

impl<'p, P: Scope<'p, T> + CheckInc<T>, T> IterSolveCached<'p, P, T>
where
    P::Accumulator: Clone,
{
    pub fn new(problem: &'p P) -> Self {
        let mut solution_index = Vec::with_capacity(problem.size());
        if !(problem.is_empty() || problem.size() == 0) {
            solution_index.push(0);
        }

        let solution = Vec::with_capacity(problem.size());
        let states = Vec::with_capacity(problem.size());

        IterSolveCached { problem, solution_indices: solution_index, solution, states }
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

impl<'p, P: Scope<'p, T> + CheckInc<T>, T> Iterator for IterSolveCached<'p, P, T>
where
    P::Accumulator: Clone,
{
    type Item = CandidateSolution<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // todo: push/pop less
        let mut index = match self.solution_indices.pop() {
            Some(index) => index,
            // terminated through backtracking
            None => return None,
        };

        let candidate = self.value(index);
        let accu = self.problem.fold_acc(
            // todo: rethink accumulator ownership -> &mut?
            self.states.last().cloned(),
            &candidate,
            self.solution_indices.len(),
        );
        let sat = self.problem.accu_sat(&accu, &candidate, self.solution_indices.len());

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
                self.states.push(accu);
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
        while index == self.problem.len() {
            if let Some(old_index) = self.solution_indices.pop() {
                index = old_index + 1;
            } else {
                return solution;
            }
            self.solution.truncate(self.solution_indices.len());
            self.states.truncate(self.solution_indices.len());
        }

        self.solution_indices.push(index);
        solution
    }
}
