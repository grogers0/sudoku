use crate::{
    pos::{Pos, PosBitSet},
    solver::Line,
};

pub(crate) struct Block(u8);

impl_index_type!(Block(u8), 9);
impl_type_indexed_slice!(BlockIndexedSlice, Block, pub(crate));

impl Block {
    #[inline]
    pub const fn from_pos(pos: Pos) -> Self {
        Self(pos.block())
    }

    #[inline]
    pub fn members_iter(&self) -> impl Iterator<Item = Pos> {
        MEMBER_VECS[*self].iter().cloned()
    }

    #[allow(dead_code)]
    #[inline]
    pub fn members_bitset(&self) -> PosBitSet {
        MEMBER_BITSETS[*self]
    }

    #[inline]
    pub fn intersecting_lines_iter(&self) -> impl Iterator<Item = Line> {
        INTERSECTING_LINES[*self].iter().cloned()
    }
}

#[static_init::dynamic]
static MEMBER_VECS: BlockIndexedSlice<Vec<Pos>> = {
    const EMPTY_VEC: Vec<Pos> = Vec::new(); // Workaround for array initialization
    let mut ret = BlockIndexedSlice::from_slice([EMPTY_VEC; Block::N]);
    for pos in Pos::iter() {
        ret[Block::from_pos(pos)].push(pos);
    }
    for block in Block::iter() {
        assert_eq!(ret[block].len(), Block::N);
    }
    ret
};

#[static_init::dynamic]
static MEMBER_BITSETS: BlockIndexedSlice<PosBitSet> = {
    let mut ret = BlockIndexedSlice::from_slice([PosBitSet::NONE; Block::N]);
    for block in Block::iter() {
        for &pos in &MEMBER_VECS[block] {
            ret[block].insert(pos);
        }
        assert_eq!(ret[block].len(), Block::N);
    }
    ret
};

#[static_init::dynamic]
static INTERSECTING_LINES: BlockIndexedSlice<Vec<Line>> = {
    const EMPTY_VEC: Vec<Line> = Vec::new(); // Workaround for array initialization
    let mut ret = BlockIndexedSlice::from_slice([EMPTY_VEC; Block::N]);
    for block in Block::iter() {
        for line in Line::iter() {
            if !(block.members_bitset() & line.members_bitset()).is_empty() {
                ret[block].push(line);
            }
        }
        assert_eq!(ret[block].len(), 6); // 3 Rows, 3 Cols
    }
    ret
};
