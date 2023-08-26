use crate::digestible::internal_macros::impl_for_as_ref_u8;
use crate::Digestible;
use bytes::{Bytes, BytesMut};
use std::io::Write;

impl_for_as_ref_u8!(Bytes);
impl_for_as_ref_u8!(BytesMut);
