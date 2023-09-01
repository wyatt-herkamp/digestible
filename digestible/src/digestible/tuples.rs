#![allow(non_snake_case)]
use crate::{DigestWriter, Digestible};
use byteorder::ByteOrder;

impl Digestible for () {
    #[inline(always)]
    fn digest<B: ByteOrder, W: DigestWriter>(&self, _: &mut W) {}
}

macro_rules! tuple_configs {
    ($($T:ident),*) => {
        impl<$($T: Digestible),*> Digestible for ($($T),*) {
            fn digest<ByteOrder: byteorder::ByteOrder, Writer: DigestWriter>(&self, writer: &mut Writer) {
                let ($($T),*) = self;
                $($T.digest::<ByteOrder, Writer>(writer);)*
            }
        }
    };
}
tuple_configs!(A, B);
tuple_configs!(A, B, C);
tuple_configs!(A, B, C, D);
tuple_configs!(A, B, C, D, E);
tuple_configs!(A, B, C, D, E, F);
