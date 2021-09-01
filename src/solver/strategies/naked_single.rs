use crate::{
    solver::strategy::StrategyResult,
    Pos, Sudoku, Value,
};

pub(crate) fn naked_single(sudoku: &Sudoku) -> StrategyResult {
    let mut ret = Vec::new();
    for pos in Pos::iter() {
        let candidates = sudoku.get_candidates_by_pos(pos);
        if candidates.count_ones() == 1 {
            let value = Value::new(candidates.iter().next().unwrap());
            ret.push((pos, value));
        }
    }
    StrategyResult {
        false_candidates: Vec::new(),
        true_candidates: ret
    }
}
