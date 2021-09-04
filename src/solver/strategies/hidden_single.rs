use crate::{
    solver::{
        strategy::StrategyResult,
        House,
    },
    pos::PosBitSet,
    Sudoku, Value,
};

pub(crate) fn hidden_single(sudoku: &Sudoku) -> StrategyResult {
    let mut ret = Vec::new();
    let mut already_chosen = PosBitSet::NONE;
    for val in Value::iter() {
        let all_candidates = sudoku.get_candidates_by_value(val);
        if all_candidates.is_empty() { continue } // Fast path for fully solved values
        let mut deduced = PosBitSet::NONE;
        for house in House::iter() {
            let candidates = all_candidates & house.members_bitset();
            if candidates.len() == 1 {
                deduced |= candidates;
            }
        }
        for pos in deduced.iter() {
            // When solving an unsolvable sudoku, we may choose two values for the same position,
            // just pick one of them
            if !already_chosen.contains(pos) {
                already_chosen.insert(pos);
                ret.push((pos, val));
            }
        }
    }
    StrategyResult {
        false_candidates: Vec::new(),
        true_candidates: ret
    }
}
