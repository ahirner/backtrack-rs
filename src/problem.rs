//! Traits defining a problem

use std::fmt::Debug;

/// Size and solution domain of a problem
pub trait Scope {
    /// Return number values needed for a complete solution
    fn size(&self) -> usize;
    /// Return all allowed values in a solution
    fn domain(&self) -> Vec<usize>;
}

/// Check if a combination of values is satisfactory
pub trait Check {
    /// Return true if new value extends an already valid partial solution
    ///
    /// # Arguments
    /// * `solution`: candidate solution from x_0 to x_l-1 deemed valid
    /// * `x`: new value
    fn extends_sat(&self, solution: &[usize], x: &usize) -> bool;
}

/// Check if a new value is satisfactory against reduced combinations incrementally.
///
/// This definition is for problems that benefit from reducing prior checks.
/// It also enables more efficient solvers.
pub trait CheckInc {
    type Accumulator: Debug;
    // todo: instead optional accu require Accumulator: Default?
    /// Produce an `Accumulator` for quick-checking next candidates in `accu_sat`
    fn fold_acc(&self, accu: Option<Self::Accumulator>, x: &usize) -> Self::Accumulator;
    /// Check if next value is valid against accumulated checks from `fold_acc`
    ///
    /// # Arguments
    /// * `accu`: accumulated checks before value `x`
    /// * `x`: new value
    /// * `index`: index of `x`, 0 is the first element, at index 0 `accu` is also `None`
    fn accu_sat(&self, accu: Option<&Self::Accumulator>, x: &usize, index: usize) -> bool;
}

impl<T: CheckInc> Check for T {
    fn extends_sat(&self, solution: &[usize], x: &usize) -> bool {
        let accu = solution.iter().fold(None, |a, x| Some(self.fold_acc(a, x)));
        self.accu_sat(accu.as_ref(), x, solution.len())
    }
}
