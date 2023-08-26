use crate::digestible::internal_macros::as_ref_then_call_inner;
use crate::digestible::Digestible;
use std::sync::Arc;

macro_rules! digestible_atomic {
    ($atomic:ty, $size:literal) => {
        impl Digestible for $atomic {
            fn digest_to_writer<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
                self.load(core::sync::atomic::Ordering::Relaxed)
                    .digest_to_writer(writer)
            }

            #[inline(always)]
            fn digest_owned(&self) -> Vec<u8> {
                let mut digest = Vec::with_capacity($size);
                self.digest_to_writer(&mut digest).unwrap();
                digest
            }

            #[inline(always)]
            fn digest_owned_with_order<B: byteorder::ByteOrder>(&self) -> Vec<u8> {
                let mut digest = Vec::with_capacity($size);
                self.load(core::sync::atomic::Ordering::Relaxed)
                    .digest_to_writer_with_order::<B, _>(&mut digest)
                    .unwrap();
                digest
            }

            #[inline(always)]
            fn digest_to_writer_with_order<B: byteorder::ByteOrder, W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> std::io::Result<()> {
                self.load(core::sync::atomic::Ordering::Relaxed)
                    .digest_to_writer(writer)
            }
        }
    };
}

digestible_atomic!(std::sync::atomic::AtomicBool, 1);
digestible_atomic!(std::sync::atomic::AtomicU8, 1);
digestible_atomic!(std::sync::atomic::AtomicU16, 2);
digestible_atomic!(std::sync::atomic::AtomicU32, 4);
digestible_atomic!(std::sync::atomic::AtomicU64, 8);
digestible_atomic!(std::sync::atomic::AtomicI8, 1);
digestible_atomic!(std::sync::atomic::AtomicI16, 2);
digestible_atomic!(std::sync::atomic::AtomicI32, 4);
digestible_atomic!(std::sync::atomic::AtomicI64, 8);

impl<T: Digestible> Digestible for Arc<T> {
    as_ref_then_call_inner!();
}
