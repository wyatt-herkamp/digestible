use std::hash::Hasher;
use std::io::Write;
/// Foreign Types that do not implement Digestible and do not have a way to access the inner data
/// ## Example
/// ```rust, no_run
/// use digestible::Digestible;
/// use std::hash::Hash;
/// use std::io::Write;
/// #[derive(Hash)]
/// pub struct MyHashableType(u32);
/// use digestible::digestible::hash_hack::HashableHack;
/// impl Digestible for MyHashableType {
///     fn digest_to_writer<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
///         let mut hashable_hack = HashableHack::new(writer);
///         self.hash(&mut hashable_hack);
///         Ok(())
///     }
/// }
/// ```
pub struct HashableHack<'a, W: Write>(&'a mut W);
impl<W: Write> Drop for HashableHack<'_, W> {
    /// Just a precaution. Most likely not needed because it should be written to a buffer
    fn drop(&mut self) {
        self.0.flush().expect("Failed to flush hashable hack");
    }
}
impl<'a, W: Write> HashableHack<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self(writer)
    }
}
impl<W: Write> Hasher for HashableHack<'_, W> {
    /// Does nothing.
    fn finish(&self) -> u64 {
        0
    }
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.0
            .write_all(bytes)
            .expect("Failed to write to hashable hack")
    }
}
