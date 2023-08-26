#[cfg(feature = "std")]
mod std_type_id_hasher;

use crate::digester::SmallDigester;
use crate::{DigestWriter, Digester, Digestible};
use byteorder::{ByteOrder, NativeEndian};
use core::any::type_name;
use core::marker::PhantomData;

/// TODO: Make a better default hasher that will be more consistent across platforms
#[cfg(feature = "std")]
pub type DefaultTypeIDHasher = std_type_id_hasher::TypeIDHasher;
#[cfg(not(feature = "std"))]
pub type DefaultTypeIDDigester = NoSTDHasher;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId<D: ?Sized = DefaultTypeIDHasher, B: ByteOrder = NativeEndian> {
    id: u64,
    hasher: PhantomData<D>,
    byte_order: PhantomData<B>,
}
impl<B: ByteOrder> From<u64> for TypeId<(), B> {
    fn from(value: u64) -> Self {
        Self {
            id: value,
            hasher: Default::default(),
            byte_order: Default::default(),
        }
    }
}

impl<D: SmallDigester<Target = u64> + ?Sized, B: ByteOrder> TypeId<D, B> {
    pub fn new<T: ?Sized>() -> Self {
        Self::from_name(type_name::<T>())
    }
    pub fn from_name(name: &'static str) -> Self {
        Self {
            id: <D as SmallDigester>::digest::<B, _>(&name),
            hasher: Default::default(),
            byte_order: Default::default(),
        }
    }
}

impl<D: ?Sized> Digestible for TypeId<D> {
    fn digest<B: ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        writer.write_u64::<B>(self.id)
    }
}
pub struct StructHeader<D: ?Sized = DefaultTypeIDHasher>(TypeId<D>);
impl<D: SmallDigester<Target = u64> + ?Sized> StructHeader<D> {
    pub fn new<T: ?Sized>() -> Self {
        Self(TypeId::<D>::new::<T>())
    }
}

impl<D: ?Sized> Digestible for StructHeader<D> {
    fn digest<B: ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        self.0.digest::<B, W>(writer);
    }
}
/// Writes an Enum Header. This is so two Enum Variants with different names but the same data will have different digests
pub struct EnumHeader<D: ?Sized = DefaultTypeIDHasher> {
    id: TypeId<D>,
    variant_id: TypeId<D>,
}
impl<D: SmallDigester<Target = u64> + ?Sized> EnumHeader<D> {
    pub fn new<T: ?Sized>(variant_name: &'static str) -> Self {
        Self {
            id: TypeId::<D>::new::<T>(),
            variant_id: TypeId::<D>::from_name(variant_name),
        }
    }
}
impl<D> Digestible for EnumHeader<D> {
    fn digest<B: ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        self.id.digest::<B, W>(writer);
        writer.write(b"::");
        self.variant_id.digest::<B, W>(writer);
    }
}
pub struct NoSTDHasher;
impl Digester for NoSTDHasher {
    type Target = u64;

    fn digest<B: ByteOrder, D: Digestible>(self, _data: &D) -> Self::Target {
        panic!("NoSTDHasher does not support digesting")
    }

    fn digest_no_return<B: ByteOrder, D: Digestible>(&mut self, _data: &D) {
        panic!("NoSTDHasher does not support digesting")
    }
}
impl SmallDigester for NoSTDHasher {
    fn digest<B: ByteOrder, D: Digestible>(_data: &D) -> Self::Target {
        panic!("NoSTDHasher does not support digesting")
    }
}
