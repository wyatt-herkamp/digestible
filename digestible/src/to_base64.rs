use crate::digester::Digester;
use crate::Digestible;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::fmt::{Debug, Formatter};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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

impl<D: Digester> IntoBase64 for D {
    fn into_base64(self) -> ToBase64<Self> {
        ToBase64(self)
    }
}
impl<D: Digester> Digester for ToBase64<D> {
    type Target = String;

    fn digest<DI: Digestible>(self, data: &DI) -> Self::Target {
        Self::encode_base64(self.0.digest(data))
    }

    fn digest_no_return<DI: Digestible>(&mut self, data: &DI) {
        self.0.digest_no_return(data)
    }
}