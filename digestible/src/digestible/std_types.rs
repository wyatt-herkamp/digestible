use crate::digestible::internal_macros::impl_for_hashable_hack;
use crate::{DigestWriter, Digestible};
use byteorder::ByteOrder;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use super::core_types::{digest_iter, digest_native_iter};
impl_for_hashable_hack!(OsStr);
impl_for_hashable_hack!(OsString);
impl_for_hashable_hack!(PathBuf);
impl_for_hashable_hack!(Path);

impl<K: Digestible, V: Digestible> Digestible for HashMap<K, V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<(&K, &V), B, W, _>(self.iter(), writer);
    }

    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}

impl<K: Digestible, V: Digestible> Digestible for BTreeMap<K, V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<(&K, &V), B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}
impl<V: Digestible> Digestible for HashSet<V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}
