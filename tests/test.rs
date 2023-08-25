use digest_macros::Digestible;
use digestible::digester::Digester;
use sha2::Digest;
use base64::engine::general_purpose::STANDARD;
use base64::{ Engine};
#[derive(Digestible)]
pub struct MyStruct {
    pub id: u32,
    pub name: String,
    #[digestible(skip)]
    pub password: String,
}
#[test]
pub fn test(){
    let test = MyStruct{
        id: 0,
        name: "Test".to_string(),
        password: "Test".to_string(),
    };
    let mut hasher = sha2::Sha256::new();
    hasher.digest(&test);
    let result = hasher.finalize();
    let vec = STANDARD.encode(result.as_slice());

    println!("{:?}", vec);

}