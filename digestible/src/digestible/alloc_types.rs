use crate::digestible::internal_macros::{as_ref_then_call_inner, impl_for_as_ref_u8};
use crate::digestible::Digestible;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use std::sync::Arc;

impl_for_as_ref_u8!(Vec<u8>);
impl_for_as_ref_u8!(String);

impl<D: Digestible> Digestible for Box<D> {
    as_ref_then_call_inner!();
}
impl<T: Digestible> Digestible for Arc<T> {
    as_ref_then_call_inner!();
}
