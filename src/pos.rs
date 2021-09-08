use std::fmt;

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
pub struct Pos(u8);

impl_index_type!(Pos(u8), 81);
impl_type_indexed_slice!(PosIndexedSlice, Pos, pub(crate));
impl_type_indexed_bitset!(PosBitSet, Pos, u128, PosBitSetIter, pub(crate));

impl Pos {
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
    pub(crate) fn neighbors_iter(&self) -> impl Iterator<Item = Pos> + Clone {
        NEIGHBOR_VECS[*self].iter().cloned()
    }

    #[inline]
    pub(crate) fn neighbors_bitset(&self) -> PosBitSet {
        NEIGHBOR_BITSETS[*self]
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r{}c{}", self.row() + 1, self.col() + 1)
    }
}

const NUM_NEIGHBORS: usize = 20; // 8 in block, 6 in row (not in block), 6 in col (not in block)

#[static_init::dynamic]
static NEIGHBOR_VECS: PosIndexedSlice<Vec<Pos>> = {
    const EMPTY_POS_VEC: Vec<Pos> = Vec::new(); // Workaround for array initialization
    let mut ret = PosIndexedSlice::from_slice([EMPTY_POS_VEC; Pos::N]);
    for pos in Pos::iter() {
        for pos2 in Pos::iter() {
            if pos == pos2 { continue }
            if pos.row() == pos2.row() ||
                pos.col() == pos2.col() ||
                pos.block() == pos2.block()
            {
                ret[pos].push(pos2);
            }
        }
    }
    for vec in &ret {
        assert_eq!(vec.len(), NUM_NEIGHBORS);
    }
    ret
};

#[static_init::dynamic]
static NEIGHBOR_BITSETS: PosIndexedSlice<PosBitSet> = {
    let mut ret = PosIndexedSlice::from_slice([PosBitSet::NONE; Pos::N]);
    for pos in Pos::iter() {
        for &pos2 in NEIGHBOR_VECS[pos].iter() {
            ret[pos].insert(pos2);
        }
    }
    for bitset in &ret {
        assert_eq!(bitset.len(), NUM_NEIGHBORS);
    }
    ret
};

