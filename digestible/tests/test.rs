use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use byteorder::{ByteOrder, NativeEndian};
use chrono::NaiveDateTime;
use digestible::digester::Digester;
use digestible::to_base64::IntoBase64;
use digestible::DigestWriter;
use digestible_macros::Digestible;
use sha2::Digest;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::time::Duration;

fn duration_digest_with<B: ByteOrder, W: DigestWriter>(digest: &Duration, writer: &mut W) {
    writer.write_u64::<B>(digest.as_secs());
}

#[derive(Digestible)]
pub struct MyStruct {
    pub id: u32,
    pub name: String,
    #[digestible(skip)]
    pub password: String,
    #[digestible(digest_with = digest_with_hash)]
    pub duration: Duration,
    #[digestible(with = duration_digest_with)]
    pub duration_two: Duration,
}
#[test]
pub fn test() {
    let test = MyStruct {
        id: 0,
        name: "Test".to_string(),
        password: "Test".to_string(),
        duration: Duration::from_secs(10),
        duration_two: Duration::from_secs(10),
    };
    let hasher = sha2::Sha256::new();
    let result = hasher.digest::<NativeEndian>(&test);
    let vec = STANDARD.encode(result.as_slice());

    println!("{:?}", vec);
}
#[test]
pub fn test_base64() {
    let test = MyStruct {
        id: 0,
        name: "Test".to_string(),
        password: "Test".to_string(),
        duration: Duration::from_secs(10),
        duration_two: Duration::from_secs(10),
    };
    let hasher = sha2::Sha256::new().into_base64();
    let result = hasher.digest::<NativeEndian>(&test);
    println!("{:?}", result);
}
#[derive(Digestible)]
pub struct TupleStruct(String);
#[derive(Digestible)]
pub struct CommonSimilarButDifferent {
    pub active: bool,
    #[digestible(digest_with = digest_with_hash)]
    pub created: NaiveDateTime,
}
#[derive(Digestible)]
pub struct SimilarButDifferentOne {
    pub id: u32,
    pub username: String,
    pub common: CommonSimilarButDifferent,
}
#[derive(Digestible)]
pub struct SimilarButDifferentTwo {
    pub id: u32,
    pub name: String,
    pub common: CommonSimilarButDifferent,
}
#[derive(Digestible)]
#[digestible(hash = LittleEndian)]
pub enum EnumExample {
    One { username: String },
    Two { name: String },
    None,
    Unit(String),
}
#[test]
pub fn hash_test() {
    let test = EnumExample::One {
        username: "Test".to_string(),
    };
    let mut default_hasher = DefaultHasher::new();
    test.hash(&mut default_hasher);
}
