# Digestible
A more dynamic Hash and Hashable trait for Rust


## Example
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
    let result = hasher.digest(&test); // This will be the type of sha2 Output
}
fn digest_to_base64(){
    let test = MyStruct{
        id: 0,
        name: "Test".to_string(),
        password: "Test".to_string(),
    };
    let hasher = sha2::Sha256::new().into_base64();
    let result = hasher.digest(&test); // This is a base64 encoded string
}
```
Then you can select any hasher that implements Digester. 
When you enable the `digest` feature all hashes that implement [digezst::Digest](https://docs.rs/digest/latest/digest/) such as SHA2 will be available.

## Key Difference.
- ByteOrder is built in. So you can digest number types in any byte order.
- Output is a Generic Associated Type.
  So you can Digest into a ByteArray, String with [Base64](https://docs.rs/digestible/0.2.0-rc.1/digestible/struct.ToBase64.html)
  or any other that the Digester implements.
- Atomic Support Via [Digest With](https://docs.rs/digestible/0.2.0-rc.1/digestible/derive.Digestible.html#digest-with-example) and the [provided functions](https://docs.rs/digestible/0.2.0-rc.1/digestible/digest_with/atomics/index.html) 