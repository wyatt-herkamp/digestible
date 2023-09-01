use crate::consts::digester_using_hasher;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

pub fn impl_hash(container: &Ident, endian_path: Path) -> TokenStream {
    let digester_using_hasher = digester_using_hasher();
    let digestible_path = crate::consts::digestible_path();
    quote! {
        impl core::hash::Hash for #container {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                let mut digester = #digester_using_hasher(state);
                <Self as #digestible_path>::digest::<#endian_path, _>(self,&mut digester);
            }
        }
    }
}
