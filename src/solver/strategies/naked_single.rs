use crate::{
    solver::strategy::StrategyResult,
    Pos, Sudoku,
};

pub(crate) fn naked_single(sudoku: &Sudoku) -> StrategyResult {
    let mut ret = Vec::new();
    for pos in Pos::iter() {
        let candidates = sudoku.get_candidates_by_pos(pos);
        if candidates.len() == 1 {
            ret.push((pos, candidates.iter().next().unwrap()));
        }
    }
    StrategyResult {
        false_candidates: Vec::new(),
        true_candidates: ret
    }
}
