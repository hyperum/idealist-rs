use core::ops::Deref;
use alloc::{
	boxed::Box,
	rc::Rc,
	sync::Arc,
};

pub struct WithBox(!);
pub struct WithRc(!);
pub struct WithArc(!);

pub unsafe fn generic_transmute<'a, A, B>(from: A) -> B {
	let to = core::mem::transmute_copy(&from);
	core::mem::forget(from);
	to
}
pub trait PointerClass {
	type Ptr<T: ?Sized>: Sized + Deref<Target = T>;
}

impl PointerClass for WithBox {
	type Ptr<T: ?Sized> = Box<T>;
}

impl PointerClass for WithRc {
	type Ptr<T: ?Sized> = Rc<T>;
}

impl PointerClass for WithArc {
	type Ptr<T: ?Sized> = Arc<T>;
}