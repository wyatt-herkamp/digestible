use crate::digestible::internal_macros::{
    as_ref_then_call_inner, impl_for_as_ref_u8, impl_for_hashable_hack,
};
use crate::digestible::Digestible;
use byteorder::ByteOrder;
use std::ffi::{OsStr, OsString};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

impl<'b> Digestible for &'b [u8] {
    #[inline(always)]
    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(self)
    }
    #[inline(always)]
    fn digest_owned(&self) -> Vec<u8> {
        self.to_vec()
    }
    fn digest_owned_with_order<B: ByteOrder>(&self) -> Vec<u8> {
        self.to_vec()
    }
}
impl<D: Digestible> Digestible for Box<D> {
    as_ref_then_call_inner!();
}
impl<'str> Digestible for &'str str {
    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.as_bytes())
    }
    crate::digestible::internal_macros::digest_owned_with_size!(
        fn size(this: &str) -> usize {
            this.as_bytes().len()
        }
    );
}

impl_for_as_ref_u8!(Vec<u8>);
impl_for_as_ref_u8!(String);

impl_for_hashable_hack!(OsStr);
impl_for_hashable_hack!(OsString);
impl_for_hashable_hack!(PathBuf);
impl_for_hashable_hack!(Path);

#[cfg(test)]
mod tests {
    #[test]
    pub fn test() {
        use crate::digestible::Digestible;
        let s = "hello world".to_string();
        let d = s.digest_owned();
        assert_eq!(d, "hello world".as_bytes());
    }
    #[test]
    pub fn test_byte_array() {
        use crate::digestible::Digestible;
        let s = "hello world".as_bytes();
        let d = s.digest_owned();
        assert_eq!(d, "hello world".as_bytes());
    }
}
