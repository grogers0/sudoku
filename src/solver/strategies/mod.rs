use crate::{
    Pos, Value,
    solver::{Line, House, Block},
};

mod guess_and_check;
mod hidden_single;
mod locked_candidate;
mod naked_single;
mod naked_subset;

pub(crate) use guess_and_check::guess_and_check;
pub(crate) use hidden_single::hidden_single;
pub(crate) use locked_candidate::locked_candidate;
pub(crate) use naked_single::naked_single;
pub(crate) use naked_subset::{naked_pair, naked_triple, naked_quad};

#[derive(Debug, Clone)]
pub enum Strategy {
    NakedSingle,
    NakedPair,
    NakedTriple,
    NakedQuad,
    HiddenSingle,
    LockedCandidate,
}

// TODO - benchmark and figure out which is the fastest order and which are worthwhile
pub const FAST: &'static [Strategy] = &[
    Strategy::NakedSingle,
    Strategy::HiddenSingle,
    Strategy::LockedCandidate,
    Strategy::NakedPair,
    Strategy::NakedTriple,
    Strategy::NakedQuad,
];

pub const ALL: &'static [Strategy] = &[
    Strategy::NakedSingle,
    Strategy::HiddenSingle,
    Strategy::LockedCandidate,
    Strategy::NakedPair,
    Strategy::NakedTriple,
    Strategy::NakedQuad,
];

// TODO - return a description of how we decided on the result?
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StrategyResult {
    NakedSingle(Pos, Value),
    HiddenSingle(Pos, Value, House),
    GuessAndCheck(Pos, Value),
    LockedCandidate {
        value: Value,
        exclusions: Vec<Pos>,
        /// Positions of the locked candidates
        positions: Vec<Pos>,
        block: Block,
        line: Line,
        /// Type 1 is "pointing" - candidates in the rest of the block are missing, so exclude the
        /// line. Type 2 is "claiming" - candidates in the rest of the line are missing.
        pointing: bool
    },
    NakedSubset {
        exclusions: Vec<(Pos, Value)>,
        /// Positions of the cells in the subset
        positions: Vec<Pos>,
        values: Vec<Value>
    }
}

impl StrategyResult {
    pub(crate) fn candidates_to_remove(&self) -> Vec<(Pos, Value)> {
        match self {
            StrategyResult::NakedSingle(_, _) => Vec::new(),
            StrategyResult::HiddenSingle(_, _, _) => Vec::new(),
            StrategyResult::GuessAndCheck(_, _) => Vec::new(),
            StrategyResult::LockedCandidate { value, exclusions, .. } => exclusions.iter().map(|pos| (*pos, *value)).collect(),
            StrategyResult::NakedSubset { exclusions, .. } => exclusions.clone()
        }
    }

    pub(crate) fn candidates_to_set(&self) -> Vec<(Pos, Value)> {
        match self {
            StrategyResult::NakedSingle(pos, val) => vec![(*pos, *val)],
            StrategyResult::HiddenSingle(pos, val, _) => vec![(*pos, *val)],
            StrategyResult::GuessAndCheck(_, _) => Vec::new(), // Handled separately when solving, not as a normal strategy
            _ => Vec::new()
        }
    }
}
