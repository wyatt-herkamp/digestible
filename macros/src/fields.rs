use crate::consts::{digest_path, hashable_hack};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Path;

mod keywords {
    use syn::custom_keyword;
    custom_keyword!(skip);
    custom_keyword!(use_std_hash);
    custom_keyword!(with);
}
#[derive(Debug, Default)]
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
        if use_std_hash {
            if let Some(digest_with) = &digest_with {
                return Err(syn::Error::new(
                    digest_with.span(),
                    "Cannot use digest_with and use_std_hash at the same time",
                ));
            }
        }
        let attr = Self {
            skip,
            use_std_hash,
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
impl ToTokens for Field<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.attr.skip {
            return;
        } else if self.attr.use_std_hash {
            let use_std_hash = self.use_std_hash(self.writer);
            tokens.extend(use_std_hash);
            return;
        }
        let digestible = digest_path();
        let ident = &self.ident;
        let ty = &self.ty;
        let endian = self.endian;
        let writer = self.writer;
        let result = if let Some(digest_with) = &self.attr.digest_with {
            quote! {
                #digest_with::<#endian,_>(#ident, #writer);
            }
        } else {
            quote! {
                <#ty as #digestible>::digest::<#endian, _>(#ident, #writer);
            }
        };
        tokens.extend(result);
    }
}
