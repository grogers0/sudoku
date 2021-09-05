use super::StrategyResult;
use crate::{
    solver::Block,
    Sudoku, Value,
};

pub(crate) fn locked_candidate(sudoku: &Sudoku) -> Option<StrategyResult> {
    for val in Value::iter() {
        let all_candidates = sudoku.get_candidates_by_value(val);
        if all_candidates.is_empty() { continue }

        for block in Block::iter() {
            let block_candidates = all_candidates & block.members_bitset();
            if block_candidates.is_empty() { continue }
            for line in block.intersecting_lines_iter() {
                let line_candidates = all_candidates & line.members_bitset();
                let intersection_candidates = block_candidates & line_candidates;
                if intersection_candidates.is_empty() { continue }

                // Type 1 - pointing
                if block_candidates.difference(intersection_candidates).is_empty() &&
                    !line_candidates.difference(intersection_candidates).is_empty()
                {
                    return Some(StrategyResult::LockedCandidate {
                        value: val,
                        exclusions: line_candidates.difference(intersection_candidates).iter().collect(),
                        positions: intersection_candidates.iter().collect(),
                        block, line,
                        pointing: true
                    });
                }

                // Type 2 - claiming
                if line_candidates.difference(intersection_candidates).is_empty() &&
                    !block_candidates.difference(intersection_candidates).is_empty()
                {
                    return Some(StrategyResult::LockedCandidate {
                        value: val,
                        exclusions: block_candidates.difference(intersection_candidates).iter().collect(),
                        positions: intersection_candidates.iter().collect(),
                        block, line,
                        pointing: false
                    });
                }
            }
        }
    }
    None
}
