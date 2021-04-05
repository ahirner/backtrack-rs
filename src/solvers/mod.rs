//! Solver implementations

pub mod naive;
pub mod cached;
pub use naive::IterSolveNaive;
pub use cached::IterSolveCached;
