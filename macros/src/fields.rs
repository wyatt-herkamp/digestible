use crate::consts::{digest_with_path, digestible_path};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Expr, Path, Type};

mod keywords {
    use syn::custom_keyword;
    custom_keyword!(skip);
    custom_keyword!(with);
    custom_keyword!(digest_with);
    custom_keyword!(as_ref);
}
#[derive(Debug, Default)]
pub struct FieldAttr {
    pub skip: bool,
    pub as_ref: Option<Type>,
    pub digest_with: Option<Path>,
}
impl Parse for FieldAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut skip = false;
        let mut as_ref = None;
        let mut digest_with: Option<Path> = None;
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::skip) {
                let _ = input.parse::<keywords::skip>()?;
                skip = true;
            } else if lookahead.peek(keywords::with) {
                let _ = input.parse::<keywords::with>()?;
                let _: syn::Token![=] = input.parse()?;
                digest_with = Some(input.parse()?);
            } else if lookahead.peek(keywords::digest_with) {
                let _ = input.parse::<keywords::digest_with>()?;
                let _: syn::Token![=] = input.parse()?;

                let internal_digest_with_path = digest_with_path(input.parse()?);
                digest_with = Some(internal_digest_with_path);
            } else if lookahead.peek(keywords::as_ref) {
                let _ = input.parse::<keywords::as_ref>()?;
                let _: syn::Token![=] = input.parse()?;
                as_ref = Some(input.parse()?);
            } else {
                return Err(lookahead.error());
            }
        }

        let attr = Self {
            skip,
            as_ref,
            digest_with,
        };
        Ok(attr)
    }
}

#[derive(Debug)]
pub struct Field<'a> {
    pub ty: syn::Type,
    pub ident: Ident,
    pub attr: FieldAttr,
    pub endian: &'a Ident,
    pub writer: &'a Ident,
}

impl<'a> Field<'a> {
    pub fn new(
        field: syn::Field,
        index: usize,
        endian: &'a Ident,
        writer: &'a Ident,
    ) -> syn::Result<Self> {
        let attr = field
            .attrs
            .iter()
            .find(|v| v.path().is_ident("digestible"))
            .map(|v| v.parse_args::<FieldAttr>())
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            ident: field
                .ident
                .as_ref()
                .cloned()
                .unwrap_or_else(|| format_ident!("field_{}", index)),
            ty: field.ty,
            attr,
            endian,
            writer,
        })
    }
}
impl ToTokens for Field<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.attr.skip {
            return;
        }
        let digestible = digestible_path();
        let ident = &self.ident;
        let ty = if let Some(as_ref) = &self.attr.as_ref {
            as_ref
        } else {
            &self.ty
        };
        let endian = self.endian;
        let writer = self.writer;
        let variable_ref: Expr = if self.attr.as_ref.is_some() {
            parse_quote! {#ident.as_ref()}
        } else {
            parse_quote! {#ident}
        };
        let result = if let Some(digest_with) = &self.attr.digest_with {
            quote! {
                #digest_with::<#endian,_>(#variable_ref, #writer);
            }
        } else {
            quote! {<#ty as #digestible>::digest::<#endian, _>(#variable_ref,#writer);}
        };

        tokens.extend(result);
    }
}
