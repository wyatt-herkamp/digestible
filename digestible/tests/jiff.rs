#![cfg(feature = "jiff")]
use byteorder::NativeEndian;
use digestible::digester::Digester;
use digestible_macros::Digestible;
use sha2::Digest;
use jiff::{civil::date, Timestamp};

#[derive(Digestible)]
pub struct Event {
    pub at: Timestamp,
    pub day: jiff::civil::Date,
}

fn digest(event: &Event) -> Vec<u8> {
    sha2::Sha256::new()
        .digest::<NativeEndian>(event)
        .to_vec()
}

#[test]
pub fn equal_values_produce_equal_digests() {
    let a = Event {
        at: Timestamp::from_second(1_700_000_000).unwrap(),
        day: date(2024, 2, 29),
    };
    let b = Event {
        at: Timestamp::from_second(1_700_000_000).unwrap(),
        day: date(2024, 2, 29),
    };
    let c = Event {
        at: Timestamp::from_second(1_700_000_001).unwrap(),
        day: date(2024, 2, 29),
    };
    assert_eq!(digest(&a), digest(&b));
    assert_ne!(digest(&a), digest(&c));
}
