use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

use crate::digestible::internal_macros::{as_ref_then_call_inner, impl_for_as_ref_u8};
use crate::digestible::Digestible;
use crate::DigestWriter;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::{Rc, Weak as WeakRc};
use alloc::string::String;
use alloc::sync::{Arc, Weak as WeakArc};
use alloc::vec::Vec;
use byteorder::ByteOrder;

use super::core_types::{digest_iter, digest_native_iter};

impl<T: Digestible> Digestible for Vec<T> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}
impl_for_as_ref_u8!(String);

impl<D: Digestible> Digestible for Box<D> {
    as_ref_then_call_inner!();
}
impl<T: Digestible> Digestible for Arc<T> {
    as_ref_then_call_inner!();
}
impl<T: Digestible> Digestible for WeakRc<T> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        self.upgrade().digest::<B, W>(writer)
    }
}
impl<T: Digestible> Digestible for WeakArc<T> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        self.upgrade().digest::<B, W>(writer)
    }
}
impl<T: Digestible> Digestible for Rc<T> {
    as_ref_then_call_inner!();
}
impl<T: Digestible + Clone> Digestible for Cow<'_, T> {
    as_ref_then_call_inner!();
}

impl<V: Digestible> Digestible for BTreeSet<V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}
impl<V: Digestible> Digestible for BinaryHeap<V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}
impl<V: Digestible> Digestible for VecDeque<V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}
impl<V: Digestible> Digestible for LinkedList<V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}

impl<K: Digestible, V: Digestible> Digestible for BTreeMap<K, V> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        digest_iter::<_, B, W, _>(self.iter(), writer);
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        digest_native_iter::<_, W, _>(self.iter(), writer);
    }
}

mod tests {
    #[test]
    fn b_tree_map() {
        use crate::Digestible;
        use alloc::collections::BTreeMap;
        use byteorder::LittleEndian;
        let mut map = BTreeMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        let mut digest = Vec::new();
        map.digest::<LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
    }
    #[test]
    fn vec() {
        use crate::Digestible;
        use alloc::vec::Vec;
        use byteorder::LittleEndian;
        let vec = vec![1, 2, 3, 4];
        let mut digest = Vec::new();
        vec.digest::<LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
    }

    #[test]
    fn b_tree_set() {
        use crate::Digestible;
        use alloc::collections::BTreeSet;
        use byteorder::LittleEndian;
        let mut set = BTreeSet::new();
        set.insert(1);
        set.insert(2);
        let mut digest = Vec::new();
        set.digest::<LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![1, 0, 0, 0, 2, 0, 0, 0]);
    }

    #[test]
    fn binary_heap() {
        use crate::Digestible;
        use alloc::collections::BinaryHeap;
        use byteorder::LittleEndian;
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(2);
        let mut digest = Vec::new();
        heap.digest::<LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![2, 0, 0, 0, 1, 0, 0, 0]);
    }

    #[test]
    fn vec_deque() {
        use crate::Digestible;
        use alloc::collections::VecDeque;
        use byteorder::LittleEndian;
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        let mut digest = Vec::new();
        deque.digest::<LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![1, 0, 0, 0, 2, 0, 0, 0]);
    }

    #[test]
    fn linked_list() {
        use crate::Digestible;
        use alloc::collections::LinkedList;
        use byteorder::LittleEndian;
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        let mut digest = Vec::new();
        list.digest::<LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![1, 0, 0, 0, 2, 0, 0, 0]);
    }

    #[test]
    fn string() {
        use crate::Digestible;
        use byteorder::LittleEndian;
        let string = "hello".to_string();
        let mut digest = Vec::new();
        string.digest::<LittleEndian, _>(&mut digest);
        assert_eq!(digest, vec![104, 101, 108, 108, 111]);
    }
}
