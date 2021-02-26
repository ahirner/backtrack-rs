use std::fmt::Debug;

/// Size and solution domain of a problem
pub trait Scope {
    /// Return size of the problem
    fn get_n(&self) -> usize;
    /// Return all possible solution values to try
    fn get_domain(&self) -> Vec<usize>;
}

/// Backtracking Problem that checks over domain values
pub trait Problem {
    /// Return true if new value extends an already valid partial solution
    ///
    /// # Arguments
    /// * `solution`: candidate solution from x_0 to x_l-1
    /// * `x_l`: new potential solution value
    fn extends_sat(&self, solution: &[usize], x_l: usize) -> bool;
}

/// Backtracking problem which benefits from accumulating prior checks
pub trait ProblemInc {
    type Accumulator: Debug;
    // todo: instead optional accu require Accumulator: Default?
    /// Produce `Accumulator` for quick-checking next candidates in `accu_sat`
    fn fold_acc(&self, accu: Option<Self::Accumulator>, x: &usize) -> Self::Accumulator;
    /// Check if next value is valid against accumulated checks in `fold_acc`
    ///
    /// # Arguments
    /// * `accu`: all accumulated checks before `x`
    /// * `x`: new solution value
    /// * `index`: index of `x`, 0 is the first element and `accu` is also None
    fn accu_sat(&self, accu: Option<&Self::Accumulator>, x: &usize, index: usize) -> bool;
}

impl<T: ProblemInc> Problem for T {
    fn extends_sat(&self, solution: &[usize], x_l: usize) -> bool {
        let accu = solution.iter().fold(None, |a, x| Some(self.fold_acc(a, x)));
        self.accu_sat(accu.as_ref(), &x_l, solution.len())
    }
}
