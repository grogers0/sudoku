use super::StrategyResult;
use crate::{
    solver::House,
    Sudoku, Value,
};

pub(crate) fn hidden_single(sudoku: &Sudoku) -> Option<StrategyResult> {
    for val in Value::iter() {
        let all_candidates = sudoku.get_candidates_by_value(val);
        if all_candidates.is_empty() { continue }

        for house in House::iter() {
            let candidates = all_candidates & house.members_bitset();
            if candidates.len() == 1 {
                let pos =  candidates.iter().next().unwrap();
                return Some(StrategyResult::HiddenSingle(pos, val, house));
            }
        }
    }
    None
}
