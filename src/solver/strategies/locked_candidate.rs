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
                        excluded_positions: line_candidates.difference(intersection_candidates).iter().collect(),
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
                        excluded_positions: block_candidates.difference(intersection_candidates).iter().collect(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        solver::{
            tests::{check_example, check_pattern_overlay_equivalence},
            Line,
        },
        Pos,
    };

    #[test]
    fn test_pointing_example1() {
        let line = "1.....863.6.13..9..3...6.....1.6....2.35.....6..3127453..25...1.1..432..82..71...";
        let step_res = StrategyResult::LockedCandidate {
            value: Value::new(7), excluded_positions: vec![Pos::new(47)],
            positions: vec![Pos::new(11), Pos::new(20)], block: Block::new(0), line: Line::new(11),
            pointing: true,
        };
        check_example(locked_candidate, line, Some(step_res.clone()));
        check_pattern_overlay_equivalence(&Sudoku::from_line(line).unwrap(), Value::new(7), step_res);
    }

    #[test]
    fn test_claiming_example1() {
        let line = ".1...584.4.81.6.....2..8..12976814..865..91721..7529683..8...1...9.1..8.781.63..4";
        let step_res = StrategyResult::LockedCandidate {
            value: Value::new(1), excluded_positions: vec![Pos::new(13)],
            positions: vec![Pos::new(3), Pos::new(4)], block: Block::new(1), line: Line::new(0),
            pointing: false
        };
        check_example(locked_candidate, line, Some(step_res.clone()));
        check_pattern_overlay_equivalence(&Sudoku::from_line(line).unwrap(), Value::new(1), step_res);
    }

}
