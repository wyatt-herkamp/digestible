use byteorder::ByteOrder;
macro_rules! define_write {
    ($(($num:ty, $endian_write:ident, $size:literal, $fun_name:ident)),*) => {
        $(
            #[doc = concat!("Writes a [`", stringify!($num), "`](core::", stringify!($num), ") to the underlying writer.")]
            #[inline(always)]
            fn $fun_name<B: ByteOrder>(&mut self, data: $num) {
                let mut buffer = [0u8; $size];
                B::$endian_write(&mut buffer, data);
                self.write(&buffer);
            }
        )*
    };
}
pub trait DigestWriter {
    fn write(&mut self, data: &[u8]);
    #[inline(always)]
    fn write_bool(&mut self, data: bool) {
        self.write(&[data as u8]);
    }
    #[inline(always)]
    fn write_u8(&mut self, data: u8) {
        self.write(&[data]);
    }
    #[inline(always)]
    fn write_i8(&mut self, data: i8) {
        self.write(&[data as u8]);
    }
    #[inline(always)]
    fn write_str(&mut self, data: &str) {
        self.write(data.as_bytes());
    }
    #[inline(always)]
    fn write_usize<B: ByteOrder>(&mut self, data: usize) {
        let mut buffer = [0u8; core::mem::size_of::<usize>()];
        B::write_uint(&mut buffer, data as u64, core::mem::size_of::<usize>());
        self.write(&buffer);
    }
    #[inline(always)]
    fn write_isize<B: ByteOrder>(&mut self, data: isize) {
        let mut buffer = [0u8; core::mem::size_of::<isize>()];
        B::write_int(&mut buffer, data as i64, core::mem::size_of::<isize>());
        self.write(&buffer);
    }
    define_write!(
        (u16, write_u16, 2, write_u16),
        (u32, write_u32, 4, write_u32),
        (u64, write_u64, 8, write_u64),
        (u128, write_u128, 16, write_u128),
        (i16, write_i16, 2, write_i16),
        (i32, write_i32, 4, write_i32),
        (i64, write_i64, 8, write_i64),
        (i128, write_i128, 16, write_i128),
        (f32, write_f32, 4, write_f32),
        (f64, write_f64, 8, write_f64)
    );
    #[inline(always)]
    fn write_separator(&mut self) {}
}
#[cfg(feature = "alloc")]
mod has_alloc {
    use crate::DigestWriter;
    use alloc::vec::Vec;

    impl DigestWriter for Vec<u8> {
        fn write(&mut self, data: &[u8]) {
            self.extend_from_slice(data);
        }
    }
}
#[cfg(feature = "bytes")]
mod has_bytes {
    use crate::DigestWriter;
    use bytes::BytesMut;

    impl DigestWriter for BytesMut {
        fn write(&mut self, data: &[u8]) {
            self.extend_from_slice(data);
        }
    }
}
