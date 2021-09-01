use crate::solver::strategy::Strategy;

pub(crate) mod naked_single;
pub(crate) mod guess_and_check;

pub const ALL: &'static [Strategy] = &[
    Strategy::NakedSingle,
];

pub const FAST: &'static [Strategy] = &[
    Strategy::NakedSingle,
];
