use crate::digestible::Digestible;

macro_rules! digestible_atomic {
    ($atomic:ty, $size:literal) => {
        impl Digestible for $atomic {
            #[inline(always)]
            fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
                self.load(core::sync::atomic::Ordering::Relaxed)
                    .digest::<B, W>(writer);
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
