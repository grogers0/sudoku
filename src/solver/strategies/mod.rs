use crate::{
    solver::{Line, House, Block, PosBitSet, ValueBitSet},
    Pos, Value,
};

mod guess_and_check;
mod hidden_single;
mod hidden_subset;
mod locked_candidate;
mod naked_single;
mod naked_subset;

pub(crate) use guess_and_check::guess_and_check;
pub(crate) use hidden_single::hidden_single;
pub(crate) use hidden_subset::{hidden_pair, hidden_triple, hidden_quadruple};
pub(crate) use locked_candidate::locked_candidate;
pub(crate) use naked_single::naked_single;
pub(crate) use naked_subset::{naked_pair, naked_triple, naked_quadruple};

#[derive(Debug, Copy, Clone)]
pub enum Strategy {
    HiddenPair,
    HiddenQuadruple,
    HiddenSingle,
    HiddenTriple,
    LockedCandidate,
    NakedPair,
    NakedQuadruple,
    NakedSingle,
    NakedTriple,
}

// TODO - benchmark and figure out which is the fastest order and which are worthwhile
pub const FAST: &'static [Strategy] = &[
    Strategy::NakedSingle,
    Strategy::HiddenSingle,
    Strategy::LockedCandidate,
    Strategy::NakedPair,
    Strategy::NakedTriple,
    Strategy::NakedQuadruple,
    //Strategy::HiddenPair,
    //Strategy::HiddenTriple,
    //Strategy::HiddenQuadruple,
];

pub const ALL: &'static [Strategy] = &[
    Strategy::NakedSingle,
    Strategy::HiddenSingle,
    Strategy::LockedCandidate,
    Strategy::NakedPair,
    Strategy::NakedTriple,
    Strategy::NakedQuadruple,
    Strategy::HiddenPair,
    Strategy::HiddenTriple,
    Strategy::HiddenQuadruple,
];

// TODO - return a description of how we decided on the result?
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum StrategyResult {
    NakedSingle(Pos, Value),
    HiddenSingle(Pos, Value, House),
    GuessAndCheck(Pos, Value),
    LockedCandidate {
        value: Value,
        excluded_positions: Vec<Pos>,
        /// Positions of the locked candidates
        positions: Vec<Pos>,
        block: Block,
        line: Line,
        /// Type 1 is "pointing" - candidates in the rest of the block are missing, so exclude the
        /// line. Type 2 is "claiming" - candidates in the rest of the line are missing.
        pointing: bool
    },
    NakedSubset {
        excluded_candidates: Vec<(Pos, Value)>,
        /// Positions of the cells in the subset
        positions: Vec<Pos>,
        values: Vec<Value>
    },
    HiddenSubset {
        excluded_candidates: Vec<(Pos, Value)>,
        /// Positions of the cells in the subset
        positions: Vec<Pos>,
        values: Vec<Value>,
        house: House
    }
}

impl StrategyResult {
    pub(crate) fn candidates_to_remove(&self) -> Vec<(Pos, Value)> {
        match self {
            StrategyResult::NakedSingle(_, _) => Vec::new(),
            StrategyResult::HiddenSingle(_, _, _) => Vec::new(),
            StrategyResult::GuessAndCheck(_, _) => Vec::new(),
            StrategyResult::LockedCandidate { value, excluded_positions, .. } =>
                excluded_positions.iter().map(|pos| (*pos, *value)).collect(),
            StrategyResult::NakedSubset { excluded_candidates, .. } => excluded_candidates.clone(),
            StrategyResult::HiddenSubset { excluded_candidates, .. } => excluded_candidates.clone()
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

/// A naked subset in some house also means there can't be a higher-order naked subset with any of
/// the same positions in that house, or a hidden subset with any of the locked values (and vice
/// versa). By keeping track of these known subsets, we skip searching those locations on
/// subsequent passes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct KnownSubsets {
    naked: PosBitSet,
    hidden: ValueBitSet
}

impl Default for KnownSubsets {
    fn default() -> Self {
        Self {
            naked: PosBitSet::NONE,
            hidden: ValueBitSet::NONE
        }
    }
}
