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

