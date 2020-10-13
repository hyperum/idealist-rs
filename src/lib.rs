#![no_std]
#![feature(const_generics, const_evaluatable_checked, generic_associated_types, never_type, arbitrary_self_types, toowned_clone_into, extend_one)]
#![allow(incomplete_features)]

extern crate alloc;

pub mod pointer_class;
pub mod array;
pub mod slice;
pub mod vec;