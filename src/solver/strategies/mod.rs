use crate::solver::strategy::Strategy;

pub(crate) mod guess_and_check;
pub(crate) mod hidden_single;
pub(crate) mod locked_candidate;
pub(crate) mod naked_single;

pub const ALL: &'static [Strategy] = &[
    Strategy::NakedSingle,
    Strategy::HiddenSingle,
    Strategy::LockedCandidate,
];

pub const FAST: &'static [Strategy] = &[
    Strategy::NakedSingle,
    Strategy::HiddenSingle,
    Strategy::LockedCandidate,
];
