/*!
# Digesting Floats

## Example

```rust
use digestible::{Digestible, Digester};
use sha2::{Digest, Sha256};
#[derive(Digestible, Default)]
pub struct MyStruct{
    #[digestible(digest_with = floats::digest_round)]
    pub small_float: f32,
    #[digestible(digest_with = floats::digest_ceil)]
    pub big_float: f64,
    #[digestible(digest_with = floats::digest_floor)]
    pub optional_float: Option<f32>,
}

let mut hasher = sha2::Sha256::new();
let result = hasher.digest_native(&MyStruct::default());
assert_eq!(result.len(), 32);
```
*/
use crate::{DigestWriter, Digestible};
use byteorder::ByteOrder;

/// A Type Float type that can be digested
///
/// The functions return the FloatType as a IntType either i32 or i64
///
/// # Do not implement this trait. This is used internally to allow for a simple digest_with implementation
#[doc(hidden)]
pub trait FloatType {
    type TargetInt: Digestible;

    fn ceil(self) -> Self::TargetInt;

    fn floor(self) -> Self::TargetInt;

    fn round(self) -> Self::TargetInt;
}
macro_rules! de_ref_then_call_inner {
    (deref: $T:path) => {
        type TargetInt = <$T>::TargetInt;
        #[inline(always)]
        fn ceil(self) -> Self::TargetInt {
            (*self).ceil()
        }
        #[inline(always)]
        fn floor(self) -> Self::TargetInt {
            (*self).floor()
        }
        #[inline(always)]
        fn round(self) -> Self::TargetInt {
            (*self).round()
        }
    };
    ($tyToImpl:ident) =>{
        impl<T: FloatType + Copy> FloatType for $tyToImpl<T>{
             de_ref_then_call_inner!(deref: T);
        }
    };
}
impl<'a, F: FloatType + Copy> FloatType for &'a F {
    de_ref_then_call_inner!(deref: F);
}
impl<'a, T: FloatType + Copy> FloatType for &'a Option<T> {
    type TargetInt = Option<T::TargetInt>;
    #[inline(always)]
    fn ceil(self) -> Self::TargetInt {
        self.as_ref().map(|v| v.ceil())
    }
    #[inline(always)]
    fn floor(self) -> Self::TargetInt {
        self.as_ref().map(|v| v.floor())
    }
    #[inline(always)]
    fn round(self) -> Self::TargetInt {
        self.as_ref().map(|v| v.round())
    }
}
#[cfg(feature = "alloc")]
mod has_alloc {
    use crate::digest_with::floats::FloatType;
    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::sync::Arc;

    de_ref_then_call_inner!(Arc);
    de_ref_then_call_inner!(Rc);
    de_ref_then_call_inner!(Box);
}
macro_rules! impl_float_type {
    ($float:ty, $target:ty) => {
        impl FloatType for $float {
            type TargetInt = $target;
            #[inline(always)]
            fn ceil(self) -> Self::TargetInt {
                self.ceil() as $target
            }
            #[inline(always)]
            fn floor(self) -> Self::TargetInt {
                self.floor() as $target
            }
            #[inline(always)]
            fn round(self) -> Self::TargetInt {
                self.round() as $target
            }
        }
    };
}
impl_float_type!(f32, i32);
impl_float_type!(f64, i64);

/// Calls ceil on the float and then passes it to [`digest`](Digestible::digest)
/// [f32 Read More](f32::ceil) or [f64 Read More](f64::ceil)
#[inline(always)]
pub fn digest_ceil<B: ByteOrder, W: DigestWriter>(bytes: impl FloatType, writer: &mut W) {
    bytes.ceil().digest::<B, W>(writer)
}
/// Calls floor on the float and then passes it to [`digest`](Digestible::digest)
/// [f32 Read More](f32::floor) or [f64 Read More](f64::floor)
#[inline(always)]
pub fn digest_floor<B: ByteOrder, W: DigestWriter>(bytes: impl FloatType, writer: &mut W) {
    bytes.floor().digest::<B, W>(writer)
}
/// Calls round on the float and then passes it to [`digest`](Digestible::digest)
/// [f32 Read More](f32::round) or [f64 Read More](f64::round)
#[inline(always)]
pub fn digest_round<B: ByteOrder, W: DigestWriter>(bytes: impl FloatType, writer: &mut W) {
    bytes.round().digest::<B, W>(writer)
}
