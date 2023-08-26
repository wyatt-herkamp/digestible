#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
pub use digestible::DigestibleAlloc;

pub mod digester;
pub mod digestible;
pub use digester::Digester;
pub use digestible::Digestible;

/// Expose [byteorder](https://crates.io/crates/byteorder)
pub use byteorder::*;
pub mod digester_writer;
#[cfg(feature = "base64")]
pub mod to_base64;
pub mod type_id;
pub mod hash_digester;

pub use digester_writer::DigestWriter as DigesterWriter;
pub use digester_writer::DigestWriter;
#[cfg(feature = "derive")]
pub use digestible_macros::Digestible;
#[cfg(feature = "base64")]
pub use to_base64::{IntoBase64, ToBase64};
