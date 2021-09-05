use super::StrategyResult;
use crate::{
    Pos, Sudoku,
};

pub(crate) fn naked_single(sudoku: &Sudoku) -> Option<StrategyResult> {
    for pos in Pos::iter() {
        let candidates = sudoku.get_candidates_by_pos(pos);
        if candidates.len() == 1 {
            let val = candidates.iter().next().unwrap();
            return Some(StrategyResult::NakedSingle(pos, val));
        }
    }
    None
}
