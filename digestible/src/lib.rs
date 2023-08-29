#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::from_over_into)]
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod digester;
pub mod digestible;
pub use crate::digestible::Digestible;
pub use digester::Digester;

/// Expose [byteorder](https://crates.io/crates/byteorder)
pub use byteorder::*;
pub mod digester_writer;
pub mod hash_digester;
#[cfg(feature = "base64")]
pub mod to_base64;

pub use digester_writer::DigestWriter;
#[cfg(feature = "derive")]
pub use digestible_macros::Digestible;
#[cfg(feature = "base64")]
pub use to_base64::{IntoBase64, ToBase64};
