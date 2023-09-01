#![allow(dead_code, clippy::extra_unused_type_parameters)]
use crate::DigestWriter;
use byteorder::ByteOrder;

/// Instead of the default Digestible implementation for all Array and Vec types of writing each element individually.
/// This will write the entire byte array at once.
pub fn digest_as_bytes<B: ByteOrder, W: DigestWriter>(bytes: impl AsRef<[u8]>, writer: &mut W) {
    writer.write(bytes.as_ref());
}
