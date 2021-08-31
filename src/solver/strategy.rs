use crate::Pos;
use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    NakedSingle
}

// TODO - return a description of how we decided on the result?
pub(crate) struct StrategyResult {
    /// Any candidates that are found to be false
    pub false_candidates: Vec<(Pos, NonZeroU8)>,
    /// Any candidates that are found to be true
    pub true_candidates: Vec<(Pos, NonZeroU8)>
}

impl StrategyResult {
    pub fn has_changes(&self) -> bool {
        !self.false_candidates.is_empty() || !self.true_candidates.is_empty()
    }
}

impl Default for StrategyResult {
    fn default() -> Self {
        Self {
            false_candidates: Vec::new(),
            true_candidates: Vec::new()
        }
    }
}
