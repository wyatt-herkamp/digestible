use crate::container_attrs::{get_container_attrs, TypeHeader};
use crate::fields::Field;
use crate::utils;
use crate::utils::{digest_writer, digestible_path, private_path};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::Result;
use syn::{DeriveInput, Path};

pub enum EnumType {
    Unit,
    Tuple,
    Struct,
}
pub struct Variant<'a> {
    pub ident: syn::Ident,
    pub fields: Vec<Field<'a>>,
    #[allow(dead_code)]
    pub endian: &'a Ident,
    pub writer: &'a Ident,
    pub enum_type: EnumType,
}

impl<'a> Variant<'a> {
    pub fn new(variant: syn::Variant, endian: &'a Ident, writer: &'a Ident) -> syn::Result<Self> {
        let mut fields: Vec<Field<'a>> = Vec::with_capacity(variant.fields.len());
        for (index, field) in variant.fields.iter().enumerate() {
            let field = Field::new(field.clone(), index, endian, writer)?;
            fields.push(field);
        }
        let enum_type = match &variant.fields {
            syn::Fields::Named(_) => EnumType::Struct,
            syn::Fields::Unnamed(_) => EnumType::Tuple,
            syn::Fields::Unit => EnumType::Unit,
        };
        Ok(Self {
            ident: variant.ident,
            fields,
            endian,
            writer,
            enum_type,
        })
    }
    pub fn catch_block(&self, enum_name: &Ident) -> TokenStream {
        let ident = &self.ident;
        let writer = self.writer;
        let fields: Vec<_> = self.fields.iter().map(|v| &v.ident).collect();
        let fn_name = format_ident!("digest_{}", self.ident);
        match self.enum_type {
            EnumType::Unit => {
                quote! {
                    #enum_name::#ident => {
                        #fn_name(#writer);
                    }
                }
            }
            EnumType::Tuple => {
                quote! {
                    #enum_name::#ident(#(#fields),*) => {
                        #fn_name(#writer, #(#fields),*);
                    }
                }
            }
            EnumType::Struct => {
                quote! {
                    #enum_name::#ident{#(#fields),*} => {
                        #fn_name(#writer, #(#fields),*);
                    }
                }
            }
        }
    }
}
impl ToTokens for Variant<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fn_name = format_ident!("digest_{}", self.ident);
        let fields_def: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|v| {
                let ty = &v.ty;
                let ident = &v.ident;
                quote! {
                    #ident: &#ty
                }
            })
            .collect();
        let fields = &self.fields;
        let digest_writer = digest_writer();
        let ident = &self.ident;
        let writer = self.writer;
        let result = quote! {
            let #fn_name = |#writer: &mut W, #(#fields_def),*| {
                #digest_writer::write(writer, stringify!(#ident).as_bytes());
                #(#fields)*
            };
        };
        tokens.extend(result);
    }
}
pub(crate) fn expand(derive_input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput {
        attrs,
        ident,
        mut generics,
        data,
        ..
    } = derive_input;
    let syn::Data::Enum(as_enum) = data else {
        //Checked before
        unsafe {
            std::hint::unreachable_unchecked();
        }
    };
    utils::add_digestible_trait(&mut generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let container_attrs = get_container_attrs(&attrs)?;

    let writer = format_ident!("writer");
    let order = format_ident!("B");
    let digest_writer = digest_writer();
    let header_write = match container_attrs.type_header {
        TypeHeader::None => quote! {},
        TypeHeader::HashName => {
            let type_name: Path = private_path!(type_name);

            quote! {
                #digest_writer::write(writer, #type_name::<Self>().as_bytes());
                #digest_writer::write(writer, b"::");
            }
        }
        TypeHeader::TypeId { .. } => {
            todo!("type_id")
        }
    };
    let mut variants = Vec::with_capacity(as_enum.variants.len());
    for variant in as_enum.variants {
        let variant = Variant::new(variant, &order, &writer)?;
        variants.push(variant);
    }
    let catch_block: Vec<_> = variants.iter().map(|v| v.catch_block(&ident)).collect();
    let digestible = digestible_path();
    let byte_order_path = crate::utils::byte_order_path();
    let impl_hash = if let Some(impl_hash) = container_attrs.impl_hash {
        utils::impl_hash(
            &ident,
            impl_hash,
            &impl_generics,
            &ty_generics,
            &where_clause,
        )
    } else {
        quote! {}
    };
    let result = quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate digestible as _digestible;
            #[automatically_derived]
            impl #impl_generics #digestible for #ident #ty_generics #where_clause {
                #[allow(non_snake_case)]
                fn digest<#order: #byte_order_path, W: _digestible::DigestWriter>(
                    &self,
                    writer: &mut W,
                ) {
                    #(#variants)*
                    #header_write
                    match self {
                        #(#catch_block)*
                    }
                }
            }
            #impl_hash
        };
    };

    Ok(result)
}
