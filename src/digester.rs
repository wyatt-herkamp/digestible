use crate::digestible::Digestible;

pub trait Digester {
    type Target: Into<Vec<u8>> + AsRef<[u8]>;

    fn digest<D: Digestible>(&mut self, data: &D);

    fn chain<D: Digestible>(mut self, data: &D) -> Self
    where
        Self: Sized,
    {
        self.digest(data);
        self
    }
}
#[cfg(feature = "digest")]
mod digest {
    use crate::digester::Digester;
    use crate::digestible::Digestible;
    use digest::Digest;

    impl<T: Digest> Digester for T {
        type Target = Vec<u8>;
        fn digest<D: Digestible>(&mut self, data: &D) {
            if D::supports_borrowed_digest() {
                self.update(data.digest().as_ref());
            } else {
                self.update(data.digest_owned());
            }
        }
    }
}
