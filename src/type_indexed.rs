/// Implements a wrapper type for a usize index that allows type-safe indexing.
macro_rules! impl_index_type {
    ($IndexType:ident($UintType:ty), $Num:literal) => {
        impl $IndexType {
            const __N: $UintType = $Num; // This should prevent mistakes where the size of the UintType is too small
            pub const N: usize = $Num;

            #[allow(dead_code)]
            #[inline]
            pub fn new(idx: usize) -> Self {
                if idx >= Self::N as usize { panic!("Index out of bounds") }
                Self(idx as $UintType)
            }

            #[inline]
            pub const unsafe fn new_unchecked(idx: usize) -> Self {
                Self(idx as $UintType)
            }

            #[inline]
            pub const fn as_usize(&self) -> usize {
                self.0 as usize
            }

            #[inline]
            pub fn iter() -> std::iter::Map<std::ops::Range<usize>, fn(usize) -> Self> {
                (0..Self::N).map(|idx| unsafe { Self::new_unchecked(idx) })
            }
        }

        impl Copy for $IndexType {}

        impl Clone for $IndexType {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl std::fmt::Debug for $IndexType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($IndexType), self.0)
            }
        }

        impl PartialEq for $IndexType {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Eq for $IndexType {}

        impl PartialOrd for $IndexType {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl Ord for $IndexType {
            #[inline]
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
    }
}


/// Implements a slice that can be indexed by a user-defined type. This allows type-safe indexing
/// without bounds checks. The `IndexType` class must look like:
///
/// ```
/// impl Foo {
///     pub const N: usize = ...;
///     pub const fn as_usize(&self) -> usize { ... }
/// }
/// ```
///
/// These cannot be part of a trait because of current limitations in rustc.
///
/// Additionally, the `IndexType` class must prevent any instance from being created that would allow
/// `as_usize` to return a value greater than or equal to `N`.
///
/// Use the [`impl_index_type!`] macro to generate the boilerplate.
macro_rules! impl_type_indexed_slice {
    ($SliceName:ident, $IndexType:ty, $Visibility:vis) => {
        $Visibility struct $SliceName<T>([T; <$IndexType>::N]);

        impl <T> $SliceName<T> {
            #[inline]
            pub const fn from_slice(other: [T; <$IndexType>::N]) -> Self {
                Self(other)
            }

            #[allow(dead_code)]
            #[inline]
            pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> {
                self.0.iter()
            }

            #[allow(dead_code)]
            #[inline]
            pub fn iter_mut<'a>(&'a mut self) -> std::slice::IterMut<'a, T> {
                self.0.iter_mut()
            }
        }

        impl <T> std::ops::Index<$IndexType> for $SliceName<T> {
            type Output = T;
            #[inline]
            fn index(&self, idx: $IndexType) -> &Self::Output {
                unsafe { self.0.get_unchecked(idx.as_usize()) }
            }
        }

        impl <T> std::ops::IndexMut<$IndexType> for $SliceName<T> {
            #[inline]
            fn index_mut(&mut self, idx: $IndexType) -> &mut Self::Output {
                unsafe { self.0.get_unchecked_mut(idx.as_usize()) }
            }
        }

        impl <T: Clone> Clone for $SliceName<T> {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl <T: std::fmt::Debug> std::fmt::Debug for $SliceName<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[")?;
                for i in 0 .. <$IndexType>::N {
                    if i > 0 { write!(f, ", ")? }
                    write!(f, "{:?}", self.0[i])?;
                }
                write!(f, "]")
            }
        }

        impl <T: PartialEq> PartialEq for $SliceName<T> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl <T: Eq> Eq for $SliceName<T> { }

        impl <'a, T> IntoIterator for &'a $SliceName<T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }
    }
}

/// Implements a bitset that can be indexed by a user-defined type. This allows type-safe indexing
/// without bounds checks. The `IndexType` class must look like:
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
/// Additionally, the `IndexType` class must prevent any instance from being created that would allow
/// `as_usize` to return a value greater than or equal to `N`.
///
/// Use the [`impl_index_type!`] macro to generate the boilerplate.
macro_rules! impl_type_indexed_bitset {
    ($BitSetName:ident, $IndexType:ty, $UintType:ty, $IterName:ident, $Visibility:vis) => {
        #[derive(Copy, Clone, PartialEq, Eq)]
        $Visibility struct $BitSetName($UintType);

        impl $BitSetName {
            pub const NONE: Self = Self(0);
            pub const ALL: Self = Self((1 << <$IndexType>::N) - 1);

            #[allow(dead_code)]
            #[inline]
            pub const fn new() -> Self {
                Self::NONE
            }

            #[inline]
            pub fn insert(&mut self, idx: $IndexType) {
                self.0 |= 1 << idx.as_usize();
            }

            #[inline]
            pub fn remove(&mut self, idx: $IndexType) {
                self.0 &= !(1 << idx.as_usize());
            }

            #[inline]
            pub const fn contains(&self, idx: $IndexType) -> bool {
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

            #[allow(dead_code)]
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.0 == Self::NONE.0
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

        impl std::fmt::Debug for $BitSetName {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        impl std::iter::FromIterator<$IndexType> for $BitSetName {
            #[inline]
            fn from_iter<It>(iter: It) -> Self
            where It: IntoIterator<Item = $IndexType> {
                let mut ret = Self::new();
                for idx in iter {
                    ret.insert(idx);
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
            type Item = $IndexType;
            fn next(&mut self) -> Option<Self::Item> {
                let trailing_zeros = self.bits.trailing_zeros() as usize;
                if self.offset + trailing_zeros >= <$IndexType>::N {
                    return None
                }
                let ret = Some(unsafe { <$IndexType>::new_unchecked(self.offset + trailing_zeros) });
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
