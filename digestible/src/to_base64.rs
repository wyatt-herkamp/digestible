use crate::digester::Digester;
use crate::Digestible;
use alloc::string::String;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use byteorder::ByteOrder;
use core::fmt::{Debug, Formatter};
/// A Type Wrapper for a Digester that encodes the result into base64
///
/// Requires the `base64` feature
pub struct ToBase64<D>(D);
impl<D> ToBase64<D> {
    pub fn new(d: D) -> Self {
        Self(d)
    }

    pub fn into_inner(self) -> D {
        self.0
    }

    #[inline(always)]
    fn encode_base64(bytes: impl AsRef<[u8]>) -> String {
        STANDARD.encode(bytes.as_ref())
    }
}
impl<D: Debug> Debug for ToBase64<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ToBase64").field(&self.0).finish()
    }
}
impl<D: Clone> Clone for ToBase64<D> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<D> AsRef<D> for ToBase64<D> {
    fn as_ref(&self) -> &D {
        &self.0
    }
}

impl<D> AsMut<D> for ToBase64<D> {
    fn as_mut(&mut self) -> &mut D {
        &mut self.0
    }
}

/// Add a method to all Digester types that turns the digester into a [ToBase64](crate::ToBase64) type
///
/// Automatically implement Digester types that implement [Digestible](crate::Digestible)
/// with a Result that implements [AsRef](core::convert::AsRef) [u8]
pub trait IntoBase64: Sized {
    fn into_base64(self) -> ToBase64<Self>;
}

impl<D: Digester> IntoBase64 for D
where
    <D as Digester>::Target: AsRef<[u8]>,
{
    fn into_base64(self) -> ToBase64<Self> {
        ToBase64(self)
    }
}
impl<D: Digester> Digester for ToBase64<D>
where
    <D as Digester>::Target: AsRef<[u8]>,
{
    type Target = String;

    fn digest<B: ByteOrder>(self, data: &impl Digestible) -> Self::Target {
        Self::encode_base64(self.0.digest::<B>(data))
    }
}
