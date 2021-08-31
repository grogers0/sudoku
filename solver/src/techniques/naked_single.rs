use crate::technique::TechniqueResult;
use std::num::NonZeroU8;
use sudoku_board::{Pos, Sudoku};

pub(crate) fn naked_single(sudoku: &Sudoku) -> TechniqueResult {
    let mut ret = Vec::new();
    for pos in Pos::iter() {
        let candidates = sudoku.get_candidates_by_pos(pos);
        if candidates.count_ones() == 1 {
            let value = NonZeroU8::new(candidates.iter().next().unwrap() as u8 + 1).unwrap();
            ret.push((pos, value));
        }
    }
    TechniqueResult {
        false_candidates: Vec::new(),
        true_candidates: ret
    }
}
