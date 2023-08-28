use syn::{parse_quote, Path};

pub fn digest_path() -> Path {
    parse_quote!(_digestible::Digestible)
}
pub fn digest_writer() -> Path {
    parse_quote!(_digestible::DigestWriter)
}
pub fn hashable_hack() -> Path {
    parse_quote!(_digestible::hash_digester::HashableHack)
}
pub fn digest_with() -> Path {
    parse_quote!(_digestible::digestible::DigestWith)
}
