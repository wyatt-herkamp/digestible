use crate::digestible::Digestible;

pub trait Digester {
    type Target: AsRef<[u8]>;

    fn digest<D: Digestible>(self, data: &D) -> Self::Target;

    fn digest_no_return<D: Digestible>(&mut self, data: &D);
}

/// Automatically implement Digester for all types that implement [Digest](digest::Digest)
///
/// Giving you access to use [sha2](https://crates.io/crates/sha2), [sha1](https://crates.io/crates/sha1), [md-5](https://crates.io/crates/md-5) and more
#[cfg(feature = "digest")]
mod digest {
    use crate::digester::Digester;
    use crate::digestible::Digestible;
    use digest::{Digest, Output};
    use std::io::Write;

    impl<T: Digest> Digester for T {
        type Target = Output<T>;
        fn digest<D: Digestible>(mut self, data: &D) -> Self::Target {
            self.digest_no_return(data);
            self.finalize()
        }
        /// Use this if you want to build a digest from multiple Digestible types
        fn digest_no_return<D: Digestible>(&mut self, data: &D) {
            let mut digest_consumer = DigestConsumerInner(self);
            data.digest_to_writer(&mut digest_consumer).unwrap();
        }
    }
    struct DigestConsumerInner<'digest, T: Digest>(&'digest mut T);

    impl<D: Digest> Write for DigestConsumerInner<'_, D> {
        /// Calls [Digest::update](digest::Digest::update) on the digest with the given data.
        #[inline]
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.update(buf);
            Ok(buf.len())
        }
        /// Flushing is a no-op for a digest.
        #[inline(always)]
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
}
