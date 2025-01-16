#![allow(dead_code, clippy::extra_unused_type_parameters)]
/*!
# Built-In Digest With Functions


*/
pub mod atomics;
pub mod floats;

use crate::hash_digester::HashableHack;
use crate::DigestWriter;
use byteorder::ByteOrder;
use core::hash::Hash;

/// Due to [RFC 1210](https://github.com/rust-lang/rust/issues/31844)
/// not being stable yet [`Vec<u8>`] and [[u8]]
/// fall under the default implementation to write each byte individually.
/// This will write the entire byte array at once.
///
/// # Example
/// ```
/// use digestible::{Digester, Digestible};
/// use sha2::{Digest, Sha256};
/// #[derive(Digestible, Default)]
/// pub struct MyStruct {
///     #[digestible(digest_with = digest_as_bytes)]
///     pub bytes: Box<[u8]>,
///     #[digestible(digest_with = digest_as_bytes)]
///     pub vec: Vec<u8>,
///     #[digestible(digest_with = digest_as_bytes)]
///     pub array: [u8; 4],
/// }
///
/// let mut hasher = sha2::Sha256::new();
/// let result = hasher.digest_native(&MyStruct::default());
/// assert_eq!(result.len(), 32);
/// ```
pub fn digest_as_bytes<B: ByteOrder, W: DigestWriter>(bytes: impl AsRef<[u8]>, writer: &mut W) {
    writer.write(bytes.as_ref());
}

/// Takes a type that implements [Hash](core::hash::Hash) and writes it to the given writer.
/// This is useful for types that do not implement [Digestible](crate::Digestible).
///
/// See [HashableHack](crate::hash_digester::HashableHack) for more information.
/// # Example
/// ```
/// use digestible::{Digester, Digestible};
/// use sha2::{Digest, Sha256};
/// use std::time::Duration;
/// #[derive(Digestible, Default)]
/// pub struct MyStruct {
///     #[digestible(digest_with = digest_with_hash)]
///     pub unit: (),
///     #[digestible(digest_with = digest_with_hash)]
///     pub optional_duration: Option<Duration>,
/// }
///
/// let mut hasher = sha2::Sha256::new();
/// let result = hasher.digest_native(&MyStruct::default());
/// assert_eq!(result.len(), 32);
/// ```
pub fn digest_with_hash<B: ByteOrder, W: DigestWriter>(hash: &impl Hash, writer: &mut W) {
    let mut hashable_hack = HashableHack(writer);
    hash.hash(&mut hashable_hack);
}

/// Takes a type that implements [AsRef]<[str]> and writes it to the given writer.
/// # Example
/// ```
/// use digestible::{Digester, Digestible};
/// use sha2::{Digest, Sha256};
/// #[derive(Default)]
/// pub struct MyStrRefType(pub String);
/// impl core::convert::AsRef<str> for MyStrRefType {
///     fn as_ref(&self) -> &str {
///         self.0.as_str()
///     }
/// }
/// #[derive(Digestible, Default)]
/// pub struct MyStruct {
///     #[digestible(digest_with = digest_as_str_ref)]
///     pub my_item: MyStrRefType,
/// }
///
/// let mut hasher = sha2::Sha256::new();
/// let result = hasher.digest_native(&MyStruct::default());
/// assert_eq!(result.len(), 32);
/// ```
pub fn digest_as_str_ref<B: ByteOrder, W: DigestWriter>(hash: &impl AsRef<str>, writer: &mut W) {
    writer.write_str(hash.as_ref());
}
