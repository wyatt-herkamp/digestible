use proc_macro2::Ident;
use syn::{parse_quote, Path};

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
