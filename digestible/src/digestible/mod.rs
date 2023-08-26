#[cfg(feature = "alloc")]
mod alloc_types;
mod atomic;
mod core_types;
mod extern_crates;
mod internal_macros;
#[cfg(feature = "std")]
mod std_types;

use core::hash::Hasher;
#[cfg(feature = "alloc")]
pub use alloc_types::DigestibleAlloc;

use crate::digester_writer::DigestWriter;
use byteorder::ByteOrder;
use crate::hash_digester::DigesterUsingHasher;

/// A data type that can be converted into a digest.
pub trait Digestible {
    fn digest_to_writer<W: DigestWriter>(&self, writer: &mut W);

    #[inline(always)]
    fn digest_to_writer_with_order<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        Self::digest_to_writer(self, writer)
    }

    #[doc(hidden)]
    fn hash(&self, hasher: &mut impl Hasher) {
        let mut digester = DigesterUsingHasher(hasher);
        self.digest_to_writer(&mut digester);
    }
}

impl<'a, D: Digestible> Digestible for &'a D {
    #[inline(always)]
    fn digest_to_writer<W: DigestWriter>(&self, writer: &mut W) {
        (*self).digest_to_writer(writer)
    }
    #[inline(always)]
    fn digest_to_writer_with_order<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        (*self).digest_to_writer_with_order::<B, _>(writer)
    }
}
pub trait DigestWith {
    type Digest;
    fn digest<W: DigestWriter>(digest: &Self::Digest, writer: &mut W);
    fn digest_with_order<B: ByteOrder, W: DigestWriter>(digest: &Self::Digest, writer: &mut W) {
        Self::digest::<W>(digest, writer)
    }
}
