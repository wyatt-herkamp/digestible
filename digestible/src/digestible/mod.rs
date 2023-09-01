/*!
# Default Implementation Notes

## Tuple Types
They are written one after another into the digest.

## Floats (f32, f64)
Precision!

Remember that floats are funky
and two floats that are basically same value will not always be the same bytes.

So if you decide to write floats you should be aware of this.
Maybe you should try rounding the floats to a certain precision before writing them.

Or even better round them to an integer.
You can use [digest_with](crate::digest_with::floats) to do this.

## rc::Weak and sync::Weak
These will attempt an upgrade and then call the digest method on the result.
This will just digest the [Option](Option) that is returned.

## Option and Result
These follow the same pattern as the STD library.
No Discriminant is written.

*/
#[cfg(feature = "alloc")]
mod alloc_types;
mod core_types;
mod extern_crates;
mod internal_macros;
#[cfg(feature = "std")]
mod std_types;
mod tuples;

use crate::digester_writer::DigestWriter;
use byteorder::{ByteOrder, NativeEndian};

/// A data type that can be converted into a digest.
pub trait Digestible {
    /// Writes the digest of this value into the given writer.
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W);
    /// Writes the digest of this value into the given writer using the native endian.
    #[inline(always)]
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        self.digest::<NativeEndian, W>(writer)
    }
}

impl<'a, D: Digestible> Digestible for &'a D {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        (*self).digest::<B, W>(writer)
    }
}
