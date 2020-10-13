use core::{
	borrow::{
		Borrow,
		BorrowMut
	},
	cmp::Ordering,
	fmt::{
		self,
		Debug,
	},
	hash::{
		self,
		Hash,
	},
	iter::FromIterator,
	num::NonZeroUsize,
	ops::{
		Deref,
		DerefMut,
		Index,
		IndexMut,
	},
	slice::{
		Iter,
		IterMut,
		SliceIndex
	},
};
use alloc::{
	borrow::{
		Cow,
		ToOwned,
	},
	boxed::Box,
	vec::{
		IntoIter,
		Vec,
	},
};

use super::{
	pointer_class::*,
	slice::*,
	array::*,
};

#[repr(transparent)]
pub struct IdealVec<T>(pub(crate) Vec<T>);

impl<T> IdealVec<T> {
	pub unsafe fn new_unchecked(vec: Vec<T>) -> Self {
		Self(vec)
	}

	pub fn new(vec: Vec<T>) -> Option<Self> {
		if vec.len() != 0 {
			Some(Self(vec))
		} else {
			None
		}
	}

	pub fn get(self) -> Vec<T> {
		self.0
	}

	pub fn get_ref<'a>(&'a self) -> &'a Vec<T> {
		&self.0
	}

	pub unsafe fn get_mut<'a>(&'a mut self) -> &'a mut Vec<T> {
		&mut self.0
	}

	pub fn as_ideal_slice(&self) -> &IdealSlice<T> {
		unsafe {IdealSlice::new_ref_unchecked(self.0.as_slice())}
	}

	pub fn len(&self) -> NonZeroUsize {
		unsafe {NonZeroUsize::new_unchecked(self.0.len())}
	}

	pub fn into_boxed_ideal_slice(self) -> Box<IdealSlice<T>> {
		unsafe {IdealSlice::new_ptr_unchecked::<WithBox>(self.0.into_boxed_slice())}
	}
}

impl<T> AsMut<[T]> for IdealVec<T> {
	fn as_mut(&mut self) -> &mut [T] {self.0.as_mut()}
}

impl<T> AsMut<IdealSlice<T>> for IdealVec<T> {
	fn as_mut(&mut self) -> &mut IdealSlice<T> {
		unsafe {IdealSlice::new_mut_unchecked(self.0.as_mut())}
	}
}

impl<T> AsMut<Self> for IdealVec<T> {
	fn as_mut(&mut self) -> &mut Self {self}
}

impl<T> AsRef<[T]> for IdealVec<T> {
	fn as_ref(&self) -> &[T] {self.0.as_ref()}
}

impl<T> AsRef<IdealSlice<T>> for IdealVec<T> {
	fn as_ref(&self) -> &IdealSlice<T> {
		unsafe {IdealSlice::new_ref_unchecked(self.0.as_ref())}
	}
}

impl<T> AsRef<Vec<T>> for IdealVec<T> {
	fn as_ref(&self) -> &Vec<T> {self.0.as_ref()}
}

impl<T> AsRef<Self> for IdealVec<T> {
	fn as_ref(&self) -> &Self {self}
}

impl<T> Borrow<[T]> for IdealVec<T> {
	fn borrow(&self) -> &[T] {
		self.0.borrow()
	}
}

impl<T> Borrow<IdealSlice<T>> for IdealVec<T> {
	fn borrow(&self) -> &IdealSlice<T> {
		unsafe {IdealSlice::new_ref_unchecked(self.0.borrow())}
	}
}

impl<T> BorrowMut<[T]> for IdealVec<T> {
	fn borrow_mut(&mut self) -> &mut [T] {
		self.0.borrow_mut()
	}
}

impl<T> BorrowMut<IdealSlice<T>> for IdealVec<T> {
	fn borrow_mut(&mut self) -> &mut IdealSlice<T> {
		unsafe {IdealSlice::new_mut_unchecked(self.0.borrow_mut())}
	}
}
impl<T: Clone> Clone for IdealVec<T> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}

	fn clone_from(&mut self, other: &Self) {
		other.as_ideal_slice().clone_into(self);
	}
}

impl<T: Debug> Debug for IdealVec<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {self.0.fmt(f)}
}

impl<T> Deref for IdealVec<T> {
	type Target = IdealSlice<T>;
	fn deref(&self) -> &Self::Target {
		unsafe {IdealSlice::new_ref_unchecked(&self.0)}
	}
}

impl<T> DerefMut for IdealVec<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {IdealSlice::new_mut_unchecked(&mut self.0)}
	}
}

impl<T: Eq> Eq for IdealVec<T> {}

impl<'a, T: 'a + Copy> Extend<&'a T> for IdealVec<T> {
	fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {self.0.extend(iter)}
	#[inline]
	fn extend_one(&mut self, &item: &'a T) {self.0.extend_one(item)}
	#[inline]
	fn extend_reserve(&mut self, additional: usize) {<Vec<T> as Extend<&'a T>>::extend_reserve(&mut self.0, additional)}
}

impl<T> Extend<T> for IdealVec<T> {
	#[inline]
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {self.0.extend(iter)}
	#[inline]
	fn extend_one(&mut self, item: T) {self.0.extend_one(item)}
	#[inline]
	fn extend_reserve(&mut self, additional: usize) {self.0.extend_reserve(additional)}
}

// TODO: Implement From RefIdealStr, IdealBinaryHeap, IdealString, IdealVecDeque

impl<T: Clone> From<&IdealSlice<T>> for IdealVec<T> {
	fn from(s: &IdealSlice<T>) -> IdealVec<T> {Self(Vec::from(&s.0))}
}

impl<T: Clone> From<&mut IdealSlice<T>> for IdealVec<T> {
	fn from(s: &mut IdealSlice<T>) -> IdealVec<T> {Self(Vec::from(&mut s.0))}
}

impl<T, const N: NonZeroUsize> From<IdealArray<T, N>> for IdealVec<T> where [T; N.get()]: Sized{
	fn from(s: IdealArray<T, N>) -> IdealVec<T> {Self(Vec::from(s.0))}
}

impl<T> From<Box<IdealSlice<T>>> for IdealVec<T> {
	fn from(s: Box<IdealSlice<T>>) -> IdealVec<T> {
		Self(<Vec::<T> as From<Box<[T]>>>::from(s.get_ptr::<WithBox>()))
	}
}

impl<'a, T> From<Cow<'a, IdealSlice<T>>> for IdealVec<T>
where
    IdealSlice<T>: ToOwned<Owned = IdealVec<T>>,
{
    fn from(s: Cow<'a, IdealSlice<T>>) -> Self {
        s.into_owned()
    }
}

impl<T> FromIterator<T> for IdealVec<T> {
	#[inline]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> IdealVec<T> {Self(Vec::<T>::from_iter(iter))}
}

impl<T: Hash> Hash for IdealVec<T> {
	#[inline]
	fn hash<H: hash::Hasher>(&self, state: &mut H) {self.0.hash(state)}
}

impl<T, I: SliceIndex<[T]>> Index<I> for IdealVec<T> {
	type Output = I::Output;
	#[inline]
	fn index(&self, index: I) -> &Self::Output {self.0.index(index)}
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for IdealVec<T> {
	#[inline]
	fn index_mut(&mut self, index: I) -> &mut Self::Output {self.0.index_mut(index)}
}

impl<T> IntoIterator for IdealVec<T> {
	type Item = T;
	type IntoIter = IntoIter<T>;
	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

impl<'a, T> IntoIterator for &'a IdealVec<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;
	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl<'a, T> IntoIterator for &'a mut IdealVec<T> {
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;
	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter_mut()
	}
}

impl<T: Ord> Ord for IdealVec<T> {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {self.0.cmp(&other.0)}
}

impl<'a, A, B: PartialEq<A>> PartialEq<&'a [A]> for IdealVec<B> {
	#[inline]
	fn eq(&self, other: &&'a [A]) -> bool {self.0 == *other}
	#[inline]
	fn ne(&self, other: &&'a [A]) -> bool {self.0 != *other}
}

impl<'a, A, B: PartialEq<A>> PartialEq<&'a IdealSlice<A>> for IdealVec<B>{
	#[inline]
	fn eq(&self, other: &&'a IdealSlice<A>) -> bool {self.0 == &other.0}
	#[inline]
	fn ne(&self, other: &&'a IdealSlice<A>) -> bool {self.0 != &other.0}
}

impl<'a, A, B: PartialEq<A>> PartialEq<&'a mut [A]> for IdealVec<B> {
	#[inline]
	fn eq(&self, other: &&'a mut [A]) -> bool {self.0 == *other}
	#[inline]
	fn ne(&self, other: &&'a mut [A]) -> bool {self.0 != *other}
}

impl<'a, A, B: PartialEq<A>> PartialEq<&'a mut IdealSlice<A>> for IdealVec<B>{
	#[inline]
	fn eq(&self, other: &&'a mut IdealSlice<A>) -> bool {self.0 == &other.0}
	#[inline]
	fn ne(&self, other: &&'a mut IdealSlice<A>) -> bool {self.0 != &other.0}
}

impl<A, B: PartialEq<A>, const N: usize> PartialEq<[A; N]> for IdealVec<B> {
	#[inline]
	fn eq(&self, other: &[A; N]) -> bool {self.0 == other}
	#[inline]
	fn ne(&self, other: &[A; N]) -> bool {self.0 != other}
}

impl<A, B: PartialEq<A>, const N: NonZeroUsize> PartialEq<IdealArray<A, N>> for IdealVec<B> where [B; N.get()]: Sized {
	#[inline]
	fn eq(&self, other: &IdealArray<A, N>) -> bool {self.0 == other.0}
	#[inline]
	fn ne(&self, other: &IdealArray<A, N>) -> bool {self.0 != other.0}
}

impl<A, B: PartialEq<A>> PartialEq<Vec<A>> for IdealVec<B> {
	#[inline]
	fn eq(&self, other: &Vec<A>) -> bool {self.0 == *other}
	#[inline]
	fn ne(&self, other: &Vec<A>) -> bool {self.0 != *other}
}

impl<A, B: PartialEq<A>> PartialEq<IdealVec<A>> for IdealVec<B> {
	#[inline]
	fn eq(&self, other: &IdealVec<A>) -> bool {self.0 == other.0}
	#[inline]
	fn ne(&self, other: &IdealVec<A>) -> bool {self.0 != other.0}
}

impl<T: PartialOrd<T>> PartialOrd<Vec<T>> for IdealVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &Vec<T>) -> Option<Ordering> {self.0.partial_cmp(other)}
}

impl<T: PartialOrd<T>> PartialOrd<Self> for IdealVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {self.0.partial_cmp(&other.0)}
}