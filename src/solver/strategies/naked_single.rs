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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    fn check_example(sudoku_line: &str, expected_res: Option<StrategyResult>) {
        let sudoku = Sudoku::from_line(sudoku_line).unwrap();
        let res = naked_single(&sudoku);
        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_example1() {
        check_example(".4......32.8......16.....2.8......6.53.1...79..62.9.....48..1...9...1.8.....7.5..",
            Some(StrategyResult::NakedSingle(Pos::new(38), Value::new(1))));
    }

    #[test]
    fn test_example2() {
        check_example("..7...92...581..............5.7..6....3..4..7..8..12.5.4.587..1.7...........2.5..",
            Some(StrategyResult::NakedSingle(Pos::new(60), Value::new(2))));
    }
}
