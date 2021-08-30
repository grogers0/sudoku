use crate::{
    bitset::BitSet81,
    pos::Pos,
};


pub fn neighbor_positions(pos: Pos) -> &'static [Pos; 20] {
    &NEIGHBOR_POSITIONS[pos.idx()]
}

pub fn neighbor_bitset(pos: Pos) -> BitSet81 {
    NEIGHBOR_BITSETS[pos.idx()]
}

const NEIGHBOR_POSITIONS: [[Pos; 20]; 81] = calc_neighbor_positions();
const NEIGHBOR_BITSETS: [BitSet81; 81] = calc_neighbor_bitsets();

const fn calc_neighbor_positions() -> [[Pos; 20]; 81] {
    let mut ret = [[Pos::MIN; 20]; 81];
    let mut pos_raw = 0;
    while pos_raw < 81 {
        ret[pos_raw] = calc_neighbor_positions_for(pos_raw);
        pos_raw += 1;
    }
    ret
}

const fn calc_neighbor_positions_for(pos_raw: usize) -> [Pos; 20] {
    let pos = unsafe { Pos::new_unchecked(pos_raw) };
    let mut ret = [Pos::MIN; 20];
    let mut pos2_raw = 0;
    let mut idx = 0;
    while pos2_raw < 81 {
        let pos2 = unsafe { Pos::new_unchecked(pos2_raw) };

        if pos_raw != pos2_raw &&
            (pos.row() == pos2.row() ||
             pos.col() == pos2.col() ||
             pos.block() == pos2.block())
        {
            ret[idx] = pos2;
            idx += 1;
        }
        pos2_raw += 1;
    }
    ret
}

const fn calc_neighbor_bitsets() -> [BitSet81; 81] {
    let mut ret = [BitSet81::NONE; 81];
    let mut pos_raw = 0;
    while pos_raw < 81 {
        let neighbor_positions = &NEIGHBOR_POSITIONS[pos_raw];
        let mut idx = 0;
        while idx < neighbor_positions.len() {
            // Rustc doesn't allow const trait fns, see https://github.com/rust-lang/rust/issues/67792
            ret[pos_raw] = BitSet81(ret[pos_raw].0 | unsafe { BitSet81::single_unchecked(neighbor_positions[idx].idx()).0 });
            idx += 1;
        }
        pos_raw += 1;
    }
    ret
}
