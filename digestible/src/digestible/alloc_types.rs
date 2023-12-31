use crate::digestible::internal_macros::{as_ref_then_call_inner, impl_for_as_ref_u8};
use crate::digestible::Digestible;
use crate::DigestWriter;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::{Rc, Weak as WeakRc};
use alloc::string::String;
use alloc::sync::{Arc, Weak as WeakArc};
use alloc::vec::Vec;
use byteorder::ByteOrder;

impl<T: Digestible> Digestible for Vec<T> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        for item in self {
            item.digest::<B, W>(writer)
        }
    }
}
impl_for_as_ref_u8!(String);

impl<D: Digestible> Digestible for Box<D> {
    as_ref_then_call_inner!();
}
impl<T: Digestible> Digestible for Arc<T> {
    as_ref_then_call_inner!();
}
impl<T: Digestible> Digestible for WeakRc<T> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        self.upgrade().digest::<B, W>(writer)
    }
}
impl<T: Digestible> Digestible for WeakArc<T> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        self.upgrade().digest::<B, W>(writer)
    }
}
impl<T: Digestible> Digestible for Rc<T> {
    as_ref_then_call_inner!();
}
impl<T: Digestible + Clone> Digestible for Cow<'_, T> {
    as_ref_then_call_inner!();
}
