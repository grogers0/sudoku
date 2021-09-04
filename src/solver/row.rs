use crate::{
    pos::{Pos, PosBitSet},
};

pub(crate) struct Row(u8);

impl_index_type!(Row(u8), 9);
impl_type_indexed_slice!(RowIndexedSlice, Row, pub(crate));

impl Row {
    #[inline]
    pub const fn from_pos(pos: Pos) -> Self {
        Self(pos.row())
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
static MEMBER_VECS: RowIndexedSlice<Vec<Pos>> = {
    const EMPTY_POS_VEC: Vec<Pos> = Vec::new(); // Workaround for array initialization
    let mut ret = RowIndexedSlice::from_slice([EMPTY_POS_VEC; Row::N]);
    for pos in Pos::iter() {
        ret[Row::from_pos(pos)].push(pos);
    }
    for row in Row::iter() {
        assert_eq!(ret[row].len(), Row::N);
    }
    ret
};

#[static_init::dynamic]
static MEMBER_BITSETS: RowIndexedSlice<PosBitSet> = {
    let mut ret = RowIndexedSlice::from_slice([PosBitSet::NONE; Row::N]);
    for row in Row::iter() {
        for &pos in &MEMBER_VECS[row] {
            ret[row].insert(pos);
        }
        assert_eq!(ret[row].len(), Row::N);
    }
    ret
};
