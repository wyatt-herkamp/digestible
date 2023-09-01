use byteorder::ByteOrder;
macro_rules! write_doc {
    ($num:ty, $endian_write:ident) => {
        write_doc!($num, $endian_write, "")
    };
    ($num:ty, $endian_write:ident,  float) => {
        write_doc!($num, $endian_write, "Please Read [Floats 32 and f64](digestible/index.html#floats-f32-f64)")
    };
    ($num:ty, $endian_write:ident, $other:literal) => {
        concat!("Writes a [`", stringify!($num), "`] to the underlying writer.",
                    "<h5>Default Implementation</h5>
                    Creates a byte array of the size with size of the type. ",
                    "Then calls [`ByteOrder::", stringify!($endian_write), "`](byteorder::ByteOrder::", stringify!($endian_write), ")
                    with the data and the data array is passed to [`write`](DigestWriter::write)",
                    $other
        )
    };

}
macro_rules! define_write {
     (
            #[doc = $doc:expr]
            ($num:ty, $endian_write:ident, $size:literal, $fun_name:ident)
     ) => {
        #[doc = $doc]
        #[inline(always)]
        fn $fun_name<B: ByteOrder>(&mut self, data: $num) {
            let mut buffer = [0u8; $size];
            B::$endian_write(&mut buffer, data);
            self.write(&buffer);
        }
    };
    ($(($num:ty, $endian_write:ident, $size:literal, $fun_name:ident)),*) => {
        $(

                define_write!(
                    #[doc = write_doc!($num, $endian_write, $size)]
                    ($num, $endian_write, $size, $fun_name)
                );

        )*
    };
}
macro_rules! deref_and_call_inner {
    ($fnName:ident, $fnParam:ident: $fnType:ty) => {
        #[inline(always)]
        fn $fnName(&mut self, $fnParam: $fnType) {
            (**self).$fnName($fnParam)
        }
    };
    ($fnName:ident,ByteOrder, $fnParam:ident: $fnType:ty) => {
        #[inline(always)]
        fn $fnName<B: byteorder::ByteOrder>(&mut self, $fnParam: $fnType) {
            (**self).$fnName::<B>($fnParam)
        }
    };
    () => {
        deref_and_call_inner!(write, data: &[u8]);
        deref_and_call_inner!(write_bool, data: bool);
        deref_and_call_inner!(write_u8, data: u8);
        deref_and_call_inner!(write_i8, data: i8);
        deref_and_call_inner!(write_str, data: &str);
        deref_and_call_inner!(write_usize,ByteOrder, data: usize);
        deref_and_call_inner!(write_isize, ByteOrder, data: isize);
        deref_and_call_inner!(write_u16,ByteOrder, data: u16);
        deref_and_call_inner!(write_u32,ByteOrder, data: u32);
        deref_and_call_inner!(write_u64,ByteOrder, data: u64);
        deref_and_call_inner!(write_u128,ByteOrder, data: u128);
        deref_and_call_inner!(write_i16,ByteOrder, data: i16);
        deref_and_call_inner!(write_i32,ByteOrder, data: i32);
        deref_and_call_inner!(write_i64,ByteOrder, data: i64);
        deref_and_call_inner!(write_i128,ByteOrder, data: i128);
        deref_and_call_inner!(write_f32,ByteOrder, data: f32);
        deref_and_call_inner!(write_f64,ByteOrder, data: f64);

    };
}
/// A writer trait targeting an in memory buffer or the Digester itself.
///
/// This is what is passed into [`Digestible::digest`](crate::Digestible::digest)
///
/// The default implementation just converts the data types to bytes and calls [`write`](DigestWriter::write)
/// ## Default Implementations
/// - [`Vec<u8>`] requires 'alloc'
/// - [bytes::BytesMut] (requires the `bytes` feature)
pub trait DigestWriter {
    /// Writes the data to the underlying writer.
    ///
    /// This is the only function that is required to be implemented.
    fn write(&mut self, data: &[u8]);
    /// Writes a [`bool`](bool) to the underlying writer
    ///
    /// # Default Implementation
    /// Casts the bool to a u8 then puts it in a byte array calls [`write`](DigestWriter::write)
    #[inline(always)]
    fn write_bool(&mut self, data: bool) {
        self.write(&[data as u8]);
    }
    /// Writes a [`u8`](u8) to the underlying writer
    ///
    /// # Default Implementation
    /// Puts the u8 in a byte array then calls [`write`](DigestWriter::write)
    #[inline(always)]
    fn write_u8(&mut self, data: u8) {
        self.write(&[data]);
    }

    /// Writes a [`i8`](i8) to the underlying writer
    ///
    /// # Default Implementation
    /// Casts to u8 then puts in a byte array then calls [`write`](DigestWriter::write)
    #[inline(always)]
    fn write_i8(&mut self, data: i8) {
        self.write(&[data as u8]);
    }

    /// Writes a [`str`](core::str) to the underlying writer
    ///
    /// # Default Implementation
    /// Calls [`write`](DigestWriter::write) with the bytes of the string
    #[inline(always)]
    fn write_str(&mut self, data: &str) {
        self.write(data.as_bytes());
    }
    #[doc = write_doc!(isize, write_uint)]
    #[inline(always)]
    fn write_usize<B: ByteOrder>(&mut self, data: usize) {
        let mut buffer = [0u8; core::mem::size_of::<usize>()];
        B::write_uint(&mut buffer, data as u64, core::mem::size_of::<usize>());
        self.write(&buffer);
    }
    #[doc = write_doc!(isize, write_int)]
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
        (i128, write_i128, 16, write_i128)
    );
    define_write!(
        #[doc = write_doc!(f32, write_f32, float)]
        (f32, write_f32, 4, write_f32)
    );
    define_write!(
        #[doc = write_doc!(f32, write_f32, float)]
        (f64, write_f64, 8, write_f64)
    );
}
impl<'a, T: DigestWriter> DigestWriter for &'a mut T {
    deref_and_call_inner!();
}
#[cfg(feature = "alloc")]
mod has_alloc {
    use crate::DigestWriter;
    use alloc::vec::Vec;
    use alloc::boxed::Box;

    impl DigestWriter for Vec<u8> {
        fn write(&mut self, data: &[u8]) {
            self.extend_from_slice(data);
        }
    }
    impl<T: DigestWriter> DigestWriter for Box<T> {
        deref_and_call_inner!();
    }
}

#[cfg(feature = "bytes")]
impl DigestWriter for bytes::BytesMut {
    fn write(&mut self, data: &[u8]) {
        self.extend_from_slice(data);
    }
}