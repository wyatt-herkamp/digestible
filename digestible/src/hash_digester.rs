use crate::{Digester, DigesterWriter, Digestible};
use byteorder::ByteOrder;
use core::hash::Hasher;
macro_rules! map_to_hasher {
    ( $(($call:ident($call_param:ident: $call_type:ty) => $to:ident)),*) => {
        $(
            #[inline(always)]
            fn $call<B: ByteOrder>(&mut self, $call_param: $call_type) {
                self.0.$to($call_param)
            }
        )*
    };

}
pub struct DigesterUsingHasher<'h, H: Hasher>(pub &'h mut H);
impl<H: Hasher> DigesterWriter for DigesterUsingHasher<'_, H> {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
    fn write_u8(&mut self, data: u8) {
        self.0.write(&[data])
    }
    fn write_i8(&mut self, data: i8) {
        self.0.write(&[data as u8])
    }
    map_to_hasher!(
        (write_u16(value: u16) => write_u16), (write_u32(value: u32) => write_u32),
        (write_u64(value: u64) => write_u64), (write_u128(value: u128) => write_u128),
        (write_usize(value: usize) => write_usize),
        (write_i16(value: i16) => write_i16), (write_i32(value: i32) => write_i32),
        (write_i64(value: i64) => write_i64), (write_i128(value: i128) => write_i128),
        (write_isize(value: isize) => write_isize)
    );
}
impl<H: Hasher> Digester for DigesterUsingHasher<'_, H> {
    type Target = u64;
    fn digest<B: ByteOrder, D: Digestible>(mut self, data: &D) -> Self::Target {
        self.digest_no_return::<B, D>(data);
        self.0.finish()
    }
    fn digest_no_return<B: ByteOrder, D: Digestible>(&mut self, data: &D) {
        data.digest::<B, _>(self)
    }
}

/// Foreign Types that do not implement Digestible and do not have a way to access the inner data
/// ## Example
/// ```rust, no_run
/// use byteorder::ByteOrder;
/// use digestible::hash_digester::HashableHack;
/// use digestible::{DigestWriter, Digestible};
/// use std::hash::Hash;
/// use std::io::Write;
/// #[derive(Hash)]
/// pub struct MyHashableType(u32);
/// impl Digestible for MyHashableType {
///     fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
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

macro_rules! map_to_digester {
    ( $(($call:ident($call_param:ident: $call_type:ty) => $to:ident)),*) => {
        $(
            #[inline(always)]
            fn $call(&mut self, $call_param: $call_type) {
                self.0.$to::<byteorder::NativeEndian>($call_param)
            }
        )*
    };

}
impl<W: crate::DigestWriter> Hasher for HashableHack<'_, W> {
    /// Does nothing.
    fn finish(&self) -> u64 {
        0
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
    fn write_u8(&mut self, i: u8) {
        self.0.write(&[i])
    }
    fn write_i8(&mut self, i: i8) {
        self.0.write(&[i as u8])
    }
    map_to_digester!(
        (write_u16(value: u16) => write_u16), (write_u32(value: u32) => write_u32),
        (write_u64(value: u64) => write_u64), (write_u128(value: u128) => write_u128),
        (write_usize(value: usize) => write_usize),
        (write_i16(value: i16) => write_i16), (write_i32(value: i32) => write_i32),
        (write_i64(value: i64) => write_i64), (write_i128(value: i128) => write_i128),
        (write_isize(value: isize) => write_isize)
    );
}
