use crate::digester::SmallDigester;
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
    type Target = u64;

    fn digest<B: ByteOrder, D: Digestible>(mut self, data: &D) -> Self::Target {
        data.digest::<B, _>(&mut self);
        self.0.finish()
    }

    fn digest_no_return<B: ByteOrder, D: Digestible>(&mut self, data: &D) {
        data.digest::<B, _>(self);
    }
}
impl SmallDigester for TypeIDHasher {
    fn digest<B: ByteOrder, D: Digestible>(data: &D) -> Self::Target {
        let hasher = Self(DefaultHasher::new());
        hasher.digest::<B, D>(data)
    }
}
