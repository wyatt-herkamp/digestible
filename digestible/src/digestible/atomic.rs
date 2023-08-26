use crate::digestible::Digestible;

macro_rules! digestible_atomic {
    ($atomic:ty, $size:literal) => {
        impl Digestible for $atomic {
            fn digest_to_writer<W: crate::DigesterWriter>(&self, writer: &mut W) {
                self.load(core::sync::atomic::Ordering::Relaxed)
                    .digest_to_writer(writer);
            }

            #[inline(always)]
            fn digest_to_writer_with_order<B: byteorder::ByteOrder, W: crate::DigesterWriter>(
                &self,
                writer: &mut W,
            ) {
                self.load(core::sync::atomic::Ordering::Relaxed)
                    .digest_to_writer(writer);
            }
        }
        #[cfg(feature = "alloc")]
        impl super::DigestibleAlloc for $atomic {
            #[inline(always)]
            fn digest_owned(&self) -> alloc::vec::Vec<u8> {
                let mut digest = alloc::vec::Vec::with_capacity($size);
                self.digest_to_writer(&mut digest);
                digest
            }
            #[inline(always)]
            fn digest_owned_with_order<B: byteorder::ByteOrder>(&self) -> alloc::vec::Vec<u8> {
                let mut digest = alloc::vec::Vec::with_capacity($size);
                self.load(core::sync::atomic::Ordering::Relaxed)
                    .digest_to_writer_with_order::<B, _>(&mut digest);
                digest
            }
        }
    };
}

digestible_atomic!(core::sync::atomic::AtomicBool, 1);
digestible_atomic!(core::sync::atomic::AtomicU8, 1);
digestible_atomic!(core::sync::atomic::AtomicU16, 2);
digestible_atomic!(core::sync::atomic::AtomicU32, 4);
digestible_atomic!(core::sync::atomic::AtomicU64, 8);
digestible_atomic!(core::sync::atomic::AtomicI8, 1);
digestible_atomic!(core::sync::atomic::AtomicI16, 2);
digestible_atomic!(core::sync::atomic::AtomicI32, 4);
digestible_atomic!(core::sync::atomic::AtomicI64, 8);
