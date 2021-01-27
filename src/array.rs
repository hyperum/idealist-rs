use super::{slice::IdealSlice, vec::IdealVec};
use alloc::vec::Vec;
use core::{
    //array::TryFromSliceError,
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Debug},
    hash::{self, Hash},
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
    slice::{Iter, IterMut},
};

#[repr(transparent)]
pub struct IdealArray<T, const N: NonZeroUsize>(pub(crate) [T; N.get()])
where
    [T; N.get()]: Sized;

impl<T, const N: NonZeroUsize> IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    pub fn new(array: [T; N.get()]) -> Self {
        Self(array)
    }

    pub fn get(self) -> [T; N.get()] {
        self.0
    }

    pub fn get_ref(&self) -> &[T; N.get()] {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut [T; N.get()] {
        &mut self.0
    }
}

impl<T, const N: NonZeroUsize> AsMut<[T]> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.0.as_mut()
    }
}

impl<T, const N: NonZeroUsize> AsMut<IdealSlice<T>> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    #[inline]
    fn as_mut(&mut self) -> &mut IdealSlice<T> {
        unsafe { IdealSlice::new_mut_unchecked(self.0.as_mut()) }
    }
}

impl<T, const N: NonZeroUsize> AsRef<[T]> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T, const N: NonZeroUsize> AsRef<IdealSlice<T>> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    #[inline]
    fn as_ref(&self) -> &IdealSlice<T> {
        unsafe { IdealSlice::new_ref_unchecked(self.0.as_ref()) }
    }
}

impl<T, const N: NonZeroUsize> Borrow<[T]> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    fn borrow(&self) -> &[T] {
        self.0.borrow()
    }
}

impl<T, const N: NonZeroUsize> Borrow<IdealSlice<T>> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    fn borrow(&self) -> &IdealSlice<T> {
        unsafe { IdealSlice::new_ref_unchecked(self.0.borrow()) }
    }
}

impl<T, const N: NonZeroUsize> BorrowMut<[T]> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        self.0.borrow_mut()
    }
}

impl<T, const N: NonZeroUsize> BorrowMut<IdealSlice<T>> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut IdealSlice<T> {
        unsafe { IdealSlice::new_mut_unchecked(self.0.borrow_mut()) }
    }
}

impl<T: Clone, const N: NonZeroUsize> Clone for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy, const N: NonZeroUsize> Copy for IdealArray<T, N> where [T; N.get()]: Sized {}

impl<T: Debug, const N: NonZeroUsize> Debug for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// TODO: Add Default implementation, either with macro or, if ever supported in the future, generically.

// TODO: Replace Deref/DerefMut-coercion With Unsize-coercion if possible in the future.
impl<T, const N: NonZeroUsize> Deref for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    type Target = IdealSlice<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { IdealSlice::new_ref_unchecked(&self.0) }
    }
}

impl<T, const N: NonZeroUsize> DerefMut for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { IdealSlice::new_mut_unchecked(&mut self.0) }
    }
}

impl<T: Eq, const N: NonZeroUsize> Eq for IdealArray<T, N> where [T; N.get()]: Sized {}

impl<T: Hash, const N: NonZeroUsize> Hash for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<'a, T, const N: NonZeroUsize> IntoIterator for &'a IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T, const N: NonZeroUsize> IntoIterator for &'a mut IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a, T: Ord, const N: NonZeroUsize> Ord for &'a IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<'a, const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<&'a [A]> for IdealArray<B, N>
where
    [B; N.get()]: Sized,
{
    #[inline]
    fn eq(&self, other: &&'a [A]) -> bool {
        self.0 == *other
    }
}

impl<'a, const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<&'a mut [A]> for IdealArray<B, N>
where
    [B; N.get()]: Sized,
{
    #[inline]
    fn eq(&self, other: &&'a mut [A]) -> bool {
        self.0 == *other
    }
}

impl<'a, const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<&'a IdealSlice<A>>
    for IdealArray<B, N>
where
    [B; N.get()]: Sized,
{
    #[inline]
    fn eq(&self, other: &&'a IdealSlice<A>) -> bool {
        self.0 == other.0
    }
}

impl<'a, const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<&'a mut IdealSlice<A>>
    for IdealArray<B, N>
where
    [B; N.get()]: Sized,
{
    #[inline]
    fn eq(&self, other: &&'a mut IdealSlice<A>) -> bool {
        self.0 == other.0
    }
}

impl<const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<[A; N.get()]> for IdealArray<B, N>
where
    [B; N.get()]: Sized,
{
    #[inline]
    fn eq(&self, other: &[A; N.get()]) -> bool {
        self.0 == *other
    }
}

impl<const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<IdealArray<A, N>> for IdealArray<B, N>
where
    [B; N.get()]: Sized,
{
    #[inline]
    fn eq(&self, other: &IdealArray<A, N>) -> bool {
        self.0 == other.0
    }
}

impl<const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<[A]> for IdealArray<B, N>
where
    [B; N.get()]: Sized,
{
    #[inline]
    fn eq(&self, other: &[A]) -> bool {
        self.0 == *other
    }
}

impl<const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<IdealSlice<A>> for IdealArray<B, N>
where
    [B; N.get()]: Sized,
{
    #[inline]
    fn eq(&self, other: &IdealSlice<A>) -> bool {
        self.0 == other.0
    }
}

impl<T: PartialOrd<T>, const N: NonZeroUsize> PartialOrd<[T; N.get()]> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    #[inline]
    fn partial_cmp(&self, other: &[T; N.get()]) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
    #[inline]
    fn lt(&self, other: &[T; N.get()]) -> bool {
        self.0.lt(other)
    }
    #[inline]
    fn le(&self, other: &[T; N.get()]) -> bool {
        self.0.le(other)
    }
    #[inline]
    fn ge(&self, other: &[T; N.get()]) -> bool {
        self.0.ge(other)
    }
    #[inline]
    fn gt(&self, other: &[T; N.get()]) -> bool {
        self.0.gt(other)
    }
}

impl<T: PartialOrd<T>, const N: NonZeroUsize> PartialOrd<Self> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }
}

// TODO: Uncomment the following three (six) TryFrom implementations when they no longer trigger ICEs.
// TODO: Implement TryFrom &[T] for ref owned, ref ref, mut mut
/*
impl<T: Copy, const N: NonZeroUsize> TryFrom<&IdealSlice<T>> for IdealArray<T, N> where [T; N.get()]: Sized {
    type Error = TryFromSliceError;
    //#[inline]
    fn try_from(slice: &IdealSlice<T>) -> Result<Self, Self::Error> {
        <[T; N.get()] as TryFrom<&[T]>>::try_from(&slice.0)
    }
}

impl<'a, T, const N: NonZeroUsize> TryFrom<&'a IdealSlice<T>> for &'a IdealArray<T, N> where [T; N.get()]: Sized, T: Copy {
    type Error = TryFromSliceError;
    #[inline]
    fn try_from(slice: &'a IdealSlice<T>) -> Result<Self, Self::Error> {
        <[T; N.get()] as TryFrom<&'a [T]>>::try_from(&slice.0)
    }
}

impl<'a, T: Copy, const N: NonZeroUsize> TryFrom<&'a mut IdealSlice<T>> for &'a mut IdealArray<T, N> where [T; N.get()]: Sized {
    type Error = TryFromSliceError;
    #[inline]
    fn try_from(slice: &'a mut IdealSlice<T>) -> Result<Self, Self::Error> {
        unsafe {
            <&'a mut [T; N.get()] as TryFrom<&'a mut [T]>>::try_from(&mut slice.0).map(|ok| generic_transmute::<&'a mut [T; N.get()], Self>(ok))
        }
    }
}*/

impl<T: Copy, const N: NonZeroUsize> TryFrom<Vec<T>> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    type Error = Vec<T>;
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        <[T; N.get()]>::try_from(vec).map(Self)
    }
}

impl<T: Copy, const N: NonZeroUsize> TryFrom<IdealVec<T>> for IdealArray<T, N>
where
    [T; N.get()]: Sized,
{
    type Error = IdealVec<T>;
    fn try_from(vec: IdealVec<T>) -> Result<Self, Self::Error> {
        Self::try_from(vec.0).map_err(IdealVec)
    }
}
