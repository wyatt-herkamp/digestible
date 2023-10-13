use proc_macro2::{Ident, TokenStream};
use syn::{GenericParam, Generics, ImplGenerics, parse_quote, Path, TypeGenerics, WhereClause};

pub fn digestible_path() -> Path {
    parse_quote!(_digestible::Digestible)
}
pub fn digest_writer() -> Path {
    parse_quote!(_digestible::DigestWriter)
}
pub fn digest_with_path(path: Path) -> Path {
    parse_quote!(_digestible::digest_with::#path)
}
pub fn digester_using_hasher() -> Path {
    parse_quote!(_digestible::hash_digester::DigesterUsingHasher)
}

pub fn byte_order_path() -> Path {
    parse_quote!(_digestible::byteorder::ByteOrder)
}
pub fn byte_order_impl_path(ident: Ident) -> Path {
    parse_quote!(_digestible::byteorder::#ident)
}

macro_rules! private_path {
    // `()` indicates that the macro takes no argument.
    ($key:ident) => {
        syn::parse_quote!(_digestible::_private::$key)
    };
}
pub(crate) use private_path;

pub fn add_digestible_trait(generics: &mut Generics) {
    if generics.params.is_empty() {
        return;
    }
    for param in &mut generics.params {
        if let GenericParam::Type(ty) = param {
            ty.bounds.push(parse_quote!(_digestible::Digestible));
        }
    }
}

use quote::quote;

/// Implements `Hash` for the container.
/// Using Digestible
pub fn impl_hash(container: &Ident, endian_path: Path, impl_generics: &ImplGenerics, ty_generics: &TypeGenerics, where_clause: &Option<&WhereClause>) -> TokenStream {
    let digester_using_hasher = digester_using_hasher();
    let digestible_path = digestible_path();
    let hash: Path = private_path!(Hash);
    let hasher: Path = private_path!(Hasher);
    quote! {
        #[automatically_derived]
        impl #impl_generics  #hash for #container #ty_generics #where_clause {
            fn hash<H: #hasher>(&self, state: &mut H) {
                let mut digester = #digester_using_hasher(state);
                <Self as #digestible_path>::digest::<#endian_path, _>(self,&mut digester);
            }
        }
    }
}
