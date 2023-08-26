use core::hash::Hasher;
use crate::{Digester, DigesterWriter, Digestible};

pub struct DigesterUsingHasher<'h, H: Hasher>(pub &'h mut H);
impl<H: Hasher> DigesterWriter for DigesterUsingHasher<'_, H> {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}
impl<H: Hasher> Digester for DigesterUsingHasher<'_, H> {
    type Target = u64;
    fn digest<D: Digestible>(mut self, data: &D) -> Self::Target {
        self.digest_no_return(data);
        self.0.finish()
    }
    fn digest_no_return<D: Digestible>(&mut self, data: &D) {
        data.digest_to_writer(self)
    }
}


/// Foreign Types that do not implement Digestible and do not have a way to access the inner data
/// ## Example
/// ```rust, no_run
/// use digestible::{Digestible, DigestWriter};
/// use std::hash::Hash;
/// use std::io::Write;
/// use digestible::hash_digester::HashableHack;
/// #[derive(Hash)]
/// pub struct MyHashableType(u32);
/// impl Digestible for MyHashableType {
///     fn digest_to_writer<W: DigestWriter>(&self, writer: &mut W)  {
///         let mut hashable_hack = HashableHack::new(writer);
///         <Self as Hash>::hash(self, &mut hashable_hack);
///     }
/// }
/// ```
pub struct HashableHack<'a, W: crate::DigestWriter>(&'a mut W);

impl<'a, W: crate::DigestWriter> HashableHack<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self(writer)
    }
}
impl<W: crate::DigestWriter> Hasher for HashableHack<'_, W> {
    /// Does nothing.
    fn finish(&self) -> u64 {
        0
    }
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes);
    }
}
