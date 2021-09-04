use std::{
    fmt,
    iter,
    ops::Range,
};

/// Represents the value each cell can have, from 1..=9 (stored as 0..9 for ease of lookup in
/// arrays)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(u8);

impl_type_indexed_slice!(ValueIndexedSlice, Value, pub(crate));
impl_type_indexed_bitset!(ValueBitSet, Value, u16, ValueBitSetIter, pub(crate));

impl Value {
    pub const N: usize = 9;

    #[inline]
    pub fn new(idx: usize) -> Self {
        if idx >= Self::N as usize { panic!("Value out of bounds") }
        Self(idx as u8)
    }

    #[inline]
    pub const unsafe fn new_unchecked(idx: usize) -> Self {
        Self(idx as u8)
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub fn iter() -> iter::Map<Range<usize>, fn(usize) -> Self> {
        (0..Self::N).map(|idx| unsafe { Self::new_unchecked(idx) })
    }

    pub fn from_char(ch: char) -> Option<Self> {
        if !('1'..='9').contains(&ch) { return None }
        Some(Self::new(ch.to_digit(10).unwrap() as usize - 1))
    }

    pub fn to_char(&self) -> char {
        char::from_digit(self.0 as u32 + 1, 10).unwrap()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

/// Rust requires nightly-only compiler magic (#[random_guess_and_check_to_fill]) to optimize
/// Option<Value> to not add an extra byte (or more). We can get around that by using this type and
/// doing the conversions.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct MaybeValue(u8);

impl MaybeValue {
    pub const NONE: Self = Self(255); // Anything >= 9 would work

    #[inline]
    pub fn from_option(other: Option<Value>) -> Self {
        match other {
            Some(val) => Self(val.0),
            None => Self::NONE
        }
    }

    #[inline]
    pub fn to_option(&self) -> Option<Value> {
        if *self == Self::NONE {
            None
        } else {
            Some(unsafe { Value::new_unchecked(self.0 as usize) })
        }
    }
}
