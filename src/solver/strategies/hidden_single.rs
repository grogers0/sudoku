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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        solver::tests::check_example,
        Pos,
    };

    #[test]
    fn test_block_example1() {
        check_example(hidden_single,
            "47..8........5...4.2..6.3...8.5.....1...9..6....7....121....85.9......13.48......",
            Some(StrategyResult::HiddenSingle(Pos::new(69), Value::new(3), House::new(26))));
    }

    #[test]
    fn test_row_example1() {
        check_example(hidden_single,
            "5...18.......6.9....7...14.62.8....1....2......9...42..1.3.9............84.57.6.9",
            Some(StrategyResult::HiddenSingle(Pos::new(56), Value::new(5), House::new(6))));
    }

    #[test]
    fn test_col_example1() {
        check_example(hidden_single,
            "....53..2.....9..863.........716...........9.25...7..47..3.8.5..4.5.612.5..2.....",
            Some(StrategyResult::HiddenSingle(Pos::new(48), Value::new(8), House::new(12))));
    }
}
