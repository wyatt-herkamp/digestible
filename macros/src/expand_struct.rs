use crate::utils::{digest_writer, digestible_path, private_path};
use crate::container_attrs::{get_container_attrs, TypeHeader};
use crate::fields::Field;
use crate::{utils};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Path};
use syn::{Fields, Result};

pub(crate) fn expand(derive_input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput{ attrs, ident, mut generics, data,.. } = derive_input;
    utils::add_digestible_trait(&mut generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let container_attrs = get_container_attrs(&attrs)?;
    let syn::Data::Struct(as_struct) = data else {
        // This is checked before
        unsafe{
            std::hint::unreachable_unchecked();
        }
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
    let byte_order_path = utils::byte_order_path();
    let impl_hash = if let Some(impl_hash) = container_attrs.impl_hash {
        utils::impl_hash(&ident, impl_hash, &impl_generics, &ty_generics, &where_clause)
    } else {
        quote! {}
    };

    let digestible = digestible_path();
    let result = quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate digestible as _digestible;
            #[automatically_derived]
            impl  #impl_generics  #digestible for #ident #ty_generics #where_clause {
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
