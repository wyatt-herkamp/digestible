use crate::digestible::Digestible;
use byteorder::ByteOrder;
use std::io::{Error, Write};
impl Digestible for bool {
    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[*self as u8])
    }
}

macro_rules! digestible_for_num {
    ($num:ty) => {
        impl Digestible for $num {
            #[inline(always)]
            fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
                writer.write_all(&[*self as u8])
            }
            super::internal_macros::use_hasher!();
            super::internal_macros::digest_owned_with_size!(1);
        }
    };
    ($num:ty as $rep:ty, $write:ident) => {
        impl Digestible for $num {
            fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
                writer.write_all(&self.to_ne_bytes())
            }
            #[inline(always)]
            fn digest_to_writer_with_order<B: ByteOrder, W: Write>(
                &self,
                writer: &mut W,
            ) -> Result<(), Error> {
                let mut buf = [0u8; std::mem::size_of::<$num>()];
                B::$write(&mut buf, *self as $rep, std::mem::size_of::<$num>());
                writer.write_all(&buf)
            }
            super::internal_macros::use_hasher!();
            super::internal_macros::digest_owned_with_size!(std::mem::size_of::<$num>);
        }
    };
    ($num:ty, $size:literal, $write:ident) => {
        impl Digestible for $num {
            /// Writes the digestible data to the writer.
            /// Using the native byte order
            fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
                writer.write_all(&self.to_ne_bytes())
            }
            #[inline(always)]
            fn digest_to_writer_with_order<B: ByteOrder, W: Write>(
                &self,
                writer: &mut W,
            ) -> Result<(), Error> {
                let mut buf = [0u8; $size];
                B::$write(&mut buf, *self);
                writer.write_all(&buf)
            }
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
digestible_for_num!(f32, 4, write_f32);
digestible_for_num!(f64, 8, write_f64);

digestible_for_num!(usize as u64, write_uint);
digestible_for_num!(isize as i64, write_int);

#[cfg(test)]
mod tests {
    use crate::digestible::Digestible;
    use byteorder::ByteOrder;
    macro_rules! test {
        ($num:ty) => {
            let byte: $num = 1 as $num;
            let num_as_bytes = byte.to_ne_bytes();
            let d = byte.digest_owned();
            assert_eq!(d, num_as_bytes);
        };
    }
    #[test]
    pub fn test() {
        test!(u8);
        test!(u16);
        test!(u32);
        test!(u64);
        test!(u128);
        test!(i8);
        test!(i16);
        test!(i32);
        test!(i64);
        test!(i128);
    }
    macro_rules! test_endian {
        ($num:ty, $endian:ty, $write:ident) => {
            let byte: $num = 1 as $num;
            let mut wtr = vec![0u8; std::mem::size_of::<$num>()];
            <$endian>::$write(&mut wtr, byte);
            let d = byte.digest_owned_with_order::<$endian>();
            assert_eq!(d, wtr);
        };
    }

    #[test]
    pub fn test_big_endian() {
        test_endian!(u16, byteorder::BigEndian, write_u16);
        test_endian!(u32, byteorder::BigEndian, write_u32);
        test_endian!(u64, byteorder::BigEndian, write_u64);
        test_endian!(u128, byteorder::BigEndian, write_u128);
        test_endian!(i16, byteorder::BigEndian, write_i16);
        test_endian!(i32, byteorder::BigEndian, write_i32);
        test_endian!(i64, byteorder::BigEndian, write_i64);
        test_endian!(i128, byteorder::BigEndian, write_i128);
    }

    #[test]
    pub fn test_little_endian() {
        test_endian!(u16, byteorder::LittleEndian, write_u16);
        test_endian!(u32, byteorder::LittleEndian, write_u32);
        test_endian!(u64, byteorder::LittleEndian, write_u64);
        test_endian!(u128, byteorder::LittleEndian, write_u128);
        test_endian!(i16, byteorder::LittleEndian, write_i16);
        test_endian!(i32, byteorder::LittleEndian, write_i32);
        test_endian!(i64, byteorder::LittleEndian, write_i64);
        test_endian!(i128, byteorder::LittleEndian, write_i128);
    }
}
