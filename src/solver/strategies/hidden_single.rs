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
        let mut deductions = PosBitSet::NONE;

        for house in House::iter() {
            let candidates = all_candidates & house.members_bitset();
            if candidates.len() == 1 {
                deductions |= candidates;
            }
        }

        // When solving an unsolvable sudoku, we may choose two different values for the same
        // position, just pick one of them and figure out later that it's unsolvable
        for pos in deductions.difference(already_chosen).iter() {
            ret.push((pos, val));
        }
        already_chosen |= deductions;
    }
    StrategyResult {
        false_candidates: Vec::new(),
        true_candidates: ret
    }
}
