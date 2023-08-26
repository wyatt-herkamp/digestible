#[cfg(feature = "std")]
mod std_type_id_hasher;

use crate::digester::SmallDigester;
use crate::Digestible;
use core::any::type_name;
use core::marker::PhantomData;

/// TODO: Make a better default hasher that will be more consistent across platforms
#[cfg(feature = "std")]
pub type DefaultTypeIDHasher = std_type_id_hasher::TypeIDHasher;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId<D: ?Sized> {
    id: u64,
    phantom: PhantomData<D>,
}
impl From<u64> for TypeId<()> {
    fn from(value: u64) -> Self {
        Self {
            id: value,
            phantom: PhantomData,
        }
    }
}

impl<D: SmallDigester<Target = u64> + ?Sized> TypeId<D> {
    pub fn new<T: ?Sized>() -> Self {
        Self::from_name(type_name::<T>())
    }
    pub fn from_name(name: &'static str) -> Self {
        Self {
            id: <D as SmallDigester>::digest(&name),
            phantom: PhantomData,
        }
    }
}

impl<D: ?Sized> Digestible for TypeId<D> {
    fn digest_to_writer<W: crate::DigestWriter>(&self, writer: &mut W) {
        self.id.digest_to_writer(writer);
    }
}
pub struct StructHeader<D: ?Sized>(TypeId<D>);
impl<D: SmallDigester<Target = u64> + ?Sized> StructHeader<D> {
    pub fn new<T: ?Sized>() -> Self {
        Self(TypeId::<D>::new::<T>())
    }
}

impl<D: ?Sized> Digestible for StructHeader<D> {
    fn digest_to_writer<W: crate::DigestWriter>(&self, writer: &mut W) {
        self.0.digest_to_writer(writer);
    }
}
/// Writes an Enum Header. This is so two Enum Variants with different names but the same data will have different digests
pub struct EnumHeader<D: ?Sized> {
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
    fn digest_to_writer<W: crate::DigestWriter>(&self, writer: &mut W) {
        self.id.digest_to_writer(writer);
        writer.write(b"::");
        self.variant_id.digest_to_writer(writer)
    }
}
