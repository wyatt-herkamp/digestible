use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use byteorder::{ByteOrder, NativeEndian};
use chrono::NaiveDateTime;
use digestible::digester::Digester;
use digestible::digestible::DigestWith;
use digestible::to_base64::IntoBase64;
use digestible::DigestWriter;
use digestible_macros::Digestible;
use sha2::Digest;
use std::time::Duration;

pub struct DurationDigestWith;
impl DigestWith for DurationDigestWith {
    type Digest = Duration;

    fn digest<B: ByteOrder, W: DigestWriter>(digest: &Self::Digest, writer: &mut W) {
        writer.write(&digest.as_secs().to_ne_bytes());
        writer.write(&digest.subsec_nanos().to_ne_bytes());
    }
}
#[derive(Digestible)]
pub struct MyStruct {
    pub id: u32,
    pub name: String,
    #[digestible(skip)]
    pub password: String,
    #[digestible(use_std_hash)]
    pub duration: Duration,
    #[digestible(with = DurationDigestWith)]
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
    let result = hasher.digest::<NativeEndian, _>(&test);
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
    let result = hasher.digest::<NativeEndian, _>(&test);
    println!("{:?}", result);
}
#[derive(Digestible)]
pub struct CommonSimilarButDifferent {
    pub active: bool,
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
pub enum SimilarButDifferent {
    One { username: String },
    Two { name: String },
}
