use crate::{
    pos::{Pos, PosBitSet},
    solver::{Row, Col},
};

/// All Rows then all Cols
pub(crate) struct Line(u8);

impl_index_type!(Line(u8), 18);
impl_type_indexed_slice!(LineIndexedSlice, Line, pub(crate));

impl Line {
    #[inline]
    pub const fn from_row(row: Row) -> Self {
        Self(row.as_usize() as u8)
    }

    #[inline]
    pub const fn from_col(col: Col) -> Self {
        Self((col.as_usize() + Row::N) as u8)
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
static MEMBER_VECS: LineIndexedSlice<Vec<Pos>> = {
    const EMPTY_POS_VEC: Vec<Pos> = Vec::new(); // Workaround for array initialization
    let mut ret = LineIndexedSlice::from_slice([EMPTY_POS_VEC; Line::N]);
    for row in Row::iter() {
        ret[Line::from_row(row)].extend(row.members_iter());
    }
    for col in Col::iter() {
        ret[Line::from_col(col)].extend(col.members_iter());
    }
    for line in Line::iter() {
        assert_eq!(ret[line].len(), Row::N);
    }
    ret
};

#[static_init::dynamic]
static MEMBER_BITSETS: LineIndexedSlice<PosBitSet> = {
    let mut ret = LineIndexedSlice::from_slice([PosBitSet::NONE; Line::N]);
    for line in Line::iter() {
        for &pos in &MEMBER_VECS[line] {
            ret[line].insert(pos);
        }
        assert_eq!(ret[line].len(), Row::N);
    }
    ret
};
