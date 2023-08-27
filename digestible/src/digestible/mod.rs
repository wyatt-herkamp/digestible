#[cfg(feature = "alloc")]
mod alloc_types;
#[cfg(feature = "atomic")]
mod atomic;
mod core_types;
mod extern_crates;
mod internal_macros;
#[cfg(feature = "std")]
mod std_types;

use core::hash::Hasher;

use crate::digester_writer::DigestWriter;
use crate::hash_digester::DigesterUsingHasher;
use byteorder::{ByteOrder, NativeEndian};

/// A data type that can be converted into a digest.
pub trait Digestible {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W);
    #[inline(always)]
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        self.digest::<NativeEndian, W>(writer)
    }

    #[doc(hidden)]
    fn hash(&self, hasher: &mut impl Hasher) {
        let mut digester = DigesterUsingHasher(hasher);
        self.digest::<NativeEndian, _>(&mut digester);
    }
}

impl<'a, D: Digestible> Digestible for &'a D {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        (*self).digest::<B, W>(writer)
    }
}
pub trait DigestWith {
    type Digest;
    fn digest<B: ByteOrder, W: DigestWriter>(digest: &Self::Digest, writer: &mut W);
}

pub struct ByteSlice<'a>(pub &'a [u8]);
impl Digestible for ByteSlice<'_> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        writer.write(self.0)
    }
}
