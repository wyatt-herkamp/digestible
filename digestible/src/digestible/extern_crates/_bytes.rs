use crate::digestible::internal_macros::impl_for_as_ref_u8;
use bytes::{Bytes, BytesMut};

impl_for_as_ref_u8!(Bytes);
impl_for_as_ref_u8!(BytesMut);
