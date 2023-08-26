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
macro_rules! digest_owned_with_size {
    (fn size($self:ident: &$t:ty) -> usize $body:block) => {
        #[inline(always)]
        fn digest_owned(&self) -> alloc::vec::Vec<u8> {
            fn size($self: &$t) -> usize $body
            let mut digest = alloc::vec::Vec::with_capacity(size(self));
            self.digest_to_writer(&mut digest);
            digest
        }
        #[inline(always)]
        fn digest_owned_with_order<B: byteorder::ByteOrder>(&self) -> alloc::vec::Vec<u8> {
            fn size($self: &$t) -> usize $body
            let mut digest = alloc::vec::Vec::with_capacity(size(self));
            self.digest_to_writer_with_order::<B, _>(&mut digest);
            digest
        }
    };
    ($size:literal) => {
        #[inline(always)]
        fn digest_owned(&self) -> alloc::vec::Vec<u8> {
            let mut digest = alloc::vec::Vec::with_capacity($size);
            self.digest_to_writer(&mut digest);
            digest
        }

        #[inline(always)]
        fn digest_owned_with_order<B: ByteOrder>(&self) -> alloc::vec::Vec<u8> {
            let mut digest = alloc::vec::Vec::with_capacity($size);
            self.digest_to_writer_with_order::<B, _>(&mut digest);
            digest
        }
    };
    ($size:path) => {
        #[inline(always)]
        fn digest_owned(&self) -> alloc::vec::Vec<u8> {
            let mut digest = alloc::vec::Vec::with_capacity($size());
            self.digest_to_writer(&mut digest);
            digest
        }

        #[inline(always)]
        fn digest_owned_with_order<B: ByteOrder>(&self) -> alloc::vec::Vec<u8> {
            let mut digest = alloc::vec::Vec::with_capacity($size());
            self.digest_to_writer_with_order::<B, _>(&mut digest);
            digest
        }
    };
}

#[allow(unused_imports)]
pub(super) use digest_owned_with_size;

#[allow(unused_macros)]
macro_rules! impl_for_hashable_hack {
    ($hashable:ty) => {
        /// Implemented calling the hash method on the type
        impl Digestible for $hashable {
            fn digest_to_writer<W: crate::DigesterWriter>(&self, writer: &mut W) {
                use core::hash::Hash;
                let mut hashable_hack = crate::hash_digester::HashableHack::new(writer);
                <Self as core::hash::Hash>::hash(self, &mut hashable_hack);
            }
            fn hash(&self, hasher: &mut impl core::hash::Hasher) {
                <Self as core::hash::Hash>::hash(self, hasher);
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
            fn digest_to_writer<W: crate::DigesterWriter>(&self, writer: &mut W) {
                writer.write(self.as_ref())
            }
            crate::digestible::internal_macros::use_hasher!();
        }
    };
}
#[allow(unused_imports)]
pub(super) use impl_for_as_ref_u8;

#[allow(unused_macros)]
macro_rules! impl_alloc_with_size_call {
    ($as_ref_u8:ty, fn size($this:ident: $this_ty:ty) -> usize $meth:block) => {
        impl crate::DigestibleAlloc for $as_ref_u8 {
            #[inline(always)]
            fn digest_owned(&self) -> Vec<u8> {
                fn size($this: $this_ty) -> usize $meth
                let mut digest = Vec::with_capacity(size(self));
                self.digest_to_writer(&mut digest);
                digest
            }

            #[inline(always)]
            fn digest_owned_with_order<B: ByteOrder>(&self) -> Vec<u8> {
                fn size($this: $this_ty) -> usize $meth
                let mut digest = Vec::with_capacity(size(self));
                self.digest_to_writer_with_order::<B, _>(&mut digest);
                digest
            }

        }
    };
}
#[allow(unused_imports)]
pub(super) use impl_alloc_with_size_call;
#[allow(unused_macros)]
macro_rules! as_ref_then_call_inner {
    () => {
        fn digest_to_writer<W: crate::DigesterWriter>(&self, writer: &mut W) {
            self.as_ref().digest_to_writer(writer)
        }
        fn digest_to_writer_with_order<B: byteorder::ByteOrder, W: crate::DigesterWriter>(
            &self,
            writer: &mut W,
        ) {
            self.as_ref().digest_to_writer_with_order::<B, W>(writer)
        }
        fn hash(&self, hasher: &mut impl core::hash::Hasher) {
            self.as_ref().hash(hasher)
        }
    };
}
#[allow(unused_imports)]
pub(super) use as_ref_then_call_inner;
#[allow(unused_macros)]
macro_rules! as_ref_then_call_inner_alloc {
    () => {
        fn digest_owned(&self) -> alloc::vec::Vec<u8> {
            self.as_ref().digest_owned()
        }
        fn digest_owned_with_order<B: byteorder::ByteOrder>(&self) -> alloc::vec::Vec<u8> {
            self.as_ref().digest_owned_with_order::<B>()
        }
    };
}
#[allow(unused_imports)]
pub(super) use as_ref_then_call_inner_alloc;
