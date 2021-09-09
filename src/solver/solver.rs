use crate::{
    solver::{
        strategies::{
            self, Coloring, Strategy, StrategyResult, KnownSubsets,
        },
        house::HouseIndexedSlice,
        House, ValueIndexedSlice,
    },
    Sudoku, Value,
};

pub struct SolveOpts<'a> {
    /// Strategies to try when solving, in order
    pub strategies: &'a [Strategy],
    /// If sudoku is unsolvable with given strategies, should we guess and check to solve it
    pub guess_and_check: bool
}

impl Default for SolveOpts<'_> {
    fn default() -> Self {
        Self {
            strategies: &strategies::ALL,
            guess_and_check: true
        }
    }
}

impl SolveOpts<'_> {
    pub fn fast() -> Self {
        Self {
            strategies: &strategies::FAST,
            guess_and_check: true
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SolveSuccess {
    Unsolvable,
    Unique,
    NonUnique
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SolveResult {
    pub success: SolveSuccess,
    /// If the sudoku was solvable, one of those solutions, otherwise the sudoku as far as we could
    /// solve it.
    pub sudoku: Sudoku,
    pub(crate) steps: Vec<StrategyResult> // TODO how to expose House, etc?
}

impl SolveResult {
    #[inline]
    pub fn is_unsolvable(&self) -> bool {
        matches!(self.success, SolveSuccess::Unsolvable)
    }

    #[inline]
    pub fn is_unique(&self) -> bool {
        matches!(self.success, SolveSuccess::Unique)
    }

    #[inline]
    pub fn is_non_unique(&self) -> bool {
        matches!(self.success, SolveSuccess::NonUnique)
    }

    pub(crate) fn merge(self, other: SolveResult) -> SolveResult {
        match (self, other) {
            (lhs@SolveResult { success: SolveSuccess::Unsolvable, sudoku: _, steps: _ },
             SolveResult { success: SolveSuccess::Unsolvable, .. }) => lhs,
            (SolveResult { success: SolveSuccess::Unsolvable, .. }, solvable_result) => solvable_result,
            (lhs@SolveResult { success: SolveSuccess::Unique, sudoku: _, steps: _ },
             SolveResult { success: SolveSuccess::Unsolvable, .. }) => lhs,
            (SolveResult { success: SolveSuccess::Unique, sudoku, steps }, _also_solvable) => {
                SolveResult { success: SolveSuccess::NonUnique, sudoku, steps }
            },
            (lhs@SolveResult { success: SolveSuccess::NonUnique, sudoku: _, steps: _ }, _) => lhs
        }
    }
}

fn run_strategies(sudoku: &Sudoku, opts: &SolveOpts) -> Option<StrategyResult> {
    struct TmpSolveState {
        known_subsets: HouseIndexedSlice<KnownSubsets>,
        colorings: ValueIndexedSlice<Option<Coloring>>
    }
    const NONE_COLORING: Option<Coloring> = None;
    let mut tmp_solve_state = TmpSolveState {
        known_subsets: HouseIndexedSlice::from_slice([Default::default(); House::N]),
        colorings: ValueIndexedSlice::from_slice([NONE_COLORING; Value::N]),
    };
    for strat in opts.strategies {
        let res = match strat {
            Strategy::HiddenPair => strategies::hidden_pair(&sudoku, &mut tmp_solve_state.known_subsets),
            Strategy::HiddenQuadruple => strategies::hidden_quadruple(&sudoku, &mut tmp_solve_state.known_subsets),
            Strategy::HiddenSingle => strategies::hidden_single(&sudoku),
            Strategy::HiddenTriple => strategies::hidden_triple(&sudoku, &mut tmp_solve_state.known_subsets),
            Strategy::LockedCandidate => strategies::locked_candidate(&sudoku),
            Strategy::MultiColor(max_color_pairs) => strategies::multi_color(&sudoku, *max_color_pairs, &mut tmp_solve_state.colorings),
            Strategy::NakedPair => strategies::naked_pair(&sudoku, &mut tmp_solve_state.known_subsets),
            Strategy::NakedQuadruple => strategies::naked_quadruple(&sudoku, &mut tmp_solve_state.known_subsets),
            Strategy::NakedSingle => strategies::naked_single(&sudoku),
            Strategy::NakedTriple => strategies::naked_triple(&sudoku, &mut tmp_solve_state.known_subsets),
            Strategy::SimpleColor => strategies::simple_color(&sudoku, &mut tmp_solve_state.colorings),
            Strategy::WxyzWing => strategies::wxyz_wing(&sudoku),
            Strategy::XyWing => strategies::xy_wing(&sudoku),
            Strategy::XyzWing => strategies::xyz_wing(&sudoku),
        };
        if res.is_some() { return res }
    }
    None
}

pub fn solve(mut sudoku: Sudoku, opts: &SolveOpts) -> SolveResult {
    let mut steps = Vec::new();
    while sudoku.progress_possible() {
        match run_strategies(&sudoku, &opts) {
            None => break, // No further progress unless we guess and check
            Some(res) => {
                for (pos, val) in res.candidates_to_remove() {
                    sudoku.remove_candidate(pos, val);
                }
                for (pos, val) in res.candidates_to_set() {
                    sudoku.set_value(pos, val);
                }
                steps.push(res);
            }
        }
    }

    if sudoku.is_solved() {
        return SolveResult { success: SolveSuccess::Unique, sudoku, steps }
    }
    if opts.guess_and_check && sudoku.progress_possible() {
        return strategies::guess_and_check(&sudoku, steps);
    }
    SolveResult { success: SolveSuccess::Unsolvable, sudoku, steps }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_unique() {
        let line = "4...3.......6..8..........1....5..9..8....6...7.2........1.27..5.3....4.9........";
        let sudoku = Sudoku::from_line(line).unwrap();
        let solve_res = solve(sudoku, &Default::default());
        let expected_sudoku = Sudoku::from_line("468931527751624839392578461134756298289413675675289314846192753513867942927345186").unwrap();
        assert!(matches!(solve_res.success, SolveSuccess::Unique));
        assert_eq!(solve_res.sudoku, expected_sudoku);
    }
}
