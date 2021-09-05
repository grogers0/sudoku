use crate::{
    solver::{
        solve, SolveResult, SolveSuccess, SolveOpts,
        strategies::StrategyResult,
    },
    Pos, Sudoku,
};

pub(crate) fn guess_and_check(sudoku: &Sudoku) -> SolveResult {
    let pos = Pos::iter()
        .filter(|&pos| sudoku.get_value(pos).is_none())
        .min_by_key(|&pos| sudoku.get_candidates_by_pos(pos).len())
        .unwrap();
    let mut res = SolveResult { success: SolveSuccess::Unsolvable, sudoku: sudoku.clone(), steps: Vec::new() };
    for val in sudoku.get_candidates_by_pos(pos).iter() {
        let mut sudoku2 = sudoku.clone();
        sudoku2.set_value(pos, val);
        let mut res2 = solve(sudoku2, &SolveOpts::fast());
        res2.steps.insert(0, StrategyResult::GuessAndCheck(pos, val));
        res = res.merge(res2);
        if let SolveSuccess::NonUnique = res.success {
            return res
        }
    }
    res
}
