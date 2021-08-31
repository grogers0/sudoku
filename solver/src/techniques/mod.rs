use crate::technique::Technique;

mod naked_single;
mod guess_and_check;

pub const ALL: &'static [Technique] = &[
    Technique::NakedSingle,
];

pub const FAST: &'static [Technique] = &[
    Technique::NakedSingle,
];

pub(crate) use naked_single::naked_single;
pub(crate) use guess_and_check::guess_and_check;
