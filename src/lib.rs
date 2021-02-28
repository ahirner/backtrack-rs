//! `backtrack-rs` lets you define and solve [backtracking](https://en.wikipedia.org/wiki/Backtracking) problems
//! succinctly.
//!
//! Problems are defined by their *scope* and *checks* against possible solutions.
//! A [Scope](crate::problem::Scope) determines length and allowed
//! values in a solution. The [Check](crate::problem::Check)
//! or [CheckInc](crate::problem::CheckInc) trait determines whether
//! a particular combination of values is satisfactory.
//!
//! ## Usage
//!
//! It is required that solutions shorter than in scope, i.e. partial
//! solutions must satisfy if the completed solutions should as well.
//! [Solvers](crate::solvers) borrow a problem in search for
//! [candidate solutions](crate::solve::CandidateSolution).
//!
//! ### Checks
//! We define the problem of counting down with a limited set of numbers and solve iteratively.
//! ```rust
//! use backtrack_rs::problem::{Check, Scope};
//! use backtrack_rs::solvers::IterSolveNaive;
//! // helper trait to filter solutions of interest
//! use backtrack_rs::solve::IterSolveExt;
//!
//! /// Obtain permutations of some 3 descending numbers
//! struct CountDown {}
//!
//! impl Scope for CountDown {
//!     fn size(&self) -> usize { 3 }
//!     fn domain(&self) -> Vec<usize> { (0..=3).collect() }
//! }
//!
//! impl Check for CountDown{
//!     fn extends_sat(&self, solution: &[usize], x: usize) -> bool {
//!         solution.last().map_or(true, |last| *last > x)
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
//! actual work on more than one prior value needs to be peformed
//! for any given sat check.
//!
//! ```rust
//! use backtrack_rs::problem::{CheckInc, Scope};
//! // ...
//! # use backtrack_rs::solvers::IterSolveNaive;
//! # use backtrack_rs::solve::IterSolveExt;
//! #  
//! # struct CountDown {}
//! #
//! # impl Scope for CountDown {
//! #     fn size(&self) -> usize { 3 }
//! #     fn domain(&self) -> Vec<usize> { (0..=3).collect() }
//! # }
//! #
//! impl CheckInc for CountDown{
//!     type Accumulator = usize;
//!
//!     fn fold_acc(&self, accu: Option<Self::Accumulator>, x: &usize) -> Self::Accumulator {
//!         // only last value is of interest for checking
//!         *x
//!     }
//!
//!     fn accu_sat(&self, accu: Option<&Self::Accumulator>, x: &usize, index: usize) -> bool {
//!        accu.map_or(true, |last| last > x)
//!     }
//! }
//! // since `CheckInc` impls `Check`, the same solver as in example above can be used
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
