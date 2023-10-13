use crate::paths::{digester_using_hasher, private_path};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Path;

pub fn impl_hash(container: &Ident, endian_path: Path) -> TokenStream {
    let digester_using_hasher = digester_using_hasher();
    let digestible_path = crate::paths::digestible_path();
    let hash: Path = private_path!(Hash);
    let hasher: Path = private_path!(Hasher);
    quote! {
        #[automatically_derived]
        impl #hash for #container {
            fn hash<H: #hasher>(&self, state: &mut H) {
                let mut digester = #digester_using_hasher(state);
                <Self as #digestible_path>::digest::<#endian_path, _>(self,&mut digester);
            }
        }
    }
}
