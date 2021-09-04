use crate::{
    pos::{Pos, PosBitSet},
};

pub(crate) struct Col(u8);

impl_index_type!(Col(u8), 9);
impl_type_indexed_slice!(ColIndexedSlice, Col, pub(crate));

impl Col {
    #[inline]
    pub const fn from_pos(pos: Pos) -> Self {
        Self(pos.col())
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
static MEMBER_VECS: ColIndexedSlice<Vec<Pos>> = {
    const EMPTY_POS_VEC: Vec<Pos> = Vec::new(); // Workaround for array initialization
    let mut ret = ColIndexedSlice::from_slice([EMPTY_POS_VEC; Col::N]);
    for pos in Pos::iter() {
        ret[Col::from_pos(pos)].push(pos);
    }
    for col in Col::iter() {
        assert_eq!(ret[col].len(), Col::N);
    }
    ret
};

#[static_init::dynamic]
static MEMBER_BITSETS: ColIndexedSlice<PosBitSet> = {
    let mut ret = ColIndexedSlice::from_slice([PosBitSet::NONE; Col::N]);
    for col in Col::iter() {
        for &pos in &MEMBER_VECS[col] {
            ret[col].insert(pos);
        }
        assert_eq!(ret[col].len(), Col::N);
    }
    ret
};
