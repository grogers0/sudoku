/// Implements a bitset that can be indexed by a user-defined type. This allows type-safe indexing
/// without bounds checks. The `KeyType` class must look like:
///
/// ```
/// impl Foo {
///     pub const N: usize = ...;
///     pub const fn as_usize(&self) -> usize { ... }
///     pub const unsafe fn new_unchecked(idx: usize) -> Self { ... } // Safe if and only if idx < N
/// }
/// ```
///
/// These cannot be part of a trait because of current limitations in rustc.
///
/// Additionally, the `KeyType` class must prevent any instance from being created that would allow
/// `as_usize` to return a value greater than or equal to `N`
macro_rules! impl_type_indexed_bitset {
    ($BitSetName:ident, $KeyType:ty, $UintType:ty, $IterName:ident, $Visibility:vis) => {
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        $Visibility struct $BitSetName($UintType);

        impl $BitSetName {
            pub const NONE: Self = Self(0);
            pub const ALL: Self = Self((1 << <$KeyType>::N) - 1);

            #[allow(dead_code)]
            #[inline]
            pub const fn new() -> Self {
                Self::NONE
            }

            #[inline]
            pub fn set(&mut self, idx: $KeyType) {
                self.0 |= 1 << idx.as_usize();
            }

            #[inline]
            pub fn remove(&mut self, idx: $KeyType) {
                self.0 &= !(1 << idx.as_usize());
            }

            #[inline]
            pub const fn contains(&self, idx: $KeyType) -> bool {
                (self.0 & (1 << idx.as_usize())) != 0
            }

            #[inline]
            pub const fn union(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }

            #[inline]
            pub const fn intersection(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }

            /// Returns a bitset with all elements of `self` which aren't in `other`
            #[allow(dead_code)]
            #[inline]
            pub const fn difference(self, other: Self) -> Self {
                Self(self.0 & !(self.0 & other.0))
            }

            /// Returns a bitset with all elements of `self` and `other` which aren't in both
            #[allow(dead_code)]
            #[inline]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                self.difference(other).union(other.difference(self))
            }

            /// Returns a bitset with all elements not in `self`
            #[inline]
            pub const fn inversed(self) -> Self {
                Self(!self.0 & Self::ALL.0)
            }

            /// The length is the number of bits set
            #[allow(dead_code)]
            #[inline]
            pub const fn len(&self) -> usize {
                self.0.count_ones() as usize
            }

            /// Returns an iterator which yields all elements whose bits that are set
            #[allow(dead_code)]
            #[inline]
            pub fn iter(&self) -> $IterName {
                <$IterName>::new(*self)
            }
        }

        impl std::ops::BitAnd for $BitSetName {
            type Output = Self;
            #[inline]
            fn bitand(self, other: Self) -> Self {
                self.intersection(other)
            }
        }

        impl std::ops::BitAndAssign for $BitSetName {
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                *self = self.intersection(other);
            }
        }

        impl std::ops::BitOr for $BitSetName {
            type Output = Self;
            #[inline]
            fn bitor(self, other: Self) -> Self {
                self.union(other)
            }
        }

        impl std::ops::BitOrAssign for $BitSetName {
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                *self = self.union(other);
            }
        }

        impl std::iter::FromIterator<$KeyType> for $BitSetName {
            #[inline]
            fn from_iter<It>(iter: It) -> Self
            where It: IntoIterator<Item = $KeyType> {
                let mut ret = Self::new();
                for idx in iter {
                    ret.set(idx);
                }
                ret
            }
        }

        impl std::ops::Not for $BitSetName {
            type Output = Self;
            #[inline]
            fn not(self) -> Self {
                self.inversed()
            }
        }

        $Visibility struct $IterName { bits: $UintType, offset: usize }

        impl $IterName {
            #[inline]
            fn new(bitset: $BitSetName) -> Self {
                Self { bits: bitset.0, offset: 0 }
            }
        }

        impl Iterator for $IterName {
            type Item = $KeyType;
            fn next(&mut self) -> Option<Self::Item> {
                let trailing_zeros = self.bits.trailing_zeros() as usize;
                if self.offset + trailing_zeros >= <$KeyType>::N {
                    return None
                }
                let ret = Some(unsafe { <$KeyType>::new_unchecked(self.offset + trailing_zeros) });
                self.offset += trailing_zeros + 1;
                self.bits >>= trailing_zeros + 1;
                ret
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.bits.count_ones() as usize;
                (len, Some(len))
            }
        }

        impl std::iter::FusedIterator for $IterName {}

        impl ExactSizeIterator for $IterName {}

        // TODO when stabilized: std::iter::TrustedLen
    }
}
