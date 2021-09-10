use crate::{
    solver::{Line, House, Block, PosBitSet, ValueBitSet},
    Pos, Value,
};

mod coloring;
mod guess_and_check;
mod hidden_single;
mod hidden_subset;
mod locked_candidate;
mod naked_single;
mod naked_subset;
mod pattern_overlay;
mod wings;

pub(crate) use coloring::{multi_color, simple_color, Coloring};
pub(crate) use guess_and_check::guess_and_check;
pub(crate) use hidden_single::hidden_single;
pub(crate) use hidden_subset::{hidden_pair, hidden_triple, hidden_quadruple};
pub(crate) use locked_candidate::locked_candidate;
pub(crate) use naked_single::naked_single;
pub(crate) use naked_subset::{naked_pair, naked_triple, naked_quadruple};
pub(crate) use pattern_overlay::pattern_overlay;
pub(crate) use wings::{xy_wing, xyz_wing, wxyz_wing};

#[cfg(test)]
pub(crate) use pattern_overlay::pattern_overlay_for_value;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    /// The maximum number of pairs of colors to allow in the chain
    MultiColor(usize),
    PatternOverlay,
    SimpleColor,
    XyWing,
    XyzWing,
    WxyzWing,
}

// TODO - benchmark and figure out which is the fastest order and which are worthwhile
pub const FAST: &'static [Strategy] = &[
    Strategy::NakedSingle,
    Strategy::HiddenSingle,
    Strategy::LockedCandidate,
    Strategy::NakedPair,
    Strategy::NakedTriple,
    Strategy::NakedQuadruple,
];

pub const ALL: &'static [Strategy] = &[
    Strategy::NakedSingle,
    Strategy::HiddenSingle,
    Strategy::LockedCandidate,
    Strategy::NakedPair,
    Strategy::HiddenPair,
    Strategy::NakedTriple,
    Strategy::HiddenTriple,
    Strategy::NakedQuadruple,
    Strategy::HiddenQuadruple,
    Strategy::XyWing,
    Strategy::XyzWing,
    Strategy::WxyzWing,
    Strategy::SimpleColor,
    Strategy::MultiColor(usize::MAX),
    Strategy::PatternOverlay,
];

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
    },
    XyWing {
        excluded_candidates: Vec<(Pos, Value)>,
        /// Positions as [xy, xz, yz]
        positions: [Pos; 3],
        /// Values as [x, y, z]
        values: [Value; 3]
    },
    XyzWing {
        excluded_candidates: Vec<(Pos, Value)>,
        /// Positions as [xyz, xz, yz]
        positions: [Pos; 3],
        /// Values as [x, y, z]
        values: [Value; 3]
    },
    WxyzWing {
        excluded_candidates: Vec<(Pos, Value)>,
        /// Positions as [wxy(z), wz, xz, yz]
        positions: [Pos; 4],
        /// Values as [w, x, y, z]
        values: [Value; 4]
    },
    SimpleColor {
        excluded_candidates: Vec<(Pos, Value)>,
        value: Value,
        /// The positions of each color (for color wrap, the eliminated color is first)
        color_positions: [Vec<Pos>; 2],
        /// A "color wrap" is where a color sees itself, otherwise it is a "color trap" where a
        /// candidate sees both pairs of colors
        color_wrap: bool
    },
    MultiColor {
        excluded_candidates: Vec<(Pos, Value)>,
        value: Value,
        color_positions: Vec<[Vec<Pos>; 2]>
    },
    PatternOverlay {
        excluded_candidates: Vec<(Pos, Value)>,
        required_candidates: Vec<(Pos, Value)>,
        value: Value,
        remaining_patterns: usize
    },
}

impl StrategyResult {
    pub(crate) fn excluded_candidates(&self) -> Vec<(Pos, Value)> {
        match self {
            StrategyResult::NakedSingle(_, _) => Vec::new(),
            StrategyResult::HiddenSingle(_, _, _) => Vec::new(),
            StrategyResult::GuessAndCheck(_, _) => Vec::new(),
            StrategyResult::LockedCandidate { value, excluded_positions, .. } =>
                excluded_positions.iter().map(|pos| (*pos, *value)).collect(),
            StrategyResult::NakedSubset { excluded_candidates, .. } => excluded_candidates.clone(),
            StrategyResult::HiddenSubset { excluded_candidates, .. } => excluded_candidates.clone(),
            StrategyResult::XyWing { excluded_candidates, .. } => excluded_candidates.clone(),
            StrategyResult::XyzWing { excluded_candidates, .. } => excluded_candidates.clone(),
            StrategyResult::WxyzWing { excluded_candidates, .. } => excluded_candidates.clone(),
            StrategyResult::SimpleColor { excluded_candidates, .. } => excluded_candidates.clone(),
            StrategyResult::MultiColor { excluded_candidates, .. } => excluded_candidates.clone(),
            StrategyResult::PatternOverlay { excluded_candidates, .. } => excluded_candidates.clone(),
        }
    }

    pub(crate) fn required_candidates(&self) -> Vec<(Pos, Value)> {
        match self {
            StrategyResult::NakedSingle(pos, val) => vec![(*pos, *val)],
            StrategyResult::HiddenSingle(pos, val, _) => vec![(*pos, *val)],
            StrategyResult::PatternOverlay { required_candidates, .. } => required_candidates.clone(),
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
