#[cfg(feature = "std")]
mod std_type_id_hasher;

use crate::{DigestWriter, Digester, Digestible};
use byteorder::{ByteOrder, NativeEndian};
use core::any::type_name;
use core::marker::PhantomData;

/// TODO: Make a better default hasher that will be more consistent across platforms
#[cfg(feature = "std")]
pub type DefaultTypeIDHasher = std_type_id_hasher::TypeIDHasher;
#[cfg(not(feature = "std"))]
pub type DefaultTypeIDDigester = NoSTDHasher;

/// TypeID is a identifier for a type that is consistent across platforms and compilers.
/// This is used to mark a type so hashes are different for different types.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId<D: ?Sized = DefaultTypeIDHasher> {
    id: u128,
    /// Only used when creating a TypeID
    hasher: PhantomData<D>,
}
impl From<u128> for TypeId {
    fn from(value: u128) -> Self {
        Self {
            id: value,
            hasher: Default::default(),
        }
    }
}
impl Into<u128> for TypeId {
    fn into(self) -> u128 {
        self.id
    }
}

impl<D: Digester<Target = u128> + Default + ?Sized> TypeId<D> {
    pub fn new<T: ?Sized>() -> Self {
        Self::from_name(type_name::<T>())
    }
    pub fn from_name(name: &'static str) -> Self {
        Self {
            // It is string. ByteOrder does not matter
            id: D::default().digest::<NativeEndian, _>(&name),
            hasher: PhantomData,
        }
    }
    pub const fn from_id(id: u128) -> Self {
        Self {
            id,
            hasher: PhantomData,
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructHeader(u128);

impl Digestible for StructHeader {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        self.0.digest::<B, _>(writer);
    }
}
impl StructHeader {
    pub const fn new(id: u128) -> Self {
        Self(id)
    }
    pub const fn new_from_typeid(id: TypeId) -> Self {
        Self(id.id)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumHeader(u128, u128);

impl Digestible for EnumHeader {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        self.0.digest::<B, _>(writer);
        writer.write(b"::");
        self.1.digest::<B, _>(writer);
    }
}
impl EnumHeader {
    pub const fn new(id: u128, discriminate: u128) -> Self {
        Self(id, discriminate)
    }
    pub const fn new_from_type_id(id: TypeId, discriminate: u128) -> Self {
        Self(id.id, discriminate)
    }
}
