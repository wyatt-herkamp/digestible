# Digestible Macro [Digestible](https://docs.rs/digestible/latest/digestible/derive.Digestible.html)

Implement the [Digestible](https://docs.rs/digestible/latest/digestible/digestible/trait.Digestible.html) trait for the given struct or enum.

This will push all data one after another into the [DigestWriter](https://docs.rs/digestible/latest/digestible/trait.DigestWriter.html).

No padding or spaces are added. Similar to how [Hash](https://doc.rust-lang.org/std/hash/derive.Hash.html) works.


## Container Attributes
- type_header Sets how the type header is written [TypeHeader](https://docs.rs/digestible/latest/digestible/index.html#type-headers)
    * none: No type header is written `#[digestible(type_header = none)]`
    * HashName: The name of the hash is written as the type header (Default) `#[digestible(type_header = HashName)]`

## Field Attributes
- skip: Skips the field when digesting
- use_std_hash: Uses [HashableHack](https://docs.rs/digestible/latest/digestible/hash_digester/struct.HashableHack.html) to digest the field
- digest_with: Path to Function that follows this signature: `fn digest<B: ByteOrder, W: DigestWriter>(digest: Type, writer: &mut W);`

### Digest With Example
```rust
extern crate digestible;

use digestible::{Digestible, DigestWriter};
use digestible::byteorder::ByteOrder;
use digestible::hash_digester::HashableHack;
use core::time::Duration;

fn duration_digest_with<B: ByteOrder, W: DigestWriter>(digest: &Duration, writer: &mut W) {
    writer.write_u64::<B>(digest.as_secs());
}
```