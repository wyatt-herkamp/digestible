use crate::digester::Digester;
use crate::Digestible;
use alloc::string::String;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use byteorder::ByteOrder;
use core::fmt::{Debug, Formatter};

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

    fn digest<B: ByteOrder, DI: Digestible>(self, data: &DI) -> Self::Target {
        Self::encode_base64(self.0.digest::<B, DI>(data))
    }

    fn digest_no_return<B: ByteOrder, DI: Digestible>(&mut self, data: &DI) {
        self.0.digest_no_return::<B, DI>(data)
    }
}
