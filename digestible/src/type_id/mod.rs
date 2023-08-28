use crate::{DigestWriter, Digester, Digestible};
use byteorder::ByteOrder;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructHeader(u128);

impl Digestible for StructHeader {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        self.0.digest::<B, _>(writer);
    }
}
impl StructHeader {
    pub const fn new(id: u128) -> Self {
        Self(id)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumHeader(u128, u128);

impl Digestible for EnumHeader {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        self.0.digest::<B, _>(writer);
        writer.write(b"::");
        self.1.digest::<B, _>(writer);
    }
}
impl EnumHeader {
    pub const fn new(id: u128, discriminate: u128) -> Self {
        Self(id, discriminate)
    }
}
