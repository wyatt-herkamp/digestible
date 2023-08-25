use crate::digestible::Digestible;
use byteorder::ByteOrder;
use std::io::{Error, Write};
use std::sync::atomic::Ordering;
use std::sync::Arc;
pub trait DigestibleAtomic {
    type TargetType: Digestible;

    fn fetch(&self, ordering: Ordering) -> Self::TargetType;
}
macro_rules! digestible_atomic {
    ($target:ty, $atomic:ty) => {
        impl DigestibleAtomic for $atomic {
            type TargetType = $target;
            fn fetch(&self, ordering: Ordering) -> Self::TargetType {
                self.load(ordering)
            }
        }
    };
}

digestible_atomic!(u8, std::sync::atomic::AtomicU8);
digestible_atomic!(u16, std::sync::atomic::AtomicU16);
digestible_atomic!(u32, std::sync::atomic::AtomicU32);
digestible_atomic!(u64, std::sync::atomic::AtomicU64);
digestible_atomic!(i8, std::sync::atomic::AtomicI8);
digestible_atomic!(i16, std::sync::atomic::AtomicI16);
digestible_atomic!(i32, std::sync::atomic::AtomicI32);
digestible_atomic!(i64, std::sync::atomic::AtomicI64);

impl<'atomic, T: DigestibleAtomic> Digestible for &'atomic T {
    type Digest<'a>  = <<T as DigestibleAtomic>::TargetType as Digestible>::Digest<'a> where Self: 'a;

    fn digest(&self) -> Self::Digest<'_> {
        unimplemented!("Atomic Types do not support borrowed digest")
    }

    fn digest_with_order<B: ByteOrder>(&self) -> Self::Digest<'_> {
        unimplemented!("Atomic Types do not support borrowed digest")
    }

    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        self.fetch(Ordering::Relaxed).digest_to_writer(writer)
    }
}

impl<T> Digestible for Arc<T>
where
    for<'b> T: Digestible + 'b,
{
    type Digest<'a>= <T as Digestible>::Digest<'a> where T: 'a;

    fn digest(&self) -> Self::Digest<'_> {
        self.as_ref().digest()
    }

    fn supports_borrowed_digest() -> bool {
        T::supports_borrowed_digest()
    }

    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        self.as_ref().digest_to_writer(writer)
    }
}
