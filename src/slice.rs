use core::{
	cmp::Ordering,
	fmt::{
		self,
		Debug,
	},
	hash::{
		self,
		Hash,
	},
	num::NonZeroUsize,
	ops::{
		Index,
		IndexMut,
	},
	slice::{
		Iter,
		IterMut,
		SliceIndex,
	},
};
use alloc::{
	borrow::{
		Cow,
		ToOwned
	},
	boxed::Box,
	rc::Rc,
	sync::Arc,
	vec::Vec,
};
use super::{
	array::*,
	pointer_class::*,
	vec::*,
};

pub struct IdealSlice<T>(pub(crate) [T]);

impl<T> IdealSlice<T> {
	#[inline]
	pub unsafe fn new_ref_unchecked<'a>(slice: &'a [T]) -> &'a Self {
		generic_transmute(slice)
	}

	#[inline]
	pub unsafe fn new_mut_unchecked<'a>(slice: &'a mut [T]) -> &'a mut Self {
		generic_transmute(slice)
	}

	#[inline]
	pub unsafe fn new_ptr_unchecked<P: PointerClass>(slice: P::Ptr<[T]>) -> P::Ptr<Self> {
		generic_transmute(slice)
	}
	
	#[inline]
	pub unsafe fn new_cow_unchecked<'a>(slice: Cow<'a, [T]>) -> Cow<'a, Self> where [T]: ToOwned, Self: ToOwned {
		generic_transmute(slice)
	}

	#[inline]
	pub fn new_ref<'a>(slice: &'a [T]) -> Option<&'a Self> {
		if slice.len() != 0 {
			unsafe {Some(generic_transmute(slice))}
		} else {
			None
		}
	}

	#[inline]
	pub fn new_mut<'a>(slice: &'a mut [T]) -> Option<&'a mut Self> {
		if slice.len() != 0 {
			unsafe {Some(generic_transmute(slice))}
		} else {
			None
		}
	}

	#[inline]
	pub fn new_ptr<'a, P: PointerClass>(slice: P::Ptr<[T]>) -> Option<P::Ptr<Self>> {
		if slice.len() != 0 {
			unsafe {Some(generic_transmute(slice))}
		} else {
			None
		}
	}

	#[inline]
	pub fn new_cow<'a>(slice: Cow<'a, [T]>) -> Option<Cow<'a, Self>> where [T]: ToOwned, Self: ToOwned {
		if slice.len() != 0 {
			unsafe {Some(generic_transmute(slice))}
		} else {
			None
		}
	}

	#[inline]
	pub fn get_ref<'a>(self: &'a Self) -> &'a [T] {
		unsafe {generic_transmute(self)}
	}

	#[inline]
	pub fn get_mut<'a>(self: &'a mut Self) -> &'a mut [T] {
		unsafe {generic_transmute(self)}
	}

	#[inline]
	pub fn get_ptr<'a, P: PointerClass>(self: P::Ptr<Self>) -> P::Ptr<[T]> {
		unsafe {generic_transmute(self)}
	}
	
	#[inline]
	pub unsafe fn get_cow<'a>(self: Cow<'a, Self>) -> Cow<'a, [T]> where [T]: ToOwned, Self: ToOwned {
		generic_transmute(self)
	}

	#[inline]
	pub const fn len(&self) -> NonZeroUsize {
		unsafe {NonZeroUsize::new_unchecked(self.0.len())}
	}

	pub fn to_ideal_vec(&self) -> IdealVec<T> where T: Clone {
		IdealVec(self.0.to_vec())
	}

	pub fn into_ideal_vec(self: Box<Self>) -> IdealVec<T> {
		IdealVec(self.get_ptr::<WithBox>().into_vec())
	}
}

impl<T> AsMut<[T]> for IdealSlice<T> {
	fn as_mut(&mut self) -> &mut [T] {&mut self.0}
}

impl<T> AsMut<Self> for IdealSlice<T> {
	fn as_mut(&mut self) -> &mut Self {self}
}

impl<T> AsRef<[T]> for IdealSlice<T> {
	fn as_ref(&self) -> &[T] {&self.0}
}

impl<T> AsRef<Self> for IdealSlice<T> {
	fn as_ref(&self) -> &Self {self}
}

// TODO: Implement Concat (slice_concat_trait)

impl<T: Debug> Debug for IdealSlice<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {self.0.fmt(f)}
}

impl<T: Eq> Eq for IdealSlice<T> {}

impl<T> From<IdealVec<T>> for Arc<IdealSlice<T>> {
	#[inline]
	fn from(v: IdealVec<T>) -> Self {
		unsafe {IdealSlice::new_ptr_unchecked::<WithArc>(Arc::from(v.0))}
	}
}

impl<T> From<IdealVec<T>> for Box<IdealSlice<T>> {
	#[inline]
	fn from(v: IdealVec<T>) -> Self {
		unsafe {IdealSlice::new_ptr_unchecked::<WithBox>(Box::from(v.0))}
	}
}

impl<'a, T: Clone> From<&'a IdealVec<T>> for Cow<'a, IdealSlice<T>> {
	fn from(v: &'a IdealVec<T>) -> Self {
		unsafe {IdealSlice::new_cow_unchecked(Cow::from(&v.0))}
	}
}

impl<T> From<IdealVec<T>> for Rc<IdealSlice<T>> {
	#[inline]
	fn from(v: IdealVec<T>) -> Self {
		unsafe {IdealSlice::new_ptr_unchecked::<WithRc>(Rc::from(v.0))}
	}
}

impl<T: Hash> Hash for IdealSlice<T> {
	fn hash<H: hash::Hasher>(&self, state: &mut H) {self.0.hash(state)}
}

impl<T, I: SliceIndex<[T]>> Index<I> for IdealSlice<T>
{
	type Output = I::Output;
	#[inline]
	fn index(&self, index: I) -> &I::Output {self.0.index(index)}
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for IdealSlice<T>
{
	#[inline]
	fn index_mut(&mut self, index: I) -> &mut I::Output {self.0.index_mut(index)}
}

impl<'a, T> IntoIterator for &'a IdealSlice<T> {
	type Item = &'a T;
	type IntoIter = Iter<'a, T>;
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}

impl<'a, T> IntoIterator for &'a mut IdealSlice<T> {
	type Item = &'a mut T;
	type IntoIter = IterMut<'a, T>;
	fn into_iter(self) -> Self::IntoIter {
		self.0.iter_mut()
	}
}

// TODO: Implement Join (slice_concat_trait)

impl<T: Ord> Ord for IdealSlice<T> {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {self.0.cmp(&other.0)}
}

impl<const N: usize, A, B: PartialEq<A>> PartialEq<[A; N]> for IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &[A; N]) -> bool {self.0 == *other}
	#[inline]
	fn ne(&self, other: &[A; N]) -> bool {self.0 != *other}
}

impl<const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<IdealArray<A, N>> for IdealSlice<B> where [A; N.get()]: Sized {
	#[inline]
	fn eq(&self, other: &IdealArray<A, N>) -> bool {self.0 == other.0}
	#[inline]
	fn ne(&self, other: &IdealArray<A, N>) -> bool {self.0 != other.0}
}

impl<'b, const N: usize, A, B: PartialEq<A>> PartialEq<[A; N]> for &'b IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &[A; N]) -> bool {self.0 == *other}
	#[inline]
	fn ne(&self, other: &[A; N]) -> bool {self.0 != *other}
}

impl<'b, const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<IdealArray<A, N>> for &'b IdealSlice<B> where [A; N.get()]: Sized {
	#[inline]
	fn eq(&self, other: &IdealArray<A, N>) -> bool {self.0 == other.0}
	#[inline]
	fn ne(&self, other: &IdealArray<A, N>) -> bool {self.0 != other.0}
}

impl<'b, const N: usize, A, B: PartialEq<A>> PartialEq<[A; N]> for &'b mut IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &[A; N]) -> bool {self.0 == *other}
	#[inline]
	fn ne(&self, other: &[A; N]) -> bool {self.0 != *other}
}

impl<'b, const N: NonZeroUsize, A, B: PartialEq<A>> PartialEq<IdealArray<A, N>> for &'b mut IdealSlice<B> where [A; N.get()]: Sized {
	#[inline]
	fn eq(&self, other: &IdealArray<A, N>) -> bool {self.0 == other.0}
	#[inline]
	fn ne(&self, other: &IdealArray<A, N>) -> bool {self.0 != other.0}
}

impl<A, B: PartialEq<A>> PartialEq<[A]> for IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &[A]) -> bool {self.0 == *other}
	#[inline]
	fn ne(&self, other: &[A]) -> bool {self.0 != *other}
}

impl<A, B: PartialEq<A>> PartialEq<IdealSlice<A>> for IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &IdealSlice<A>) -> bool {self.0 == other.0}
	#[inline]
	fn ne(&self, other: &IdealSlice<A>) -> bool {self.0 != other.0}
}

impl<A, B: PartialEq<A>> PartialEq<Vec<A>> for &IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &Vec<A>) -> bool {&self.0 == *other}
	#[inline]
	fn ne(&self, other: &Vec<A>) -> bool {&self.0 != *other}
}

impl<A, B: PartialEq<A>> PartialEq<IdealVec<A>> for &IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &IdealVec<A>) -> bool {&self.0 == other.0}
	#[inline]
	fn ne(&self, other: &IdealVec<A>) -> bool {&self.0 != other.0}
}

impl<A, B: PartialEq<A>> PartialEq<Vec<A>> for &mut IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &Vec<A>) -> bool {&self.0 == *other}
	#[inline]
	fn ne(&self, other: &Vec<A>) -> bool {&self.0 != *other}
}

impl<A, B: PartialEq<A>> PartialEq<IdealVec<A>> for &mut IdealSlice<B> {
	#[inline]
	fn eq(&self, other: &IdealVec<A>) -> bool {&self.0 == other.0}
	#[inline]
	fn ne(&self, other: &IdealVec<A>) -> bool {&self.0 != other.0}
}

impl<T: PartialOrd<T>> PartialOrd<[T]> for IdealSlice<T> {
	#[inline]
	fn partial_cmp(&self, other: &[T]) -> Option<Ordering> {self.0.partial_cmp(other)}
}

impl<T: PartialOrd<T>> PartialOrd<Self> for IdealSlice<T> {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {self.0.partial_cmp(&other.0)}
}

// TODO: Implement Pattern (pattern)

impl<T: Clone> ToOwned for IdealSlice<T> {
	type Owned = IdealVec<T>;
	fn to_owned(&self) -> Self::Owned {self.to_ideal_vec()}
	fn clone_into(&self, target: &mut IdealVec<T>) {self.0.clone_into(&mut target.0)}
}