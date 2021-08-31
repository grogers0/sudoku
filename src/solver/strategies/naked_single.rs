use crate::{
    solver::strategy::StrategyResult,
    Pos, Sudoku,
};
use std::num::NonZeroU8;

pub(crate) fn naked_single(sudoku: &Sudoku) -> StrategyResult {
    let mut ret = Vec::new();
    for pos in Pos::iter() {
        let candidates = sudoku.get_candidates_by_pos(pos);
        if candidates.count_ones() == 1 {
            let value = NonZeroU8::new(candidates.iter().next().unwrap() as u8 + 1).unwrap();
            ret.push((pos, value));
        }
    }
    StrategyResult {
        false_candidates: Vec::new(),
        true_candidates: ret
    }
}
