use syn::{parse_quote, Path};

pub fn digest_path() -> Path {
    parse_quote!(_digestible::Digestible)
}
pub fn hashable_hack() -> Path {
    parse_quote!(_digestible::digestible::hash_hack::HashableHack)
}
pub fn digest_with() -> Path {
    parse_quote!(_digestible::digestible::DigestWith)
}
