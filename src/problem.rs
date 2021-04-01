//! Traits defining a problem

use std::fmt::Debug;

/// Size and solution domain of a problem
pub trait Scope<'a, T: 'a = usize> {
    /// Return number values needed for a complete solution
    fn size(&self) -> usize;

    /// Return value from solution domain
    /// # Arguments
    /// * `index`: index < than `len` into domain
    fn value(&'a self, index: usize) -> T;
    /// Return number of domain values
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() > 0
    }

    // todo: is this good practice or we want some IntoIterator impl?
    fn iter_values(&'a self) -> ScopeIter<'a, T>
    where
        Self: Sized,
    {
        ScopeIter { index: 0, scope: self }
    }
}

pub struct ScopeIter<'a, T> {
    index: usize,
    scope: &'a dyn Scope<'a, T>,
}

impl<'a, T> Iterator for ScopeIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.scope.len() {
            None
        } else {
            self.index += 1;
            Some(self.scope.value(self.index - 1))
        }
    }
}

/// Check if a combination of values is satisfactory
pub trait Check<T = usize> {
    /// Return true if new value extends an already valid partial solution
    ///
    /// # Arguments
    /// * `solution`: candidate solution from x_0 to x_l-1 deemed valid
    /// * `x`: new value
    // todo: accept x: Borrow<T> or similar if without loss in ergonomics
    fn extends_sat(&self, solution: &[T], x: &T) -> bool;
}

/// Check if a new value is satisfactory against reduced combinations incrementally.
///
/// This definition is for problems that benefit from reducing prior checks.
/// It also enables more efficient solvers.
pub trait CheckInc<T = usize> {
    type Accumulator: Debug;
    // todo: instead optional accu require Accumulator: Default?
    /// Produce an `Accumulator` for quick-checking next candidates in `accu_sat`
    fn fold_acc(&self, accu: Option<&Self::Accumulator>, x: &T) -> Self::Accumulator;
    /// Check if next value is valid against accumulated checks from `fold_acc`
    ///
    /// # Arguments
    /// * `accu`: accumulated checks before value `x`
    /// * `x`: new value
    /// * `index`: index of `x`, 0 is the first element, at index 0 `accu` is also `None`
    fn accu_sat(&self, accu: Option<&Self::Accumulator>, x: &T, index: usize) -> bool;
}

impl<C: CheckInc<T>, T> Check<T> for C {
    fn extends_sat(&self, solution: &[T], x: &T) -> bool {
        let mut accu = None;
        // manual fold with locally owned accu
        for s in solution {
            accu = Some(self.fold_acc(accu.as_ref(), s));
        }
        self.accu_sat(accu.as_ref(), x, solution.len())
    }
}
