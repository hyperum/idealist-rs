#![no_std]
#![feature(const_generics, const_evaluatable_checked, generic_associated_types, never_type, const_fn, const_trait_impl, arbitrary_self_types, const_option, toowned_clone_into, dropck_eyepatch, extend_one)]
#![allow(incomplete_features)]

extern crate alloc;

pub mod pointer_class;
pub mod array;
pub mod slice;
pub mod vec;