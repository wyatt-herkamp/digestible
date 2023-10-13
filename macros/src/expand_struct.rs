use crate::paths::{digest_writer, digestible_path, private_path};
use crate::container_attrs::{get_container_attrs, ContainerAttrs, TypeHeader};
use crate::fields::Field;
use crate::shared;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Path};
use syn::{Fields, Result};

pub(crate) fn expand(derive_input: DeriveInput) -> Result<TokenStream> {
    let name = &derive_input.ident;
    let container_attrs = get_container_attrs!(derive_input);
    let syn::Data::Struct(as_struct) = derive_input.data else {
        unreachable!("digestible can only be derived for enums (expand_struct.rs)")
    };

    let mut fields = Vec::with_capacity(as_struct.fields.len());
    let writer = format_ident!("writer");
    let order = format_ident!("B");
    for (index, field) in as_struct.fields.iter().enumerate() {
        let field = Field::new(field.clone(), index, &order, &writer)?;
        fields.push(field);
    }
    let field_names: Vec<_> = fields.iter().map(|v| &v.ident).collect();

    let expand_fields = match &as_struct.fields {
        Fields::Named(_) => {
            quote! {
                let Self { #(#field_names),* } = self;
            }
        }
        Fields::Unnamed(_) => {
            quote! {
                let Self(#(#field_names),*) = self;
            }
        }
        Fields::Unit => {
            quote! {}
        }
    };

    let digest_writer = digest_writer();

    let header_write = match container_attrs.type_header {
        TypeHeader::None => quote! {},
        TypeHeader::HashName => {
            let type_name : Path = private_path!(type_name);
            quote! {
                #digest_writer::write(writer, #type_name::<Self>().as_bytes());
            }
        }
        TypeHeader::TypeId { .. } => {
            todo!("type_id")
        }
    };
    let byte_order_path = crate::paths::byte_order_path();
    let impl_hash = if let Some(impl_hash) = container_attrs.impl_hash {
        shared::impl_hash(name, impl_hash)
    } else {
        quote! {}
    };

    let digestible = digestible_path();
    let result = quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate digestible as _digestible;
            #[automatically_derived]
            impl #digestible for #name {
                fn digest<#order: #byte_order_path, W: _digestible::DigestWriter>(
                    &self,
                    writer: &mut W,
                ) {
                    #header_write
                    #expand_fields
                    #(#fields)*
                }
            }
            #impl_hash
        };
    };

    Ok(result)
}
