#[allow(unused_macros)]
macro_rules! impl_for_hashable_hack {
    ($hashable:ty) => {
        /// Implemented calling the hash method on the type
        impl Digestible for $hashable {
            fn digest<B: byteorder::ByteOrder, W: $crate::DigestWriter>(&self, writer: &mut W) {
                let mut hashable_hack = $crate::hash_digester::HashableHack(writer);
                <Self as core::hash::Hash>::hash(self, &mut hashable_hack);
            }
        }
    };
}
#[allow(unused_imports)]
pub(super) use impl_for_hashable_hack;
#[allow(unused_macros)]
macro_rules! impl_for_as_ref_u8 {
    ($as_ref_u8:ty) => {
        impl $crate::Digestible for $as_ref_u8 {
            fn digest<B: byteorder::ByteOrder, W: $crate::DigestWriter>(&self, writer: &mut W) {
                writer.write(self.as_ref())
            }
        }
    };
}
#[allow(unused_imports)]
pub(super) use impl_for_as_ref_u8;

#[allow(unused_macros)]
macro_rules! as_ref_then_call_inner {
    () => {
        fn digest<B: byteorder::ByteOrder, W: $crate::DigestWriter>(&self, writer: &mut W) {
            self.as_ref().digest::<B, W>(writer)
        }
    };
}
#[allow(unused_imports)]
pub(super) use as_ref_then_call_inner;
