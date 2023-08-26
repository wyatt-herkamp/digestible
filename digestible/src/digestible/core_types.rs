use crate::digestible::Digestible;
use crate::DigestWriter;
use byteorder::ByteOrder;

impl<'a> Digestible for &'a [u8] {
    #[inline(always)]
    fn digest_to_writer<W: DigestWriter>(&self, writer: &mut W) {
        writer.write(self)
    }
}
impl<'str> Digestible for &'str str {
    fn digest_to_writer<W: crate::DigestWriter>(&self, writer: &mut W) {
        writer.write(self.as_bytes())
    }
}
#[cfg(feature = "alloc")]
impl<'str> super::DigestibleAlloc for &'str str {
    crate::digestible::internal_macros::digest_owned_with_size!(
        fn size(this: &str) -> usize {
            this.as_bytes().len()
        }
    );
}

impl Digestible for bool {
    fn digest_to_writer<W: crate::DigestWriter>(&self, writer: &mut W) {
        writer.write(&[*self as u8])
    }
}

macro_rules! digestible_for_num {
    ($num:ty) => {
        impl Digestible for $num {
            #[inline(always)]
            fn digest_to_writer<W: crate::DigesterWriter>(&self, writer: &mut W) {
                writer.write(&[*self as u8])
            }
            super::internal_macros::use_hasher!();
        }
        #[cfg(feature = "alloc")]
        impl super::DigestibleAlloc for $num {
            super::internal_macros::digest_owned_with_size!(1);
        }
    };
    ($num:ty as $rep:ty, $write:ident) => {
        impl Digestible for $num {
            fn digest_to_writer<W: crate::DigesterWriter>(&self, writer: &mut W) {
                writer.write(&self.to_ne_bytes());
            }
            #[inline(always)]
            fn digest_to_writer_with_order<B: ByteOrder, W: crate::DigesterWriter>(
                &self,
                writer: &mut W,
            ) {
                let mut buf = [0u8; core::mem::size_of::<$num>()];
                B::$write(&mut buf, *self as $rep, core::mem::size_of::<$num>());
                writer.write(&buf);
            }
            super::internal_macros::use_hasher!();
        }
        #[cfg(feature = "alloc")]
        impl super::DigestibleAlloc for $num {
            super::internal_macros::digest_owned_with_size!(core::mem::size_of::<$num>);
        }
    };
    ($num:ty, $size:literal, $write:ident) => {
        impl Digestible for $num {
            /// Writes the digestible data to the writer.
            /// Using the native byte order
            fn digest_to_writer<W: crate::DigesterWriter>(&self, writer: &mut W) {
                writer.write(&self.to_ne_bytes())
            }
            #[inline(always)]
            fn digest_to_writer_with_order<B: ByteOrder, W: crate::DigesterWriter>(
                &self,
                writer: &mut W,
            ) {
                let mut buf = [0u8; $size];
                B::$write(&mut buf, *self);
                writer.write(&buf)
            }
            super::internal_macros::use_hasher!();
        }

        #[cfg(feature = "alloc")]
        impl super::DigestibleAlloc for $num {
            super::internal_macros::digest_owned_with_size!($size);
        }
    };
    ($num:ty, $size:literal, $write:ident, no_hash) => {
        impl Digestible for $num {
            /// Writes the digestible data to the writer.
            /// Using the native byte order
            fn digest_to_writer<W: crate::DigesterWriter>(&self, writer: &mut W) {
                writer.write(&self.to_ne_bytes())
            }
            #[inline(always)]
            fn digest_to_writer_with_order<B: ByteOrder, W: crate::DigesterWriter>(
                &self,
                writer: &mut W,
            ) {
                let mut buf = [0u8; $size];
                B::$write(&mut buf, *self);
                writer.write(&buf)
            }
        }

        #[cfg(feature = "alloc")]
        impl super::DigestibleAlloc for $num {
            super::internal_macros::digest_owned_with_size!($size);
        }
    };
}

digestible_for_num!(u8);
digestible_for_num!(u16, 2, write_u16);
digestible_for_num!(u32, 4, write_u32);
digestible_for_num!(u64, 8, write_u64);
digestible_for_num!(u128, 16, write_u128);
digestible_for_num!(i8);
digestible_for_num!(i16, 2, write_i16);
digestible_for_num!(i32, 4, write_i32);
digestible_for_num!(i64, 8, write_i64);
digestible_for_num!(i128, 16, write_i128);
digestible_for_num!(f32, 4, write_f32, no_hash);
digestible_for_num!(f64, 8, write_f64, no_hash);

digestible_for_num!(usize as u64, write_uint);
digestible_for_num!(isize as i64, write_int);
