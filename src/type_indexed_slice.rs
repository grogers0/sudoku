/// Implements a slice that can be indexed by a user-defined type. This allows type-safe indexing
/// without bounds checks. The `KeyType` class must look like:
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
/// Additionally, the `KeyType` class must prevent any instance from being created that would allow
/// `as_usize` to return a value greater than or equal to `N`
macro_rules! impl_type_indexed_slice {
    ($StructName:ident, $KeyType:ty, $Visibility:vis) => {
        $Visibility struct $StructName<T>([T; <$KeyType>::N]);

        impl <T> $StructName<T> {
            #[inline]
            pub const fn from_slice(other: [T; <$KeyType>::N]) -> Self {
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

        impl <T> std::ops::Index<$KeyType> for $StructName<T> {
            type Output = T;
            #[inline]
            fn index(&self, idx: $KeyType) -> &Self::Output {
                unsafe { self.0.get_unchecked(idx.as_usize()) }
            }
        }

        impl <T> std::ops::IndexMut<$KeyType> for $StructName<T> {
            #[inline]
            fn index_mut(&mut self, idx: $KeyType) -> &mut Self::Output {
                unsafe { self.0.get_unchecked_mut(idx.as_usize()) }
            }
        }

        impl <T: Clone> Clone for $StructName<T> {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl <T: PartialEq> PartialEq for $StructName<T> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl <T: Eq> Eq for $StructName<T> { }

        impl <'a, T> IntoIterator for &'a $StructName<T> {
            type Item = &'a T;
            type IntoIter = std::slice::Iter<'a, T>;
            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }
    }
}
