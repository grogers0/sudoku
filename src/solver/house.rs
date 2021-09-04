use crate::{
    pos::{Pos, PosBitSet},
    solver::{Row, Col, Block, Line},
};

/// All Rows, then all Cols, then all Blocks
pub(crate) struct House(u8);

impl_index_type!(House(u8), 27);
impl_type_indexed_slice!(HouseIndexedSlice, House, pub(crate));

impl House {
    #[allow(dead_code)]
    #[inline]
    pub const fn from_row(row: Row) -> Self {
        Self(row.as_usize() as u8)
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn from_col(col: Col) -> Self {
        Self((col.as_usize() + Row::N) as u8)
    }

    #[inline]
    pub const fn from_line(line: Line) -> Self {
        Self(line.as_usize() as u8)
    }

    #[inline]
    pub const fn from_block(block: Block) -> Self {
        Self((block.as_usize() + {2 * Row::N}) as u8)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn members_iter(&self) -> impl Iterator<Item = Pos> {
        MEMBER_VECS[*self].iter().cloned()
    }

    #[inline]
    pub fn members_bitset(&self) -> PosBitSet {
        MEMBER_BITSETS[*self]
    }
}

#[static_init::dynamic]
static MEMBER_VECS: HouseIndexedSlice<Vec<Pos>> = {
    const EMPTY_POS_VEC: Vec<Pos> = Vec::new(); // Workaround for array initialization
    let mut ret = HouseIndexedSlice::from_slice([EMPTY_POS_VEC; House::N]);
    for line in Line::iter() {
        ret[House::from_line(line)].extend(line.members_iter());
    }
    for block in Block::iter() {
        ret[House::from_block(block)].extend(block.members_iter());
    }
    for house in House::iter() {
        assert_eq!(ret[house].len(), Row::N);
    }
    ret
};

#[static_init::dynamic]
static MEMBER_BITSETS: HouseIndexedSlice<PosBitSet> = {
    let mut ret = HouseIndexedSlice::from_slice([PosBitSet::NONE; House::N]);
    for house in House::iter() {
        for &pos in &MEMBER_VECS[house] {
            ret[house].insert(pos);
        }
        assert_eq!(ret[house].len(), Row::N);
    }
    ret
};
