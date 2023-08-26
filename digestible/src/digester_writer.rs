pub trait DigestWriter {
    fn write(&mut self, data: &[u8]);
}
#[cfg(feature = "alloc")]
mod has_alloc {
    use crate::DigestWriter;
    use alloc::vec::Vec;

    impl DigestWriter for Vec<u8> {
        fn write(&mut self, data: &[u8]) {
            self.extend_from_slice(data);
        }
    }
}
