use crate::{
    solver::{solve, SolveOpts, SolveResult},
    Pos, Sudoku
};
use std::num::NonZeroU8;
use rand::{
    seq::SliceRandom,
    thread_rng, RngCore,
};

// FIXME - add symmetry
pub struct GenerateOpts<'a> {
    pub solve_opts: SolveOpts<'a>,
    pub rng: Box<dyn RngCore>
}

impl Default for GenerateOpts<'_> {
    #[inline]
    fn default() -> Self {
        Self {
            solve_opts: SolveOpts::fast(),
            rng: Box::new(thread_rng())
        }
    }
}

fn fill_initial_chunk(sudoku: &mut Sudoku, rng: &mut dyn RngCore, positions: &[Pos]) {
    let mut candidates: Vec<_> = sudoku.get_candidates_by_pos(positions[0]).iter()
        .map(|val_idx| NonZeroU8::new(val_idx as u8 + 1).unwrap())
        .collect();
    assert!(candidates.len() >= positions.len());
    candidates.partial_shuffle(rng, positions.len());
    for (&pos, value) in positions.into_iter().zip(candidates) {
        sudoku.set_value(pos, value);
    }
}

// The initial chunks can be filled without backtracking, see "Optimization" in
// https://dlbeer.co.nz/articles/sudoku.html
fn fill_initial_chunks(sudoku: &mut Sudoku, rng: &mut dyn RngCore) {
    // FIXME - get these with better constants

    fill_initial_chunk(sudoku, rng, &[
        Pos::row_col(0, 0), Pos::row_col(0, 1), Pos::row_col(0, 2),
        Pos::row_col(1, 0), Pos::row_col(1, 1), Pos::row_col(1, 2),
        Pos::row_col(2, 0), Pos::row_col(2, 1), Pos::row_col(2, 2),
    ]);

    fill_initial_chunk(sudoku, rng, &[
        Pos::row_col(3, 0), Pos::row_col(4, 0), Pos::row_col(5, 0),
        Pos::row_col(6, 0), Pos::row_col(7, 0), Pos::row_col(8, 0),
    ]);

    fill_initial_chunk(sudoku, rng, &[
        Pos::row_col(0, 3), Pos::row_col(0, 4), Pos::row_col(0, 5),
        Pos::row_col(0, 6), Pos::row_col(0, 7), Pos::row_col(0, 8),
    ]);

    // I don't understand how to pick the right candidates for the remaining part of the top band
    // without backtracking, the above link doesn't really explain it well, so just leave it there.
}

fn random_guess_and_check_to_fill(sudoku: Sudoku, rng: &mut dyn RngCore) -> Option<Sudoku> {
    let mut no_guess_and_check = SolveOpts::fast();
    no_guess_and_check.guess_and_check = false;
    let sudoku = match solve(sudoku, &no_guess_and_check) {
        SolveResult::Unsolvable(s) => s,
        SolveResult::Unique(s) => return Some(s),
        SolveResult::NonUnique(_, _) => unreachable!()
    };

    if !sudoku.progress_possible() {
        return None
    }

    // I don't think it matters if the position we choose is random, since the candidate is
    let pos = Pos::iter()
        .filter(|&pos| sudoku.get_value(pos).is_none())
        .min_by_key(|&pos| sudoku.get_candidates_by_pos(pos).count_ones())
        .unwrap();
    let mut candidates: Vec<_> = sudoku.get_candidates_by_pos(pos).iter()
        .map(|value_idx| NonZeroU8::new(value_idx as u8 + 1).unwrap())
        .collect();
    candidates.shuffle(rng);
    for value in candidates {
        let mut sudoku2 = sudoku.clone();
        sudoku2.set_value(pos, value);
        match random_guess_and_check_to_fill(sudoku2, rng) {
            Some(s) => return Some(s),
            None => () // Try next candidate as this was not solvable
        }
    }
    None
}

fn generate_solved(rng: &mut dyn RngCore) -> Sudoku {
    let mut sudoku = Sudoku::new();
    fill_initial_chunks(&mut sudoku, rng);
    random_guess_and_check_to_fill(sudoku, rng).unwrap()
}

fn sudoku_without_given(sudoku: &Sudoku, removed_pos: Pos) -> Sudoku {
    let mut givens = Vec::new();
    for pos in Pos::iter() {
        if pos == removed_pos { continue }
        match sudoku.get_value(pos) {
            Some(val) => givens.push((pos, val)),
            None => ()
        }
    }
    let mut sudoku = Sudoku::new();
    for (pos, val) in givens {
        sudoku.set_value(pos, val);
    }
    sudoku
}

pub fn generate(mut opts: GenerateOpts) -> Sudoku {
    let mut sudoku = generate_solved(&mut opts.rng);

    // Remove random givens as long as the sudoku stays uniquely solvable
    let mut positions: Vec<_> = Pos::iter().collect();
    positions.shuffle(&mut opts.rng);
    while let Some(pos) = positions.pop() {
        if sudoku.get_value(pos).is_none() { continue }
        let sudoku2 = sudoku_without_given(&sudoku, pos);
        match solve(sudoku2.clone(), &opts.solve_opts) {
            SolveResult::NonUnique(_, _) => (),
            SolveResult::Unique(_) => sudoku = sudoku2,
            SolveResult::Unsolvable(_) => unreachable!()
        };
    }
    sudoku
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let sudoku = generate(Default::default());
        assert!(!Pos::iter().all(|pos| sudoku.get_value(pos).is_some()));
        assert!(solve(sudoku, &SolveOpts::fast()).is_unique());
    }
}
