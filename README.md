# Digestible
A more dynamic Hash and Hashable trait for Rust


## Usage
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
When you enable the `digest` feature all hashes that implement [digest::Digest](https://docs.rs/digest/latest/digest/) such as SHA2 will be available.

## Features

## ByteOrder
ByteOrder is applied to types that use byteorder such as numbers and floats.

### Atomic
Atomics can be used to create a hash. When you enabled the `atomic` feature. Ordering relaxed is used for all atomics.

### to_base64
The result will automatically be encoded to base64. 
When you call `into_base64` on the hasher. 
The hasher must output a type that impl `AsRef<[u8]>` 
This will be the default when you enable the `digest` feature.

### Chrono
Chrono Types are implemented using the `hash` feature.

### Float
Implements Digestible for f32 and f64. 

#### Notes
The Digest Writer will always provide the write_f32 and write_f64 functions.

