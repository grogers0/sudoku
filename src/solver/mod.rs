mod block;
mod col;
mod house;
mod line;
mod row;
mod solver;
mod strategies;
#[cfg(test)]
mod tests;

pub(crate) use block::Block;
pub(crate) use col::Col;
pub(crate) use house::House;
pub(crate) use line::Line;
pub(crate) use row::Row;
pub(crate) use crate::pos::{PosBitSet, PosIndexedSlice};
pub(crate) use crate::value::{ValueBitSet, ValueIndexedSlice};

pub use solver::{solve, SolveOpts, SolveResult, SolveSuccess};
pub use strategies::{Strategy, ALL, FAST};
