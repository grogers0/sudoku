use crate::{
    pos::{Pos, PosBitSet},
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
}

#[static_init::dynamic]
static MEMBER_VECS: BlockIndexedSlice<Vec<Pos>> = {
    const EMPTY_POS_VEC: Vec<Pos> = Vec::new(); // Workaround for array initialization
    let mut ret = BlockIndexedSlice::from_slice([EMPTY_POS_VEC; Block::N]);
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
