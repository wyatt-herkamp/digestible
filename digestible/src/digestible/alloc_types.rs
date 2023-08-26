use crate::digestible::internal_macros::{as_ref_then_call_inner, as_ref_then_call_inner_alloc, digest_owned_with_size, impl_alloc_with_size_call, impl_for_as_ref_u8, use_hasher};
use crate::digestible::Digestible;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use byteorder::ByteOrder;
use core::hash::Hasher;
pub trait DigestibleAlloc: Digestible {
    #[inline(always)]
    fn digest_owned(&self) -> Vec<u8> {
        let mut digest = Vec::new();
        self.digest_to_writer(&mut digest);
        digest
    }

    #[inline(always)]
    fn digest_owned_with_order<B: ByteOrder>(&self) -> Vec<u8> {
        let mut digest = Vec::new();
        self.digest_to_writer_with_order::<B, _>(&mut digest);
        digest
    }
}

impl<'a> DigestibleAlloc for &'a [u8] {
    digest_owned_with_size!(
        fn size(this: &[u8]) -> usize {
            this.len()
        }
    );
}

impl_for_as_ref_u8!(Vec<u8>);
impl_alloc_with_size_call!(Vec<u8>, fn size(this: &Vec<u8>) -> usize {
    this.len()
});
impl_for_as_ref_u8!(String);
impl_alloc_with_size_call!(String, fn size(this: &String) -> usize {
    this.len()
});

#[cfg(test)]
mod tests {
    use crate::DigestibleAlloc;
    use alloc::string::ToString;

    #[test]
    pub fn test() {
        let s = "hello world".to_string();
        let d = s.digest_owned();
        assert_eq!(d, "hello world".as_bytes());
    }
    #[test]
    pub fn test_byte_array() {
        let s = "hello world".as_bytes();
        let d = s.digest_owned();
        assert_eq!(d, "hello world".as_bytes());
    }
}
impl<D: Digestible> Digestible for Box<D> {
    as_ref_then_call_inner!();
}
impl<D: DigestibleAlloc> DigestibleAlloc for Box<D> {
    as_ref_then_call_inner_alloc!();
}

impl<T: Digestible> Digestible for alloc::sync::Arc<T> {
    as_ref_then_call_inner!();
}
impl<T: DigestibleAlloc> DigestibleAlloc for alloc::sync::Arc<T> {
    as_ref_then_call_inner_alloc!();
}
