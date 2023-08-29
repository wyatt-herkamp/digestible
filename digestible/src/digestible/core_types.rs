use crate::digestible::Digestible;
use crate::DigestWriter;
use byteorder::ByteOrder;

impl<'a, T: Digestible> Digestible for &'a [T] {
    #[inline(always)]
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        for item in *self {
            item.digest::<B, W>(writer)
        }
    }
}
impl<'str> Digestible for &'str str {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        writer.write(self.as_bytes())
    }
}

impl Digestible for bool {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        writer.write(&[*self as u8])
    }
}
macro_rules! hash {
    (hash) => {
        crate::digestible::internal_macros::use_hasher!();
    };
    (no_hash) => {};
}
macro_rules! digestible_for_num {
    ($num:ty, 1,$write:ident) => {
        impl Digestible for $num {
            /// Writes the digestible data to the writer.
            /// Using the native byte order
            fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
                writer.$write(*self)
            }
            super::internal_macros::use_hasher!();
        }
    };
    ($num:ty, $write:ident) => {
        digestible_for_num!($num, $write, hash);
    };
    ($num:ty,  $write:ident, $hash:ident) => {
        impl Digestible for $num {
            /// Writes the digestible data to the writer.
            /// Using the native byte order
            fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
                writer.$write::<B>(*self)
            }
            hash!($hash);
        }
    };
}

digestible_for_num!(u8, 1, write_u8);
digestible_for_num!(u16, write_u16);
digestible_for_num!(u32, write_u32);
digestible_for_num!(u64, write_u64);
digestible_for_num!(u128, write_u128);
digestible_for_num!(i8, 1, write_i8);
digestible_for_num!(i16, write_i16);
digestible_for_num!(i32, write_i32);
digestible_for_num!(i64, write_i64);
digestible_for_num!(i128, write_i128);
digestible_for_num!(f32, write_f32, no_hash);
digestible_for_num!(f64, write_f64, no_hash);

digestible_for_num!(usize, write_usize);
digestible_for_num!(isize, write_isize);



impl<T: Digestible> Digestible for Option<T>{
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        match self {
            Some(value) => {
                writer.write_u8(1);
                value.digest::<B, W>(writer);
            }
            None => {
                writer.write_u8(0);
            }
        }
    }
}
impl<S: Digestible, E: Digestible> Digestible for Result<S, E>{
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        match self {
            Ok(value) => {
                writer.write_u8(0);
                value.digest::<B, W>(writer);
            }
            Err(value) => {
                writer.write_u8(1);
                value.digest::<B, W>(writer);
            }
        }
    }
}
impl Digestible for (){
    fn digest<B: ByteOrder, W: DigestWriter>(&self, _: &mut W) {}
}