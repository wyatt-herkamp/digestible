use syn::{parse_quote, Path};

pub fn digest_path() -> Path {
    parse_quote!(_digestible::Digestible)
}
