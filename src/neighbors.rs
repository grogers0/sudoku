use crate::{
    pos::{Pos, PosIndexedSlice, PosBitSet},
};


pub fn neighbor_positions(pos: Pos) -> impl Iterator<Item = Pos> {
    NEIGHBOR_POSITIONS[pos].iter().cloned()
}

pub(crate) fn neighbor_bitset(pos: Pos) -> PosBitSet {
    NEIGHBOR_BITSETS[pos]
}

/// Every cell has 8 neighbors in its block, and 6 in its row and col (which aren't in its block)
const NUM_NEIGHBORS: usize = 20;

#[static_init::dynamic]
static NEIGHBOR_POSITIONS: PosIndexedSlice<[Pos; NUM_NEIGHBORS]> = calc_neighbor_positions();

#[static_init::dynamic]
static NEIGHBOR_BITSETS: PosIndexedSlice<PosBitSet> = calc_neighbor_bitsets();


fn calc_neighbor_positions() -> PosIndexedSlice<[Pos; NUM_NEIGHBORS]> {
    let mut ret = PosIndexedSlice::from_slice([[Pos::new(0); NUM_NEIGHBORS]; Pos::N]);
    for pos in Pos::iter() {
        ret[pos] = calc_neighbor_positions_for(pos);
    }
    ret
}

fn calc_neighbor_positions_for(pos: Pos) -> [Pos; NUM_NEIGHBORS] {
    let mut ret = [pos; NUM_NEIGHBORS];
    let mut idx = 0;
    for pos2 in Pos::iter() {
        if pos == pos2 { continue }
        if pos.row() == pos2.row() ||
            pos.col() == pos2.col() ||
            pos.block() == pos2.block()
        {
            ret[idx] = pos2;
            idx += 1;
        }
    }
    ret
}

fn calc_neighbor_bitsets() -> PosIndexedSlice<PosBitSet> {
    let mut ret = PosIndexedSlice::from_slice([PosBitSet::NONE; Pos::N]);
    for pos in Pos::iter() {
        for &pos2 in NEIGHBOR_POSITIONS[pos].iter() {
            ret[pos].set(pos2);
        }
    }
    ret
}
