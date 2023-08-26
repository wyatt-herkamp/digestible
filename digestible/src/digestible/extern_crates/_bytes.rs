use crate::digestible::internal_macros::{impl_for_as_ref_u8};
use crate::{DigestWriter, Digestible};
use bytes::{Bytes, BytesMut};

impl_for_as_ref_u8!(Bytes);
impl_for_as_ref_u8!(BytesMut);

impl DigestWriter for BytesMut {
    fn write(&mut self, data: &[u8]) {
        self.extend_from_slice(data);
    }
}
