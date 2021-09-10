use super::{
    strategies::{self, StrategyResult},
    solve, SolveOpts, Strategy,
};
use crate::{Sudoku, Value};

pub(crate) fn check_example(stratfn: fn(&Sudoku) -> Option<StrategyResult>, sudoku_line: &str, expected_res: Option<StrategyResult>) {
    let sudoku = Sudoku::from_line(sudoku_line).unwrap();
    let res = stratfn(&sudoku);
    assert_eq!(res, expected_res);
}

#[allow(dead_code)]
pub(crate) fn solve_expected_step(sudoku: &Sudoku, strategies: &[Strategy]) -> StrategyResult {
    let opts = SolveOpts {
        strategies: strategies,
        guess_and_check: false,
        stop_after_first_step: true,
    };
    solve(sudoku.clone(), &opts).steps.into_iter().next().unwrap()
}

pub(crate) fn check_pattern_overlay_equivalence(sudoku: &Sudoku, val: Value, step_result: StrategyResult) {
    let res = strategies::pattern_overlay_for_value(&sudoku, val).unwrap();
    // Pattern overlay should be at least as powerful as any other step for single values
    for cand@(_, _) in step_result.excluded_candidates() {
        if !res.excluded_candidates().contains(&cand) {
            panic!("Expected excluded candidate {:?} to be present in pattern overlay result but wasn't, pattern overlay returned: {:?}, sudoku:\n{:?}", cand, res, sudoku);
        }
    }
    for cand@(_, _) in step_result.required_candidates() {
        if !res.required_candidates().contains(&cand) {
            panic!("Expected required candidate {:?} to be present in pattern overlay result but wasn't, pattern overlay returned: {:?}, sudoku:\n{:?}", cand, res, sudoku);
        }
    }
}


#[cfg(feature = "generator")]
// NOTE - this takes a lot of time for some strategies! Also, use `cargo test --release`
#[allow(dead_code)]
pub(crate) fn generate_example(
    required_next_strats: &[Strategy],
    allowed_before_strats: Option<&[Strategy]>,
    disallowed_next_strats: Option<&[Strategy]>) -> Sudoku
{
    use crate::generator::generate;

    let disallowed_next_strats: Vec<_> = match disallowed_next_strats {
        Some(strats) => strats,
        None => strategies::ALL
    }.iter().cloned().filter(|strat| !required_next_strats.contains(strat)).collect();
    let allowed_before_strats: Vec<_> = match allowed_before_strats {
        Some(strats) => strats,
        None => &[Strategy::NakedSingle, Strategy::HiddenSingle], // Only singles so we can use the line format instead of pencilmarks format to store the example
    }.iter().cloned().filter(|strat| !required_next_strats.contains(strat)).collect();

    let allowed_before_opts = SolveOpts {
        strategies: &allowed_before_strats,
        guess_and_check: false,
        stop_after_first_step: false,
    };
    let required_next_opts = SolveOpts {
        strategies: required_next_strats,
        guess_and_check: false,
        stop_after_first_step: true,
    };
    let disallowed_next_opts = SolveOpts {
        strategies: &disallowed_next_strats,
        guess_and_check: false,
        stop_after_first_step: true,
    };
    loop {
        let sudoku = generate(Default::default());
        let sudoku = solve(sudoku, &allowed_before_opts).sudoku;
        if !solve(sudoku.clone(), &required_next_opts).steps.is_empty() &&
            solve(sudoku.clone(), &disallowed_next_opts).steps.is_empty()
        {
            return sudoku;
        }
    }
}
