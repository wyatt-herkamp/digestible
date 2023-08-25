use crate::consts::digest_path;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{Path, Type};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

mod keywords {
    use syn::custom_keyword;
    custom_keyword!(skip);
}
#[derive(Debug)]
pub struct FieldAttr {
    pub skip: bool,
    pub digest_with: Option<syn::Path>,
}
impl Parse for FieldAttr{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut skip = false;
        let mut digest_with = None;
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::skip) {
                let _ = input.parse::<keywords::skip>()?;
                skip = true;
            } else if lookahead.peek(syn::Token![=]) {
                let _: syn::Token![=] = input.parse()?;
                digest_with = Some(input.parse()?);
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(Self {
            skip,
            digest_with,
        })
    }
}
impl Default for FieldAttr {
    fn default() -> Self {
        Self {
            skip: false,
            digest_with: None,
        }
    }
}
#[derive(Debug)]
pub struct Field {
    pub ty: syn::Type,
    pub ident: Ident,
    pub attr: FieldAttr,
}

impl Field {
    pub fn new(field: syn::Field, index: usize) -> syn::Result<Self> {
        let attr = field
            .attrs
            .iter()
            .find(|v| v.path().is_ident("digestible"))
            .map(|v| v.parse_args::<FieldAttr>())
            .transpose()?
            .unwrap_or_default();

        Ok(Self {
            ident: field.ident.as_ref().map(|v| v.clone()).unwrap_or_else(|| Ident::new(&*index.to_string(), field.span())),
            ty: field.ty,
            attr,
        })
    }
    pub fn digest(&self, writer: &Ident) -> Option<TokenStream> {
        if self.attr.skip {
            return None;
        }
        let digestible = digest_path();
        let ident = &self.ident;
        let ty = &self.ty;
        let result = if let Some(value) = &self.attr.digest_with {
            todo!("digest_with")
        } else {
            quote! {
                <#ty as #digestible>::digest_to_writer(#ident, writer)?;
            }
        };
        Some(result)
    }
   pub  fn digest_with_order(
        &self,
        endian: &Ident,
        writer: &Ident,
    ) -> Option<TokenStream> {
        if self.attr.skip {
            return None;
        }
        let digestible = digest_path();
        let ident = &self.ident;
        let ty = &self.ty;
        let result = if let Some(value) = &self.attr.digest_with {
            todo!("digest_with")
        } else {
            quote! {
                <#ty as #digestible>::digest_to_writer_with_order::<#endian, _>(#ident, #writer)?;
            }
        };
        Some(result)
    }
}
