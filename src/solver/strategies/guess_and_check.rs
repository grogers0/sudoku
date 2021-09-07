use crate::{
    solver::{
        solve, SolveResult, SolveSuccess, SolveOpts,
        strategies::StrategyResult,
    },
    Pos, Sudoku,
};

pub(crate) fn guess_and_check(sudoku: &Sudoku, mut initial_steps: Vec<StrategyResult>) -> SolveResult {
    let pos = Pos::iter()
        .filter(|&pos| sudoku.get_value(pos).is_none())
        .min_by_key(|&pos| sudoku.get_candidates_by_pos(pos).len())
        .unwrap();
    let mut res = SolveResult { success: SolveSuccess::Unsolvable, sudoku: sudoku.clone(), steps: Vec::new() };
    for val in sudoku.get_candidates_by_pos(pos).iter() {
        let mut sudoku2 = sudoku.clone();
        sudoku2.set_value(pos, val);
        res = res.merge(solve(sudoku2, &SolveOpts::fast()));
        if let SolveSuccess::NonUnique = res.success {
            initial_steps.push(StrategyResult::GuessAndCheck(pos, val));
            initial_steps.append(&mut res.steps);
            res.steps = initial_steps;
            return res;
        }
    }
    res.steps = initial_steps;
    res
}
