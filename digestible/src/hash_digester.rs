use crate::{DigestWriter, Digester, Digestible};
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
/// Allows you to pass a Hasher into a type that excepts a Digester
///
/// ## Example
///
/// ```rust
/// use digestible::hash_digester::DigesterUsingHasher;
/// use digestible::{Digester, Digestible};
/// use std::hash::Hasher;
/// use std::io::Write;
/// #[derive(Digestible)]
/// pub struct MyStruct{
///     pub id: u32,
///     pub name: String,
/// }
///
///     let mut hasher = std::collections::hash_map::DefaultHasher::new();
///     let test = MyStruct{
///         id: 0,
///         name: "Test".to_string(),
///     };
///     let mut my_digest = DigesterUsingHasher(&mut hasher);
///     let result = my_digest.digest::<byteorder::NativeEndian>(&test);
///     println!("{:?}", result);
pub struct DigesterUsingHasher<'h, H: Hasher>(pub &'h mut H);
impl<H: Hasher> DigestWriter for DigesterUsingHasher<'_, H> {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
    #[inline(always)]
    fn write_u8(&mut self, data: u8) {
        self.0.write_u8(data)
    }
    #[inline(always)]
    fn write_i8(&mut self, data: i8) {
        self.0.write_i8(data)
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
    fn digest<B: ByteOrder>(mut self, data: &impl Digestible) -> Self::Target {
        Digestible::digest::<B, _>(data, &mut self);
        self.0.finish()
    }
}

/// Foreign Types that do not implement Digestible and do not have a way to access the inner data
/// ## Example
/// ```rust
/// use byteorder::ByteOrder;
/// use digestible::hash_digester::HashableHack;
/// use digestible::Digester;
/// use digestible::{DigestWriter, Digestible};
/// use sha2::Digest;
/// use std::hash::Hash;
/// use std::io::Write;
/// #[derive(Hash)]
/// pub struct MyHashableType(u32);
/// impl Digestible for MyHashableType {
///     fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
///         let mut hashable_hack = HashableHack(writer);
///         <Self as Hash>::hash(self, &mut hashable_hack);
///     }
/// }
///
/// use byteorder::NativeEndian;
/// let test = MyHashableType(0);
/// let mut hasher = sha2::Sha256::new();
/// let result = hasher.digest::<NativeEndian>(&test).to_vec();
/// println!("{:?}", result);
/// ```
pub struct HashableHack<'w, W: crate::DigestWriter>(pub &'w mut W);

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
