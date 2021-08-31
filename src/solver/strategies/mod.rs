use crate::solver::strategy::Strategy;

mod naked_single;
mod guess_and_check;

pub const ALL: &'static [Strategy] = &[
    Strategy::NakedSingle,
];

pub const FAST: &'static [Strategy] = &[
    Strategy::NakedSingle,
];

pub(crate) use naked_single::naked_single;
pub(crate) use guess_and_check::guess_and_check;
