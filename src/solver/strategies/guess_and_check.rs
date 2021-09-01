use crate::{
    solver::{solve, SolveResult, SolveOpts},
    Pos, Sudoku, Value,
};

pub(crate) fn guess_and_check(sudoku: &Sudoku) -> SolveResult {
    let pos = Pos::iter()
        .filter(|&pos| sudoku.get_value(pos).is_none())
        .min_by_key(|&pos| sudoku.get_candidates_by_pos(pos).count_ones())
        .unwrap();
    let mut res = SolveResult::Unsolvable(sudoku.clone());
    for value_idx in sudoku.get_candidates_by_pos(pos).iter() {
        let mut sudoku2 = sudoku.clone();
        sudoku2.set_value(pos, Value::new(value_idx));
        res = res.merge(solve(sudoku2, &SolveOpts::fast()));
        if let already_non_unique@SolveResult::NonUnique(_, _) = res {
            return already_non_unique
        }
    }
    res
}
