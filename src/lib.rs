//! `backtrack` lets you solve [backtracking](https://en.wikipedia.org/wiki/Backtracking) problems
//! simply and generically.
//!
//! Problems are defined by their *scope* and *checks* against possible solutions.
//!
//! A [Scope](crate::problem::Scope) determines length and allowed values of a solution.
//! The domain defaults to `usize`, but any `T` works if it lives as long as its `Scope`, including references.
//!
//! The [Check](crate::problem::Check) or [CheckInc](crate::problem::CheckInc) trait determines whether
//! a particular combination of values is satisfying.
//!
//!
//! ## Usage
//!
//! It is required that solutions shorter than the entire scope, i.e. partial
//! solutions must satisfy if the completed solutions should as well.
//!
//! [Solvers](crate::solvers) borrow a problem in search for
//! [candidate solutions](crate::solve::CandidateSolution).
//!
//! ### Checks
//! We define the problem of counting down with a limited set of numbers and solve iteratively.
//! ```rust
//! use backtrack::problem::{Check, Scope};
//! use backtrack::solvers::IterSolveNaive;
//! // helper trait to filter solutions of interest
//! use backtrack::solve::IterSolveExt;
//!
//! /// Obtain permutations of some 3 descending numbers
//! struct CountDown {}
//!
//! impl Scope<'_> for CountDown {
//!     fn size(&self) -> usize { 3 }
//!     fn value(&self, index: usize) -> usize { index }
//!     fn len(&self) -> usize { 4 }
//! }
//!
//! impl Check for CountDown{
//!     fn extends_sat(&self, solution: &[usize], x: &usize) -> bool {
//!         solution.last().map_or(true, |last| *last > *x)
//!     }
//! }
//!
//! let solver = IterSolveNaive::new(&CountDown{});
//! let mut sats = solver.sat_iter();
//!
//! assert_eq!(sats.next(), Some(vec![2, 1, 0]));
//! assert_eq!(sats.next(), Some(vec![3, 1, 0]));
//! assert_eq!(sats.next(), Some(vec![3, 2, 0]));
//! assert_eq!(sats.next(), Some(vec![3, 2, 1]));
//! assert_eq!(sats.next(), None);
//! ```
//! ### Incremental Checks
//! If your checks can be formulated against a reduced solution,
//! implement [CheckInc](crate::problem::CheckInc) instead.
//!
//! The same result as above can be obtained by first "computing"
//! the last item at each step. Such an approach makes more sense if
//! work on more than one prior value needs to be peformed
//! for any given sat check.
//!
//! ```rust
//! use backtrack::problem::{CheckInc, Scope};
//! // ...
//! # use backtrack::solvers::IterSolveNaive;
//! # use backtrack::solve::IterSolveExt;
//! #  
//! # struct CountDown {}
//! #
//! # impl Scope<'_> for CountDown {
//! #     fn size(&self) -> usize { 3 }
//! #     fn value(&self, index: usize) -> usize { index }
//! #     fn len(&self) -> usize { 4 }
//! # }
//! #
//! impl CheckInc for CountDown{
//!     type Accumulator = usize;
//!
//!     fn fold_acc(&self, accu: Option<Self::Accumulator>, x: &usize, index: usize) -> Self::Accumulator {
//!         // only last value is of interest for checking
//!         *x
//!     }
//!
//!     fn accu_sat(&self, accu: Option<&Self::Accumulator>, x: &usize, index: usize) -> bool {
//!        accu.map_or(true, |last| last > x)
//!     }
//! }
//! // since `CheckInc` impls `Check`, the same solver as before can be used
//! // todo: specialize solver to actually realize performance advantage
//! // ...
//! #
//! # let solver = IterSolveNaive::new(&CountDown{});
//! # let mut sats = solver.sat_iter();
//! #
//! # assert_eq!(sats.next(), Some(vec![2, 1, 0]));
//! # assert_eq!(sats.next(), Some(vec![3, 1, 0]));
//! # assert_eq!(sats.next(), Some(vec![3, 2, 0]));
//! # assert_eq!(sats.next(), Some(vec![3, 2, 1]));
//! # assert_eq!(sats.next(), None);
//! ```
pub mod problem;
pub mod problems;
pub mod solve;
pub mod solvers;
