#[macro_use]
mod type_indexed_slice;
#[macro_use]
mod type_indexed_bitset;

mod pos;
mod neighbors;
mod value;
mod sudoku;

pub use crate::{
    pos::Pos,
    sudoku::Sudoku,
    value::Value,
};

#[cfg(feature = "solver")]
pub mod solver;

#[cfg(feature = "generator")]
pub mod generator;
