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
        self.len() == 0
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
    /// Produce `Accumulator` for quick-checking next candidates in `accu_sat`
    ///
    /// # Arguments
    /// * `accu`: accumulator from previous checks or `None` at first
    /// * `x`: solution candidate
    /// * `position`: position of `x` into a full solution (0-based)
    fn fold_acc(
        &self,
        accu: Option<Self::Accumulator>,
        x: &T,
        position: usize,
    ) -> Self::Accumulator;
    /// Check if next value is valid against accumulated checks from `fold_acc`
    ///
    /// # Arguments
    /// * `accu`: accumulated checks with prior values `x`
    /// * `x`: solution candidate
    /// * `position`: position of `x` into a full solution (0-based)
    fn accu_sat(&self, accu: &Self::Accumulator, x: &T, position: usize) -> bool;
}

impl<C: Check<T>, T> CheckInc<T> for C
where
    T: Debug + Clone,
{
    type Accumulator = Vec<T>;

    fn fold_acc(
        &self,
        accu: Option<Self::Accumulator>,
        x: &T,
        _position: usize,
    ) -> Self::Accumulator {
        let mut accu = accu.unwrap_or_default();
        accu.push(x.clone());
        accu
    }

    fn accu_sat(&self, accu: &Self::Accumulator, x: &T, _position: usize) -> bool {
        self.extends_sat(&accu[..accu.len() - 1], x)
    }
}

// note: can't blanket impl Check for CheckInc without conflicting with C impls,
// leaving that as doc that we would probably need something like C: !Check
// see: https://github.com/rust-lang/rfcs/issues/1053
