use crate::digestible::Digestible;
use std::io::{Error, Write};
impl<'b> Digestible for &'b [u8] {
    type Digest<'a>  = &'a [u8]where Self: 'a;

    fn digest(&self) -> Self::Digest<'_> {
        self
    }

    fn supports_borrowed_digest() -> bool {
        true
    }

    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(self)
    }
}
impl Digestible for String {
    type Digest<'a> = &'a [u8];

    fn digest(&self) -> Self::Digest<'_> {
        self.as_bytes()
    }
    fn supports_borrowed_digest() -> bool {
        true
    }

    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(self.as_bytes())
    }
}

impl Digestible for Vec<u8> {
    type Digest<'a> = &'a [u8];

    fn supports_borrowed_digest() -> bool {
        true
    }
    fn digest(&self) -> Self::Digest<'_> {
        self.as_slice()
    }

    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        writer.write_all(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test() {
        use crate::digestible::Digestible;
        let s = "hello world".to_string();
        let d = s.digest();
        assert_eq!(d, "hello world".as_bytes());
        let d = s.digest_owned();
        assert_eq!(d, "hello world".as_bytes());
    }
    #[test]
    pub fn test_byte_array() {
        use crate::digestible::Digestible;
        let s = "hello world".as_bytes();
        let d = s.digest();
        assert_eq!(d, "hello world".as_bytes());
        let d = s.digest_owned();
        assert_eq!(d, "hello world".as_bytes());
    }
}
