# Digestible [![Build Status]][actions] [![Latest Version]][crates.io]
[Build Status]: https://img.shields.io/github/actions/workflow/status/wyatt-herkamp/digestible/commit.yml?branch=master
[actions]: https://github.com/wyatt-herkamp/digestible/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/digestible.svg
[crates.io]: https://crates.io/crates/digestible
A more dynamic Hash and Hashable trait for Rust

---

## Key Difference over STD Hash and Hashable
- ByteOrder is built in. So you can digest number types in any byte order. [ByteOrder](https://docs.rs/byteorder/latest/byteorder/)
- Output is a Generic Associated Type.
  So you can Digest into a ByteArray, 
  String with [Base64](https://docs.rs/digestible/0.2.0/digestible/to_base64/index.html)
  or any type the Digester uses.
- Skip Fields with `#[digestible(skip)]`
- 'digest_with' and 'with' to override the default digest behavior. [digest_with](https://docs.rs/digestible/0.2.0/digestible/digest_with/index.html)
- Support for all Hashing Algorithms that implement [digest::Digest](https://docs.rs/digest/latest/digest/) such as SHA2, md-5, and many more.
- Writing Type Headers to prevent collisions with similar types. (This is optional and can be disabled with `#[digestible(type_header = none)]`)
---

## Features
- No STD Support
- Digest to implement Digester for all types that implement [digest::Digest](https://docs.rs/digest/latest/digest/)
- Float and Atomic Support using `digest_with`

---
## Working with Hash
### For types that do not Implement Digestible
you can add `#[digestible(digest_with = digest_with_hash)]` to your variable
to tell Digestible to use Hash to digest it.

### Implementing Hash with Digestible. 
Adding `#[digestible(hash)]` to your struct or enum will implement Hash for it. 
Using the digest function. 
Allowing you to have a Hash and Digestible that are the same. 



## Example Using SHA2
```rust
#[derive(Digestible)]
pub struct MyStruct {
    pub id: u32,
    pub name: String,
    #[digestible(skip)]
    pub password: String,
}
fn digest_to_bytes(){
    let test = MyStruct{
        id: 0,
        name: "Test".to_string(),
        password: "Test".to_string(),
    };
    let mut hasher = sha2::Sha256::new();
    let result = hasher.digest_native(&test); // This will be the type of sha2 Output
}
fn digest_to_base64(){
    let test = MyStruct{
        id: 0,
        name: "Test".to_string(),
        password: "Test".to_string(),
    };
    let hasher = sha2::Sha256::new().into_base64();
    let result = hasher.digest_native(&test); // This is a base64 encoded string
}
```