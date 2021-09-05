mod block;
mod col;
mod house;
mod line;
mod row;
mod solver;
mod strategies;
mod strategy;

pub(crate) use block::Block;
pub(crate) use col::Col;
pub(crate) use house::House;
pub(crate) use line::Line;
pub(crate) use row::Row;

pub use solver::{solve, SolveOpts, SolveResult};
pub use strategies::{ALL, FAST};
pub use strategy::Strategy;
