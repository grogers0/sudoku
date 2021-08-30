use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BitSet9(pub(crate) u16);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BitSet81(pub(crate) u128);

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
        (self.0 & (1 << idx)) == 0
    }
}


impl BitAnd for BitSet9 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl BitAndAssign for BitSet9 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
}

impl BitOr for BitSet9 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for BitSet9 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
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
        Self(!self.0 & Self::ALL.0)
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
        (self.0 & (1 << idx)) == 0
    }
}


impl BitAnd for BitSet81 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl BitAndAssign for BitSet81 {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
}

impl BitOr for BitSet81 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl BitOrAssign for BitSet81 {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
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
        Self(!self.0 & Self::ALL.0)
    }
}



/*


type BitSet9 = BitSet<u16, 9>;
type BitSet81 = BitSet<u128, 81>;

struct BitSet<Storage: BitSetStorage<NBITS>, const NBITS: usize>(Storage);

impl <Storage: BitSetStorage<NBITS>, const NBITS: usize> BitSet<Storage, NBITS> {
    const ZERO: Self = Self::new(Storage::ZERO);

    const fn new(storage: Storage) -> Self { Self(storage) }
    pub fn count_ones(&self) -> usize { self.0._count_ones() }
}

trait BitSetStorage<const NBITS: usize> :
    BitAnd + BitAndAssign +
    BitOr + BitOrAssign +
    BitXor + BitXorAssign +
    Not +
    Sized +
    Copy +
    PartialEq +
    Sub<Self, Output=Self> +
    Shl<usize, Output=Self>
{
    const BITS: usize;
    const ZERO: Self;
    const ONE: Self;
    fn _count_ones(self) -> usize;
}

impl <const NBITS: usize> BitSetStorage<NBITS> for u16 {
    const BITS: usize = Self::BITS as usize;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    fn _count_ones(self) -> usize { self.count_ones() as usize }
}

impl <const NBITS: usize> BitSetStorage<NBITS> for u32 {
    const BITS: usize = Self::BITS as usize;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    fn _count_ones(self) -> usize { self.count_ones() as usize }
}

impl <const NBITS: usize> BitSetStorage<NBITS> for u128 {
    const BITS: usize = Self::BITS as usize;
    const ZERO: Self = 0;
    const ONE: Self = 1;
    fn _count_ones(self) -> usize { self.count_ones() as usize }
}
*/
