use crate::solver::{solve, SolveResult, SolveOpts};
use std::num::NonZeroU8;
use sudoku_board::{Pos, Sudoku};

pub(crate) fn guess_and_check(sudoku: &Sudoku) -> SolveResult {
    let pos = Pos::iter()
        .filter(|&pos| sudoku.get_value(pos).is_none())
        .min_by_key(|&pos| sudoku.get_candidates_by_pos(pos).count_ones())
        .unwrap();
    let mut res = SolveResult::Unsolvable(sudoku.clone());
    for value_idx in sudoku.get_candidates_by_pos(pos).iter() {
        let mut sudoku2 = sudoku.clone();
        sudoku2.set_value(pos, NonZeroU8::new(value_idx as u8 + 1).unwrap());
        println!("set pos: {}, value: {}", pos.idx(), value_idx + 1);
        println!("{:?}", sudoku2);
        res = res.merge(solve(sudoku2, SolveOpts::fast()));
        if let already_non_unique@SolveResult::NonUnique(_, _) = res {
            return already_non_unique
        }
    }
    res
}
