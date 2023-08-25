mod consts;
mod expand_struct;
mod fields;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Digestible, attributes(digestible))]
pub fn digestible(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // Check if its an enum
    let result = match &input.data {
        syn::Data::Struct(_) => expand_struct::expand(input).into(),
        _ => Err(syn::Error::new_spanned(
            input,
            "rules can only be derived for structs",
        )),
    };
    match result {
        Ok(ok) => ok.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
