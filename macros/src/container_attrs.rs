use syn::parse::{Parse, ParseStream};

use syn::{parse_quote, Path};

#[derive(Debug)]
pub enum TypeHeader {
    None,
    HashName,
    #[allow(dead_code)]
    TypeId {
        path_to_type_id_gen: Path,
    },
}
impl Default for TypeHeader {
    fn default() -> Self {
        Self::HashName
    }
}
impl Parse for TypeHeader {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keywords::None) {
            let _ = input.parse::<keywords::None>()?;
            Ok(Self::None)
        } else if lookahead.peek(keywords::HashName) {
            let _ = input.parse::<keywords::HashName>()?;
            Ok(Self::HashName)
        } else if lookahead.peek(keywords::type_id) {
            todo!("type_id")
        } else {
            Err(lookahead.error())
        }
    }
}
mod keywords {
    use syn::custom_keyword;
    custom_keyword!(type_header);
    custom_keyword!(None);
    custom_keyword!(HashName);
    custom_keyword!(type_id);
    custom_keyword!(hash);
}
#[derive(Debug, Default)]
pub struct ContainerAttrs {
    pub type_header: TypeHeader,
    pub impl_hash: Option<Path>,
}
impl Parse for ContainerAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut type_header = TypeHeader::default();
        let mut impl_hash = None;
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::type_header) {
                let _ = input.parse::<keywords::type_header>()?;
                let _: syn::Token![=] = input.parse()?;
                type_header = input.parse()?;
            } else if lookahead.peek(keywords::hash) {
                let _ = input.parse::<keywords::hash>()?;
                if input.peek(syn::Token![=]) {
                    let _: syn::Token![=] = input.parse()?;
                    impl_hash = Some(byte_order_impl_path(input.parse()?));
                } else {
                    impl_hash = Some(byte_order_impl_path(parse_quote!(NativeEndian)));
                }
            } else {
                return Err(lookahead.error());
            }
        }
        let attr = Self {
            type_header,
            impl_hash,
        };
        Ok(attr)
    }
}
macro_rules! get_container_attrs {
    ($input:ident) => {
        $input
            .attrs
            .iter()
            .find(|v| v.path().is_ident("digestible"))
            .map(|v| v.parse_args::<ContainerAttrs>())
            .transpose()?
            .unwrap_or_default()
    };
}
use crate::consts::byte_order_impl_path;
pub(crate) use get_container_attrs;
