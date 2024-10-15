use digestible::Digestible;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

#[derive(Digestible)]
#[digestible(hash = LittleEndian)]
pub struct MyStructGenerics<T> {
    pub id: u32,
    pub t: T,
}
#[derive(Digestible)]
pub struct MyStructGenericsAlreadyRequired<T: Digestible> {
    pub id: u32,
    pub t: T,
}

#[test]
pub fn hash_test() {
    let test = MyStructGenerics {
        id: 0,
        t: "Test".to_string(),
    };
    let mut default_hasher = DefaultHasher::new();
    test.hash(&mut default_hasher);
}

#[derive(Digestible)]
pub enum MyEnum<T> {
    A(T),
    B(u32),
}
