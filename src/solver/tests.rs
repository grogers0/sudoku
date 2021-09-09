use super::strategies::StrategyResult;
use crate::Sudoku;

pub(crate) fn check_example(stratfn: fn(&Sudoku) -> Option<StrategyResult>, sudoku_line: &str, expected_res: Option<StrategyResult>) {
    let sudoku = Sudoku::from_line(sudoku_line).unwrap();
    let res = stratfn(&sudoku);
    assert_eq!(res, expected_res);
}
