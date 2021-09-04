use std::{
    iter,
    ops::Range,
};

/// The position of a cell on the board, numbered as:
/// ```text
/// 0  1  2  | 3  4  5  | 6  7  8
/// 9  10 11 | 12 13 14 | 15 16 17
/// 18 19 20 | 21 22 23 | 24 25 26
/// ---------+----------+---------
/// 27 28 29 | 30 31 32 | 33 34 35
/// 36 37 38 | 39 40 41 | 42 43 44
/// 45 46 47 | 48 49 50 | 51 52 53
/// ---------+----------+---------
/// 54 55 56 | 57 58 59 | 60 61 62
/// 63 64 65 | 66 67 68 | 69 70 71
/// 72 73 74 | 75 76 77 | 78 79 80
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Pos(u8);

impl_type_indexed_slice!(PosIndexedSlice, Pos, pub(crate));
impl_type_indexed_bitset!(PosBitSet, Pos, u128, PosBitSetIter, pub(crate));

impl Pos {
    pub const N: usize = 81;

    #[inline]
    pub fn new(idx: usize) -> Self {
        if idx >= Self::N as usize { panic!("Index out of bounds") }
        Self(idx as u8)
    }

    #[inline]
    pub const unsafe fn new_unchecked(idx: usize) -> Self {
        Self(idx as u8)
    }

    #[inline]
    pub fn row_col(row: u8, col: u8) -> Self {
        if row >= 9 { panic!("Row out of bounds") }
        if col >= 9 { panic!("Col out of bounds") }
        Self(row * 9 + col)
    }

    #[inline]
    pub const fn row(&self) -> u8 {
        self.0 / 9
    }

    #[inline]
    pub const fn col(&self) -> u8 {
        self.0 % 9
    }

    /// 
    /// Blocks are numbered in the same way as positions, e.g.
    /// ```text
    /// 0 1 2
    /// 3 4 5
    /// 6 7 8
    /// ```
    #[inline]
    pub const fn block(&self) -> u8 {
        (self.row() / 3) * 3 + (self.col() / 3) // TODO - better as a lookup table?
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn iter() -> iter::Map<Range<usize>, fn(usize) -> Self> {
        (0..Self::N).map(|idx| unsafe { Self::new_unchecked(idx) })
    }
}
