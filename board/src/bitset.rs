use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BitSet9(u16);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BitSet81(u128);

impl BitSet9 {
    const BITS: usize = 9;
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self((1 << Self::BITS) - 1);

    /// Returns the set with a single bit set
    #[inline]
    pub fn single(idx: usize) -> Self {
        if idx >= Self::BITS { panic!("Index out of bounds") }
        unsafe { Self::single_unchecked(idx) }
    }

    #[inline]
    pub const unsafe fn single_unchecked(idx: usize) -> Self {
        Self(1 << idx)
    }

    /// Set a single bit
    #[inline]
    pub fn set(&mut self, idx: usize) {
        if idx >= Self::BITS { panic!("Index out of bounds") }
        self.0 |= 1 << idx;
    }

    /// Clears a single bit
    #[inline]
    pub fn clear(&mut self, idx: usize) {
        if idx >= Self::BITS { panic!("Index out of bounds") }
        self.0 &= !(1 << idx)
    }

    /// Gets whether a single bit is set or not
    #[inline]
    pub fn get(self, idx: usize) -> bool {
        if idx >= Self::BITS { panic!("Index out of bounds") }
        (self.0 & (1 << idx)) != 0
    }

    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    pub const fn intersect(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    pub const fn inversed(self) -> Self {
        Self(!self.0 & Self::ALL.0)
    }

    #[inline]
    pub const fn count_ones(&self) -> usize {
        self.0.count_ones() as usize
    }

    // Iterate over the positions of the bits that are set
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=usize> {
        BitSet9Iter::new(*self)
    }
}


impl BitAnd for BitSet9 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersect(other)
    }
}

impl BitAndAssign for BitSet9 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = self.intersect(other);
    }
}

impl BitOr for BitSet9 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}

impl BitOrAssign for BitSet9 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = self.union(other);
    }
}

impl BitXor for BitSet9 {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }
}

impl BitXorAssign for BitSet9 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl Not for BitSet9 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.inversed()
    }
}

pub struct BitSet9Iter {
    bits: u16,
    offset: usize
}

impl BitSet9Iter {
    #[inline]
    fn new(bitset: BitSet9) -> Self {
        Self { bits: bitset.0, offset: 0 }
    }
}

impl Iterator for BitSet9Iter {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let trailing_zeros = self.bits.trailing_zeros() as usize;
        if self.offset + trailing_zeros >= 9 {
            return None
        }
        let ret = Some(self.offset + trailing_zeros);
        self.offset += trailing_zeros + 1;
        self.bits >>= trailing_zeros + 1;
        ret
    }
}

impl BitSet81 {
    const BITS: usize = 81;
    pub const NONE: Self = Self(0);
    pub const ALL: Self = Self((1 << Self::BITS) - 1);

    /// Returns the set with a single bit set
    #[inline]
    pub fn single(idx: usize) -> Self {
        if idx >= Self::BITS { panic!("Index out of bounds") }
        unsafe { Self::single_unchecked(idx) }
    }

    #[inline]
    pub const unsafe fn single_unchecked(idx: usize) -> Self {
        Self(1 << idx)
    }

    /// Set a single bit
    #[inline]
    pub fn set(&mut self, idx: usize) {
        if idx >= Self::BITS { panic!("Index out of bounds") }
        self.0 |= 1 << idx;
    }

    /// Clears a single bit
    #[inline]
    pub fn clear(&mut self, idx: usize) {
        if idx >= Self::BITS { panic!("Index out of bounds") }
        self.0 &= !(1 << idx)
    }

    /// Gets whether a single bit is set or not
    #[inline]
    pub fn get(self, idx: usize) -> bool {
        if idx >= Self::BITS { panic!("Index out of bounds") }
        (self.0 & (1 << idx)) != 0
    }

    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    pub const fn intersect(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    pub const fn inversed(self) -> Self {
        Self(!self.0 & Self::ALL.0)
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn count_ones(&self) -> usize {
        self.0.count_ones() as usize
    }
    // Iterate over the positions of the bits that are set
    #[allow(dead_code)]
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=usize> {
        BitSet81Iter::new(*self)
    }
}


impl BitAnd for BitSet81 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersect(other)
    }
}

impl BitAndAssign for BitSet81 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = self.intersect(other);
    }
}

impl BitOr for BitSet81 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}

impl BitOrAssign for BitSet81 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = self.union(other);
    }
}

impl BitXor for BitSet81 {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }
}

impl BitXorAssign for BitSet81 {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl Not for BitSet81 {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.inversed()
    }
}

pub struct BitSet81Iter {
    bits: u128,
    offset: usize
}

impl BitSet81Iter {
    #[inline]
    fn new(bitset: BitSet81) -> Self {
        Self { bits: bitset.0, offset: 0 }
    }
}

impl Iterator for BitSet81Iter {
    type Item = usize;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let trailing_zeros = self.bits.trailing_zeros() as usize;
        if self.offset + trailing_zeros >= 81 {
            return None
        }
        let ret = Some(self.offset + trailing_zeros);
        self.offset += trailing_zeros + 1;
        self.bits >>= trailing_zeros + 1;
        ret
    }
}
