# Digestible
A better more dynamic Hash and Hashable trait for Rust


## Usage
```rust
#[derive(Digestible)]
pub struct MyStruct {
    pub id: u32,
    pub name: String,
    #[digestible(skip)]
    pub password: String,
}
```
Then you can select any hasher that implements Digester. 
When you enable the `digest` feature all hashes that implement [digest::Digest](https://docs.rs/digest/latest/digest/) such as SHA2 will be available.

