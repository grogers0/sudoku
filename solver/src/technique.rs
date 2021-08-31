use std::num::NonZeroU8;
use sudoku_board::{Pos};

#[derive(Debug, Clone, Copy)]
pub enum Technique {
    NakedSingle
}

pub struct TechniqueResult {
    /// Any candidates that are found to be false
    pub false_candidates: Vec<(Pos, NonZeroU8)>,
    /// Any candidates that are found to be true
    pub true_candidates: Vec<(Pos, NonZeroU8)>
}

impl TechniqueResult {
    pub fn has_changes(&self) -> bool {
        !self.false_candidates.is_empty() || !self.true_candidates.is_empty()
    }
}

impl Default for TechniqueResult {
    fn default() -> TechniqueResult {
        TechniqueResult {
            false_candidates: Vec::new(),
            true_candidates: Vec::new()
        }
    }
}
