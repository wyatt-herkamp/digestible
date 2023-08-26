use crate::digester::SmallDigester;
use crate::{DigestWriter, Digester, Digestible};
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

    fn digest<D: Digestible>(mut self, data: &D) -> Self::Target {
        data.digest_to_writer(&mut self);
        self.0.finish()
    }

    fn digest_no_return<D: Digestible>(&mut self, data: &D) {
        data.digest_to_writer(self);
    }
}
impl SmallDigester for TypeIDHasher {
    fn digest<D: Digestible>(data: &D) -> Self::Target {
        let hasher = Self(DefaultHasher::new());
        hasher.digest(data)
    }
}
