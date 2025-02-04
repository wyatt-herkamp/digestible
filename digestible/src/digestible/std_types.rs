use crate::digestible::internal_macros::impl_for_hashable_hack;
use crate::{DigestWriter, Digestible};
use byteorder::ByteOrder;
use std::collections::{HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use super::core_types::{digest_iter, digest_native_iter};
impl_for_hashable_hack!(OsStr);
impl_for_hashable_hack!(OsString);
impl_for_hashable_hack!(PathBuf);
impl_for_hashable_hack!(Path);

impl<S, K: Digestible, V: Digestible> Digestible for HashMap<K, V, S> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }

    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}

impl<S, V: Digestible> Digestible for HashSet<V, S> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}

#[cfg(test)]
mod tests {
    use crate::Digestible;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn hash_map() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        let mut digest = Vec::new();
        map.digest::<byteorder::LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![107, 101, 121, 118, 97, 108, 117, 101]);
    }


    #[test]
    fn hashset() {
        let mut map = HashSet::new();
        map.insert("key");
        map.insert("value");
        let mut digest = Vec::new();
        map.digest::<byteorder::LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![107, 101, 121, 118, 97, 108, 117, 101]);
    }
}
