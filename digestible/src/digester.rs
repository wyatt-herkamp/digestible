use crate::digestible::Digestible;

use byteorder::ByteOrder;
/// A Type that can Digest data into a Target.
pub trait Digester {
    type Target;

    /// Digest the Data into the Target using the Given ByteOrder
    ///
    ///
    /// ## Parameters
    /// - data: The data to digest
    /// ## Type Parameters
    /// - B: The [ByteOrder](crate::byteorder::ByteOrder) to use
    /// - D: The [Digestible](crate::Digestible) type to digest
    ///
    /// ## Example
    /// ```rust
    /// use byteorder::NativeEndian;
    /// use digestible::{Digester, Digestible};
    /// use sha2::{Digest, Sha256};
    /// use std::time::Duration;
    ///
    /// #[derive(Digestible)]
    /// pub struct MyStruct {
    ///     pub id: u32,
    ///     pub name: String,
    ///     #[digestible(skip)]
    ///     pub password: String,
    ///     #[digestible(use_std_hash)]
    ///     pub duration: Duration,
    /// }
    ///
    /// let test = MyStruct {
    ///     id: 0,
    ///     name: "Test".to_string(),
    ///     password: "Test".to_string(),
    ///     duration: Duration::from_secs(10),
    /// };
    /// let hasher = sha2::Sha256::new();
    /// let result = hasher.digest::<NativeEndian>(&test);
    /// ```
    fn digest<B: ByteOrder>(self, data: &impl Digestible) -> Self::Target;
    /// Calls [digest](Self::digest) with [NativeEndian](byteorder::NativeEndian) as the ByteOrder
    fn digest_native(self, data: &impl Digestible) -> Self::Target
    where
        Self: Sized,
    {
        self.digest::<byteorder::NativeEndian>(data)
    }
    /// Calls [digest](Self::digest) with [byteorder::BigEndian](byteorder::BigEndian) as the ByteOrder
    fn digest_big_endian(self, data: &impl Digestible) -> Self::Target
    where
        Self: Sized,
    {
        self.digest::<byteorder::BigEndian>(data)
    }
    /// Calls [digest](Self::digest) with [byteorder::LittleEndian](byteorder::LittleEndian) as the ByteOrder
    fn digest_little_endian(self, data: &impl Digestible) -> Self::Target
    where
        Self: Sized,
    {
        self.digest::<byteorder::LittleEndian>(data)
    }
}

/// Automatically implement Digester for all types that implement [Digest](digest::Digest)
///
/// Giving you access to use [sha2](https://crates.io/crates/sha2), [sha1](https://crates.io/crates/sha1), [md-5](https://crates.io/crates/md-5) and more
#[cfg(feature = "digest_0_10")]
mod digest_0_10 {
    use crate::digester::Digester;
    use crate::digestible::Digestible;
    use byteorder::ByteOrder;
    use digest_0_10::{Digest, Output};
    impl<T: Digest> Digester for T {
        type Target = Output<T>;
        fn digest<B: ByteOrder>(mut self, data: &impl Digestible) -> Self::Target {
            data.digest::<B, _>(&mut DigestConsumerInner(&mut self));
            self.finalize()
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
