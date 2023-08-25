mod alloc_types;
mod atomic;
mod num_types;

use byteorder::ByteOrder;
use std::io;
use std::io::Write;

pub trait Digestible {
    type Digest<'a>: AsRef<[u8]>
    where
        Self: 'a;
    /// For Num Types, this is true
    #[inline(always)]
    fn constant_size() -> bool {
        false
    }
    #[inline(always)]
    fn supports_borrowed_digest() -> bool {
        false
    }
    /// Support returning a Digest Reference
    fn digest(&self) -> Self::Digest<'_>{
        unimplemented!("Can not digest Struct into Ref")
    }
    fn digest_to_writer<W: Write>(&self, writer: &mut W) -> Result<(), io::Error>;

    #[inline(always)]
    fn digest_owned(&self) -> Vec<u8> {
        let mut digest = Vec::new();
        self.digest_to_writer(&mut digest).unwrap();
        digest
    }
    #[inline(always)]
    fn digest_with_order<B: ByteOrder>(&self) -> Self::Digest<'_> {
        self.digest()
    }
    #[inline(always)]
    fn digest_owned_with_order<B: ByteOrder>(&self) -> Vec<u8>
    where
        Self: Sized,
    {
        let mut digest = Vec::new();
        self.digest_to_writer_with_order::<B,_>(&mut digest).unwrap();
        digest
    }

    #[inline(always)]
    fn digest_to_writer_with_order<B: ByteOrder, W: Write>(
        &self,
        writer: &mut W,
    ) -> Result<(), io::Error> {
        Self::digest_to_writer(self, writer)
    }
}
