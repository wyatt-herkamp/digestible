mod alloc_types;
mod atomic;
mod extern_crates;
pub mod hash_hack;
mod internal_macros;
mod num_types;

use byteorder::ByteOrder;
use core::hash::Hasher;
use std::io::Write;
/// A data type that can be converted into a digest.
pub trait Digestible {
    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;

    #[inline(always)]
    fn digest_owned(&self) -> Vec<u8> {
        let mut digest = Vec::new();
        self.digest_to_writer(&mut digest).unwrap();
        digest
    }

    #[inline(always)]
    fn digest_owned_with_order<B: ByteOrder>(&self) -> Vec<u8> {
        let mut digest = Vec::new();
        self.digest_to_writer_with_order::<B, _>(&mut digest)
            .unwrap();
        digest
    }

    #[inline(always)]
    fn digest_to_writer_with_order<B: ByteOrder, W: Write>(
        &self,
        writer: &mut W,
    ) -> std::io::Result<()> {
        Self::digest_to_writer(self, writer)
    }

    fn digest_to_hasher(&self, hasher: &mut impl Hasher) {
        let digest_owned = self.digest_owned();
        hasher.write(&digest_owned);
    }
}
impl<'a, D: Digestible> Digestible for &'a D {
    #[inline(always)]
    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        (*self).digest_to_writer(writer)
    }
    #[inline(always)]
    fn digest_owned(&self) -> Vec<u8> {
        (*self).digest_owned()
    }
    #[inline(always)]
    fn digest_owned_with_order<B: ByteOrder>(&self) -> Vec<u8> {
        (*self).digest_owned_with_order::<B>()
    }
    #[inline(always)]
    fn digest_to_writer_with_order<B: ByteOrder, W: Write>(
        &self,
        writer: &mut W,
    ) -> std::io::Result<()> {
        (*self).digest_to_writer_with_order::<B, _>(writer)
    }
}
pub trait DigestWith {
    type Digest;
    fn digest<W: Write>(digest: &Self::Digest, writer: &mut W) -> std::io::Result<()>;
    fn digest_with_order<B: ByteOrder, W: Write>(
        digest: &Self::Digest,
        writer: &mut W,
    ) -> std::io::Result<()> {
        Self::digest::<W>(digest, writer)
    }
}
