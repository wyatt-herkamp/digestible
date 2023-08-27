mod consts;
mod expand_struct;
mod fields;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Implements the [Digestible](crate::Digestible) trait for the given struct.
///
/// This will push all data one after another into the digest.
/// No padding or spaces are added.
///
/// ## Field Attributes
/// - skip: Skips the field when digesting
/// - use_std_hash: Uses [HashableHack](digestible::digestible::hash_hack::HashableHack)
/// to digest the field
/// - digest_with: Path to Type that implements [DigestWith](digestible::digestible::DigestWith)
#[proc_macro_derive(Digestible, attributes(digestible))]
pub fn digestible(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Check if its an enum
    let result = match &input.data {
        syn::Data::Struct(_) => expand_struct::expand(input),
        _ => Err(syn::Error::new_spanned(
            input,
            "digestible can only be derived for structs",
        )),
    };
    match result {
        Ok(ok) => ok.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
