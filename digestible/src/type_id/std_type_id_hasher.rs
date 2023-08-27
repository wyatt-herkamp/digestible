use crate::{DigestWriter, Digester, Digestible};
use byteorder::ByteOrder;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub struct TypeIDHasher(DefaultHasher);
impl DigestWriter for TypeIDHasher {
    fn write(&mut self, data: &[u8]) {
        self.0.write(data);
    }
}
impl Digester for TypeIDHasher {
    type Target = u128;

    fn digest<B: ByteOrder, D: Digestible>(mut self, data: &D) -> Self::Target {
        data.digest::<B, _>(&mut self);
        self.0.finish() as u128
    }

    fn digest_no_return<B: ByteOrder, D: Digestible>(&mut self, data: &D) {
        data.digest::<B, _>(self);
    }
}
