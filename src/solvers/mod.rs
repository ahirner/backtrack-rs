//! Solver implementations

pub mod cached;
pub mod naive;
pub use cached::IterSolveCached;
pub use naive::IterSolveNaive;
