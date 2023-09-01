#![doc(html_root_url = "https:///docs.rs/digestible/latest/digestible/")]
/*!
# Digestible
A more dynamic [Hash](core::hash::Hash) and [Hasher](core::hash::Hasher) trait for Rust
## Key Difference.
- ByteOrder is built in. So you can digest number types in any byte order.
- Output is a Generic Associated Type.
So you can Digest into a ByteArray, String with [Base64](ToBase64)
or any other that the Digester implements.
## Key Types
#### [Digestible](digestible::Digestible)
A trait that allows you to digest data into a [Digester](digester::Digester)
Equivalent to [Hash](core::hash::Hash) but with more control over the digesting process

#### [Digester]
A trait that allows you to digest data into a Target type
Equivalent to [Hasher](core::hash::Hasher) but with more control over the digesting process

#### [DigestWriter](DigestWriter)
A trait that allows you to write data to a writer.
This is used internally by [Digester] this is what is passed into [Digestible] to digest data

#### [Digestible Macro](digestible_macros::Digestible)
A macro that allows you to automatically implement [Digestible] for your types
Similar to [derive(Hash)](core::hash::Hash) but with more control over the digesting process

## Type Headers
Type Header is a way to identify complex type when being digested.
Preventing collisions between similar types.
This is generated by the macro by putting the [type_name](core::any::type_name)
of the type in the digest.

### Enums
The Type Header for Enum is written as [type_name](core::any::type_name)::`variant_name`
*/
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::from_over_into)]
#![deny(missing_docs)]
#[cfg(feature = "alloc")]
extern crate alloc;
#[doc(hidden)]
pub mod digester;
pub mod digestible;
#[doc(inline)]
pub use crate::digestible::Digestible;
#[doc(inline)]
pub use digester::Digester;
#[doc(hidden)]
pub mod digester_writer;
#[cfg(feature = "base64")]
#[doc(hidden)]
pub mod to_base64;
#[doc(inline)]
pub use byteorder;
/// Provides some sometimes useful digest_with implementations
pub mod digest_with;

/// Provides inter-compatibility with [Hasher](core::hash::Hasher)/[Hash](core::hash::Hash) and [Digester](crate::Digester)/[Digestible](crate::Digestible)
pub mod hash_digester;

#[doc(inline)]
pub use digester_writer::DigestWriter;
#[cfg(feature = "derive")]
///# Digestible Macro
///
/// Implement the [Digestible] trait for the given struct or enum.
///
/// This will push all data one after another into the [DigestWriter].
///
/// No padding or spaces are added. Similar to how [Hash](core::hash::Hash) works.
///
///
/// ## Container Attributes
/// ### type_header
/// Sets how the type header is written [TypeHeader](https:///docs.rs/digestible/latest/digestible/index.html#type-headers)
/// Options:
/// - none: No type header is written `#[digestible(type_header = none)]`
/// - HashName: The name of the hash is written as the type header (Default) `#[digestible(type_header = HashName)]`
/// ### impl_hash
/// The macro will also implement [Hash](core::hash::Hash) for the given struct or enum using [DigesterUsingHasher](hash_digester::DigesterUsingHasher).
/// This will put the same data the Digestible trait would into the hasher. Allowing you to use `digest_with` and including type headers.
/// This will not be useful if you are using this type in a HashMap or HashSet. As this will provide more data than the Hash trait would. and can be slower.
/// By default this uses NativeEndian.
/// You can change this by using
/// - `#[digestible(impl_hash = LittleEndian)]`
/// - `#[digestible(impl_hash = BigEndian)]`
/// - `#[digestible(impl_hash = NetworkEndian)]`
/// - `#[digestible(impl_hash = NativeEndian)]`
/// #### Output
///```rust
/// use digestible_macros::Digestible;
/// #[derive(Digestible)]
/// // Add this #[digestible(impl_hash = LittleEndian)]
/// pub struct MyStruct {
///     pub id: u32,
/// }
/// // This will be generated
/// impl core::hash::Hash for MyStruct {
///     fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
///         let mut digester = digestible::hash_digester::DigesterUsingHasher(state);
///         <Self as digestible::Digestible>::digest::<byteorder::LittleEndian, _>(
///             self,
///             &mut digester,
///         );
///     }
/// }
/// ```
/// ## Field Attributes
/// ### skip: Skips the field when digesting
/// ### with: Path to a digest fn
/// Required Fn Signature: `fn digest<B: ByteOrder, W: DigestWriter>(digest: Type, writer: &mut W);`
///
/// ```rust, no_run
/// use core::time::Duration;
/// use digestible::byteorder::ByteOrder;
/// use digestible::hash_digester::HashableHack;
/// use digestible::{DigestWriter, Digestible};
///
/// fn duration_digest_with<B: ByteOrder, W: DigestWriter>(digest: &Duration, writer: &mut W) {
///     writer.write_u64::<B>(digest.as_secs());
/// }
/// ```
/// ### digest_with
/// Function provided in the [digest_with](crate::digest_with) module Example: `#[digestible(digest_with = digest_with_hash)]`
pub use digestible_macros::Digestible;
#[cfg(feature = "base64")]
#[doc(inline)]
pub use to_base64::{IntoBase64, ToBase64};
