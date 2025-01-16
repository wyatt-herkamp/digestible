use crate::digestible::internal_macros::impl_for_hashable_hack;
use crate::digestible::Digestible;
use crate::DigestWriter;
use byteorder::ByteOrder;
use core::marker::PhantomData;

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
impl Digestible for char {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        writer.write(&[*self as u8])
    }
}

macro_rules! digestible_for_num {
    ($num:ty, 1,$write:ident) => {
        impl Digestible for $num {
            /// Writes the digestible data to the writer.
            /// Using the native byte order
            #[inline(always)]
            fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
                writer.$write(*self)
            }
        }
    };
    ($num:ty,  $write:ident) => {
        impl Digestible for $num {
            /// Writes the digestible data to the writer.
            /// Using the native byte order
            #[inline(always)]
            fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
                writer.$write::<B>(*self)
            }
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
digestible_for_num!(f32, write_f32);
digestible_for_num!(f64, write_f64);

digestible_for_num!(usize, write_usize);
digestible_for_num!(isize, write_isize);

impl<T: Digestible> Digestible for Option<T> {
    #[inline]
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        if let Some(value) = self {
            value.digest::<B, W>(writer);
        }
    }
}
impl<S: Digestible, E: Digestible> Digestible for Result<S, E> {
    #[inline]
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        match self {
            Ok(value) => {
                value.digest::<B, W>(writer);
            }
            Err(value) => {
                value.digest::<B, W>(writer);
            }
        }
    }
}

impl<T: ?Sized> Digestible for PhantomData<T> {
    #[inline(always)]
    fn digest<B: ByteOrder, W: DigestWriter>(&self, _: &mut W) {}
}
impl_for_hashable_hack!(core::time::Duration);

/// Digests an interator of digestible items
#[inline(always)]
pub(crate) fn digest_iter<'item, Item, B, W, I>(iter: I, writer: &mut W)
where
    Item: Digestible + 'item,
    B: ByteOrder,
    W: DigestWriter,
    I: Iterator<Item = Item>,
{
    for item in iter {
        item.digest::<B, W>(writer);
    }
}
/// Digests an interator of digestible items
#[inline(always)]
pub(crate) fn digest_native_iter<'item, Item, W, I>(iter: I, writer: &mut W)
where
    Item: Digestible + 'item,
    W: DigestWriter,
    I: Iterator<Item = Item>,
{
    for item in iter {
        item.digest_native::<W>(writer);
    }
}
