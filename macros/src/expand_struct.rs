use crate::consts::digest_path;
use crate::fields::Field;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Result;
use syn::{DeriveInput, Error};

pub(crate) fn expand(derive_input: DeriveInput) -> Result<TokenStream> {
    let name = &derive_input.ident;
    let as_struct = match derive_input.data {
        syn::Data::Struct(s) => s,
        _ => {
            return Err(Error::new_spanned(
                derive_input,
                "digestible can only be derived for structs",
            ))
        }
    };
    let mut fields = Vec::with_capacity(as_struct.fields.len());

    for (index, field) in as_struct.fields.into_iter().enumerate() {
        let field = Field::new(field.clone(), index)?;
        fields.push(field);
    }
    let field_names: Vec<_> = fields.iter().map(|v| v.ident.clone()).collect();
    let digestible = digest_path();
    let writer = format_ident!("writer");
    let order = format_ident!("B");
    let write_fields_order: Vec<_> = fields
        .iter()
        .map(|v| v.digest_with_order(&order, &writer))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect();
    let result = quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate digestible as _digestible;
            #[automatically_derived]
            impl #digestible for #name {
                fn digest<#order: _digestible::ByteOrder, W: _digestible::DigestWriter>(
                    &self,
                    writer: &mut W,
                ) {
                    let Self { #(#field_names),* } = self;
                    #(#write_fields_order)*
                }
            }
        };
    };

    Ok(result)
}
