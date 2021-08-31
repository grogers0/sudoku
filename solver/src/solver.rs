use crate::{
    technique::{Technique, TechniqueResult},
    techniques,
};
use sudoku_board::Sudoku;

pub struct SolveOpts {
    /// Techniques to try when solving, in order
    techniques: Vec<Technique>,
    /// If sudoku is unsolvable with given techniques, should we guess and check to solve it
    guess_and_check: bool
}

impl Default for SolveOpts {
    fn default() -> Self {
        SolveOpts {
            techniques: techniques::ALL.iter().cloned().collect::<Vec<Technique>>(),
            guess_and_check: true
        }
    }
}

impl SolveOpts {
    pub fn fast() -> SolveOpts {
        SolveOpts {
            techniques: techniques::FAST.iter().cloned().collect::<Vec<Technique>>(),
            guess_and_check: true
        }
    }
}

#[derive(Debug, Clone)]
pub enum SolveResult {
    Unsolvable(Sudoku), // The sudoku as far as we were able to solve it
    Unique(Sudoku),
    NonUnique(Sudoku, Sudoku)
}

impl SolveResult {
    pub(crate) fn merge(self, other: SolveResult) -> SolveResult {
        match (self, other) {
            (SolveResult::Unsolvable(s), SolveResult::Unsolvable(_)) => SolveResult::Unsolvable(s),
            (SolveResult::Unsolvable(_), solvable_result) => solvable_result,
            (SolveResult::Unique(s), SolveResult::Unsolvable(_)) => SolveResult::Unique(s),
            (SolveResult::Unique(s1), SolveResult::Unique(s2)) => SolveResult::NonUnique(s1, s2),
            (SolveResult::Unique(s1), SolveResult::NonUnique(s2, _)) => SolveResult::NonUnique(s1, s2),
            (SolveResult::NonUnique(s1, s2), _) => SolveResult::NonUnique(s1, s2)
        }
    }
}

struct SolveState {
    unique: Option<bool>,
    unique_solution: Option<Sudoku>
}

fn run_techniques(sudoku: &Sudoku, opts: &SolveOpts) -> TechniqueResult {
    for technique in &opts.techniques {
        let res = match technique {
            Technique::NakedSingle => techniques::naked_single(&sudoku)
        };
        if res.has_changes() { return res }
    }
    Default::default()
}

pub fn solve(mut sudoku: Sudoku, opts: SolveOpts) -> SolveResult {
    //let mut state = SolveState {
    //    unique: None,
    //    unique_solution: None
    //};
    while sudoku.progress_possible() {
        let res = run_techniques(&sudoku, &opts);
        if !res.has_changes() { break }
        for (pos, value) in res.false_candidates {
            sudoku.remove_candidate(pos, value);
        }
        for (pos, value) in res.true_candidates {
            sudoku.set_value(pos, value);
        }
    }

    if sudoku.is_solved() {
        return SolveResult::Unique(sudoku)
    }
    if opts.guess_and_check && sudoku.progress_possible() {
        return techniques::guess_and_check(&sudoku);
    }
    SolveResult::Unsolvable(sudoku)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_unique() {
        let line = "4...3.......6..8..........1....5..9..8....6...7.2........1.27..5.3....4.9........";
        let sudoku = Sudoku::from_line(line).unwrap();
        let solve_res = solve(sudoku, Default::default());
        match solve_res {
            SolveResult::Unique(s) => {
                println!("{}", s.to_line());
            },
            SolveResult::Unsolvable(s) => {
                println!("{}", s.to_line());
                println!("{:?}", s);
                panic!();
            },
            SolveResult::NonUnique(s1, s2) => {
                println!("{}", s1.to_line());
                println!("{}", s2.to_line());
                println!("{:?}", s1);
                panic!();
            }
        }
    }
}
