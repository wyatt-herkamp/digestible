use crate::digestible::Digestible;

use byteorder::{ByteOrder, NativeEndian};

pub trait Digester {
    type Target;

    fn digest<B: ByteOrder, D: Digestible>(self, data: &D) -> Self::Target;

    fn digest_native_order<D: Digestible>(self, data: &D) -> Self::Target
    where
        Self: Sized,
    {
        self.digest::<NativeEndian, D>(data)
    }

    fn digest_no_return<B: ByteOrder, D: Digestible>(&mut self, data: &D);
}

/// Automatically implement Digester for all types that implement [Digest](digest::Digest)
///
/// Giving you access to use [sha2](https://crates.io/crates/sha2), [sha1](https://crates.io/crates/sha1), [md-5](https://crates.io/crates/md-5) and more
#[cfg(feature = "digest_0_10")]
mod digest {
    use crate::digester::Digester;
    use crate::digestible::Digestible;
    use byteorder::ByteOrder;
    use digest_0_10::{Digest, Output};

    impl<T: Digest> Digester for T {
        type Target = Output<T>;
        fn digest<B: ByteOrder, D: Digestible>(mut self, data: &D) -> Self::Target {
            self.digest_no_return::<B, D>(data);
            self.finalize()
        }
        /// Use this if you want to build a digest from multiple Digestible types

        fn digest_no_return<B: ByteOrder, D: Digestible>(&mut self, data: &D) {
            let mut digest_consumer = DigestConsumerInner(self);
            data.digest::<B, _>(&mut digest_consumer);
        }
    }
    struct DigestConsumerInner<'digest, T: Digest>(&'digest mut T);

    impl<D: Digest> crate::DigestWriter for DigestConsumerInner<'_, D> {
        /// Calls [Digest::update](digest::Digest::update) on the digest with the given data.
        #[inline]
        fn write(&mut self, buf: &[u8]) {
            self.0.update(buf);
        }
    }
}
