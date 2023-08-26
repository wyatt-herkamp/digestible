use crate::consts::{digest_path, digest_with as digest_with_path, hashable_hack};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Path;

mod keywords {
    use syn::custom_keyword;
    custom_keyword!(skip);
    custom_keyword!(use_std_hash);
    custom_keyword!(with);
}
#[derive(Debug)]
pub struct FieldAttr {
    pub skip: bool,
    pub use_std_hash: bool,
    pub digest_with: Option<Path>,
}
impl Parse for FieldAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut skip = false;
        let mut use_std_hash = false;
        let mut digest_with: Option<Path> = None;
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::skip) {
                let _ = input.parse::<keywords::skip>()?;
                skip = true;
            } else if lookahead.peek(keywords::use_std_hash) {
                let _ = input.parse::<keywords::use_std_hash>()?;
                use_std_hash = true;
            } else if lookahead.peek(keywords::with) {
                let _ = input.parse::<keywords::with>()?;
                let _: syn::Token![=] = input.parse()?;
                digest_with = Some(input.parse()?);
            } else {
                return Err(lookahead.error());
            }
        }
        if digest_with.is_some() && use_std_hash {
            return Err(syn::Error::new(
                digest_with.unwrap().span(),
                "Cannot use digest_with and use_std_hash at the same time",
            ));
        }
        let attr = Self {
            skip,
            use_std_hash,
            digest_with,
        };
        Ok(attr)
    }
}
impl Default for FieldAttr {
    fn default() -> Self {
        Self {
            skip: false,
            use_std_hash: false,
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
            ident: field
                .ident
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or_else(|| Ident::new(&*index.to_string(), field.span())),
            ty: field.ty,
            attr,
        })
    }
    pub fn digest_with_order(&self, endian: &Ident, writer: &Ident) -> Option<TokenStream> {
        if self.attr.skip {
            return None;
        } else if self.attr.use_std_hash {
            return Some(self.use_std_hash(writer));
        }
        let digestible = digest_path();
        let ident = &self.ident;
        let ty = &self.ty;
        let result = if let Some(digest_with) = &self.attr.digest_with {
            let digest_with_path = digest_with_path();
            quote! {
                <#digest_with as #digest_with_path>::digest::<#endian,_>(#ident, #writer);
            }
        } else {
            quote! {
                <#ty as #digestible>::digest::<#endian, _>(#ident, #writer);
            }
        };
        Some(result)
    }
    fn use_std_hash(&self, writer: &Ident) -> TokenStream {
        let hashable_hack = hashable_hack();
        let ident = &self.ident;
        let ty = &self.ty;
        quote! {
            {
                let mut hashable_hack = #hashable_hack::new(#writer);
                <#ty as std::hash::Hash>::hash(#ident, &mut hashable_hack);
            }
        }
    }
}
