use crate::{
    solver::{solve, SolveOpts, SolveResult, Row, Col, Block},
    Pos, Sudoku,
};
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
            solve_opts: Default::default(),
            rng: Box::new(thread_rng())
        }
    }
}

fn fill_initial_chunk(sudoku: &mut Sudoku, rng: &mut dyn RngCore, positions: &[Pos]) {
    let mut candidates: Vec<_> = sudoku.get_candidates_by_pos(positions[0]).iter().collect();
    assert!(candidates.len() >= positions.len());
    candidates.partial_shuffle(rng, positions.len());
    for (&pos, val) in positions.into_iter().zip(candidates) {
        sudoku.set_value(pos, val);
    }
}

// The initial chunks can be filled without backtracking, see "Optimization" in
// https://dlbeer.co.nz/articles/sudoku.html
fn fill_initial_chunks(sudoku: &mut Sudoku, rng: &mut dyn RngCore) {
    fill_initial_chunk(sudoku, rng, &Block::new(0).members_iter().collect::<Vec<_>>());
    fill_initial_chunk(sudoku, rng, &Row::new(0).members_iter().skip(3).collect::<Vec<_>>());
    fill_initial_chunk(sudoku, rng, &Col::new(0).members_iter().skip(3).collect::<Vec<_>>());
    // I don't understand how to pick the right candidates for the remaining part of the top band
    // without backtracking, the above link doesn't really explain it either...
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
        .min_by_key(|&pos| sudoku.get_candidates_by_pos(pos).len())
        .unwrap();
    let mut candidates: Vec<_> = sudoku.get_candidates_by_pos(pos).iter().collect();
    candidates.shuffle(rng);
    for val in candidates {
        let mut sudoku2 = sudoku.clone();
        sudoku2.set_value(pos, val);
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
