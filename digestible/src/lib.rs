#![doc(html_root_url = "https://docs.rs/digestible/latest/digestible/")]
/*!
# Digestible
A more dynamic [Hash](core::hash::Hash) and [Hasher](core::hash::Hasher) trait for Rust

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
#[cfg(feature = "alloc")]
extern crate alloc;
#[doc(hidden)]
pub mod digester;
#[doc(hidden)]
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
#[doc = include_str!("digestible-macro.md")]
pub use digestible_macros::Digestible;
#[cfg(feature = "base64")]
#[doc(inline)]
pub use to_base64::{IntoBase64, ToBase64};
