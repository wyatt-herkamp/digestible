/*!
# Digesting Atomic Types

This module provides a way to digest atomic types.

## Example

```rust
use digestible::{Digestible, Digester};
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU8, AtomicU32, AtomicU16, AtomicU64, AtomicBool, Ordering};
#[derive(Digestible, Default)]
pub struct MyStruct{
    #[digestible(digest_with = atomics::digest_seq_cst)]
    pub u8: AtomicU8,
    #[digestible(digest_with = atomics::digest_relaxed)]
    pub u16: AtomicU16,
    #[digestible(digest_with = atomics::digest_acquire)]
    pub bool: AtomicBool,
}

let mut hasher = sha2::Sha256::new();
let result = hasher.digest_native(&MyStruct::default());
assert_eq!(result.len(), 32);
```
*/
use crate::Digestible;
use core::sync::atomic::Ordering;

/// This is used internally to allow for a simple digest_with implementation
///
/// # Do not implement this trait. This is used internally to allow for a simple digest_with implementation
#[doc(hidden)]
pub trait AtomicType {
    /// The type that is returned when the atomic is loaded
    type TargetType: Digestible;
    /// Loads the atomic with the given ordering
    fn load(&self, order: Ordering) -> Self::TargetType;
}
impl<'a, T: AtomicType> AtomicType for &'a T {
    type TargetType = T::TargetType;

    fn load(&self, order: Ordering) -> Self::TargetType {
        (**self).load(order)
    }
}
#[cfg(feature = "alloc")]
mod has_alloc {
    use crate::digest_with::atomics::AtomicType;
    use alloc::boxed::Box;
    use alloc::rc::Rc;
    use alloc::sync::Arc;
    use core::sync::atomic::Ordering;

    impl<T: AtomicType> AtomicType for Arc<T> {
        type TargetType = T::TargetType;

        fn load(&self, order: Ordering) -> Self::TargetType {
            self.as_ref().load(order)
        }
    }
    impl<T: AtomicType> AtomicType for Rc<T> {
        type TargetType = T::TargetType;

        fn load(&self, order: Ordering) -> Self::TargetType {
            self.as_ref().load(order)
        }
    }
    impl<T: AtomicType> AtomicType for Box<T> {
        type TargetType = T::TargetType;

        fn load(&self, order: Ordering) -> Self::TargetType {
            self.as_ref().load(order)
        }
    }
}
macro_rules! digestible_atomic {

    ($(($atomic:ty, $target:ty)),*) =>{
        $(
            impl AtomicType for $atomic{
                type TargetType = $target;

                fn load(&self, ordering: Ordering) -> Self::TargetType {
                    self.load(ordering)
                }
        }
        )*
    };
}
digestible_atomic!(
    (core::sync::atomic::AtomicBool, bool),
    (core::sync::atomic::AtomicU8, u8),
    (core::sync::atomic::AtomicU16, u16),
    (core::sync::atomic::AtomicU32, u32),
    (core::sync::atomic::AtomicU64, u64),
    (core::sync::atomic::AtomicI8, i8),
    (core::sync::atomic::AtomicI16, i16),
    (core::sync::atomic::AtomicI32, i32),
    (core::sync::atomic::AtomicI64, i64),
    (core::sync::atomic::AtomicUsize, usize),
    (core::sync::atomic::AtomicIsize, isize)
);
macro_rules! digest_with {
    (
        $ordering:path,
        $name:ident
    ) => {
        #[doc = concat!("Loads the Digestible Data from the Atomic. Using Ordering [", stringify!($ordering), "](core::sync::atomic::",stringify!($ordering),")")]
        #[doc = "\n\n AtomicType is implemented for all Atomic Types except AtomicPtr"]
        #[inline(always)]
        pub fn $name<B: byteorder::ByteOrder, W: crate::DigestWriter>(atomic: &impl AtomicType, writer: &mut W) {
            atomic.load($ordering).digest::<B, W>(writer);
        }
    };
    ($(($ordering:path, $name:ident)),*) => {
        $(
            digest_with!($ordering, $name);
        )*
    };
}
digest_with!(
    (Ordering::Relaxed, digest_relaxed),
    (Ordering::Acquire, digest_acquire),
    (Ordering::SeqCst, digest_seq_cst)
);
