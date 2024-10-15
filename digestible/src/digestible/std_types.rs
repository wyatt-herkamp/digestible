use crate::digestible::internal_macros::impl_for_hashable_hack;
use crate::{DigestWriter, Digestible};
use byteorder::ByteOrder;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
impl_for_hashable_hack!(OsStr);
impl_for_hashable_hack!(OsString);
impl_for_hashable_hack!(PathBuf);
impl_for_hashable_hack!(Path);

impl<K: Digestible, V: Digestible> Digestible for HashMap<K, V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        for (key, value) in self {
            key.digest::<B, W>(writer);
            value.digest::<B, W>(writer);
        }
    }
}

impl<K: Digestible, V: Digestible> Digestible for BTreeMap<K, V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        for (key, value) in self {
            key.digest::<B, W>(writer);
            value.digest::<B, W>(writer);
        }
    }
}
impl<V: Digestible> Digestible for HashSet<V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        for value in self {
            value.digest::<B, W>(writer);
        }
    }
}
