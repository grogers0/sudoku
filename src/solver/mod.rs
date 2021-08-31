mod solver;
mod strategy;
mod strategies;

pub use strategy::Strategy;
pub use strategies::{ALL, FAST};
pub use solver::{solve, SolveOpts, SolveResult};
