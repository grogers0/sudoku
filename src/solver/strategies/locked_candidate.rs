use crate::{
    solver::{
        strategy::StrategyResult,
        Block,
    },
    pos::PosBitSet,
    Sudoku, Value,
};

pub(crate) fn locked_candidate(sudoku: &Sudoku) -> StrategyResult {
    let mut ret = Vec::new();
    for val in Value::iter() {
        let all_candidates = sudoku.get_candidates_by_value(val);
        if all_candidates.is_empty() { continue } // Fast path for fully solved values
        let mut eliminations = PosBitSet::NONE;

        for block in Block::iter() {
            let block_candidates = all_candidates & block.members_bitset();
            if block_candidates.is_empty() { continue }
            for line in block.intersecting_lines_iter() {
                let line_candidates = all_candidates & line.members_bitset();
                let intersection_candidates = block_candidates & line_candidates;
                if intersection_candidates.is_empty() { continue }

                // Type 1
                if block_candidates.difference(intersection_candidates).is_empty() {
                    eliminations |= line_candidates.difference(intersection_candidates);
                }

                // Type 2
                if line_candidates.difference(intersection_candidates).is_empty() {
                    eliminations |= block_candidates.difference(intersection_candidates);
                }
            }
        }
        for pos in eliminations.iter() {
            ret.push((pos, val));
        }
    }

    StrategyResult {
        false_candidates: ret,
        true_candidates: Vec::new()
    }
}
