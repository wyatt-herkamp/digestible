#[allow(unused_macros)]
macro_rules! use_hasher {
    () => {
        #[inline(always)]
        fn hash(&self, hasher: &mut impl core::hash::Hasher) {
            <Self as core::hash::Hash>::hash(self, hasher);
        }
    };
}
#[allow(unused_imports)]
pub(super) use use_hasher;

#[allow(unused_macros)]
macro_rules! impl_for_hashable_hack {
    ($hashable:ty) => {
        /// Implemented calling the hash method on the type
        impl Digestible for $hashable {
            fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
                
                let mut hashable_hack = crate::hash_digester::HashableHack::new(writer);
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
        impl Digestible for $as_ref_u8 {
            fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
                writer.write(self.as_ref())
            }
            crate::digestible::internal_macros::use_hasher!();
        }
    };
}
#[allow(unused_imports)]
pub(super) use impl_for_as_ref_u8;

#[allow(unused_macros)]
macro_rules! as_ref_then_call_inner {
    () => {
        fn digest<B: byteorder::ByteOrder, W: crate::DigesterWriter>(&self, writer: &mut W) {
            self.as_ref().digest::<B, W>(writer)
        }
        fn hash(&self, hasher: &mut impl core::hash::Hasher) {
            self.as_ref().hash(hasher)
        }
    };
}
#[allow(unused_imports)]
pub(super) use as_ref_then_call_inner;
