#![no_std]
#![feature(
    const_generics,
    const_evaluatable_checked,
    generic_associated_types,
    never_type,
    arbitrary_self_types,
    toowned_clone_into,
    extend_one
)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(incomplete_features, clippy::module_name_repetitions)]

extern crate alloc;

pub mod array;
pub mod pointer_class;
pub mod slice;
pub mod vec;
