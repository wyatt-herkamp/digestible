#[allow(unused_macros)]
macro_rules! use_hasher {
    () => {
        fn digest_to_hasher(&self, hasher: &mut impl core::hash::Hasher) {
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
        fn digest_owned(&self) -> Vec<u8> {
            fn size($self: &$t) -> usize $body
            let mut digest = Vec::with_capacity(size(self));
            self.digest_to_writer(&mut digest).unwrap();
            digest
        }
        #[inline(always)]
        fn digest_owned_with_order<B: byteorder::ByteOrder>(&self) -> Vec<u8> {
            fn size($self: &$t) -> usize $body
            let mut digest = Vec::with_capacity(size(self));
            self.digest_to_writer_with_order::<B, _>(&mut digest)
                .unwrap();
            digest
        }
    };
    ($size:literal) => {
        #[inline(always)]
        fn digest_owned(&self) -> Vec<u8> {
            let mut digest = Vec::with_capacity($size);
            self.digest_to_writer(&mut digest).unwrap();
            digest
        }

        #[inline(always)]
        fn digest_owned_with_order<B: ByteOrder>(&self) -> Vec<u8> {
            let mut digest = Vec::with_capacity($size);
            self.digest_to_writer_with_order::<B, _>(&mut digest)
                .unwrap();
            digest
        }
    };
    ($size:path) => {
        #[inline(always)]
        fn digest_owned(&self) -> Vec<u8> {
            let mut digest = Vec::with_capacity($size());
            self.digest_to_writer(&mut digest).unwrap();
            digest
        }

        #[inline(always)]
        fn digest_owned_with_order<B: ByteOrder>(&self) -> Vec<u8> {
            let mut digest = Vec::with_capacity($size());
            self.digest_to_writer_with_order::<B, _>(&mut digest)
                .unwrap();
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
            fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
                use std::hash::Hash;
                let mut hashable_hack = crate::digestible::hash_hack::HashableHack::new(writer);
                self.hash(&mut hashable_hack);
                Ok(())
            }
            crate::digestible::internal_macros::use_hasher!();
        }
    };
}
#[allow(unused_imports)]
pub(super) use impl_for_hashable_hack;
#[allow(unused_macros)]
macro_rules! impl_for_as_ref_u8 {
    ($as_ref_u8:ty) => {
        impl Digestible for $as_ref_u8 {
            fn digest_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
                writer.write_all(self.as_ref())
            }
            crate::digestible::internal_macros::digest_owned_with_size!(
                fn size(this: &$as_ref_u8) -> usize {
                    this.len()
                }
            );
            crate::digestible::internal_macros::use_hasher!();
        }
    };
}
#[allow(unused_imports)]
pub(super) use impl_for_as_ref_u8;
#[allow(unused_macros)]
macro_rules! as_ref_then_call_inner {
    () => {
        fn digest_to_writer<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
            self.as_ref().digest_to_writer(writer)
        }
        fn digest_owned(&self) -> Vec<u8> {
            self.as_ref().digest_owned()
        }
        fn digest_owned_with_order<B: byteorder::ByteOrder>(&self) -> Vec<u8> {
            self.as_ref().digest_owned_with_order::<B>()
        }
        fn digest_to_writer_with_order<B: byteorder::ByteOrder, W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> std::io::Result<()> {
            self.as_ref().digest_to_writer_with_order::<B, W>(writer)
        }
        fn digest_to_hasher(&self, hasher: &mut impl std::hash::Hasher) {
            self.as_ref().digest_to_hasher(hasher)
        }
    };
}
#[allow(unused_imports)]
pub(super) use as_ref_then_call_inner;
