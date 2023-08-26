///! This file is used to implement Digestible for chrono types.
///
/// This is done by using the HashableHack struct to write the data to a writer.
use crate::digestible::internal_macros::{impl_for_hashable_hack, use_hasher};
use crate::Digestible;
use chrono::{DateTime, Duration, NaiveDateTime, NaiveTime, TimeZone};
use crate::hash_digester::HashableHack;


impl_for_hashable_hack!(Duration);
impl_for_hashable_hack!(NaiveTime);
impl_for_hashable_hack!(NaiveDateTime);

impl<Tz: TimeZone> Digestible for DateTime<Tz> {
    fn digest_to_writer<W: crate::DigestWriter>(&self, writer: &mut W) {
        let mut hashable_hack = HashableHack::new(writer);
        self.hash(&mut hashable_hack);
    }
    use_hasher!();
}
#[cfg(test)]
mod tests {
    use crate::{DigestWriter, Digester, Digestible, IntoBase64};
    use sha2::Digest;
    pub struct ChronoTests {
        pub duration: chrono::Duration,
        pub naive_time: chrono::NaiveTime,
        pub naive_date_time: chrono::NaiveDateTime,
        pub date_time: chrono::DateTime<chrono::Utc>,
    }
    impl Default for ChronoTests {
        fn default() -> Self {
            Self {
                duration: chrono::Duration::seconds(0),
                naive_time: Default::default(),
                naive_date_time: Default::default(),
                date_time: Default::default(),
            }
        }
    }
    impl Digestible for ChronoTests {
        fn digest_to_writer<W: DigestWriter>(&self, writer: &mut W) {
            self.duration.digest_to_writer(writer);
            self.naive_time.digest_to_writer(writer);
            self.naive_date_time.digest_to_writer(writer);
            self.date_time.digest_to_writer(writer);
        }
    }
    #[test]
    pub fn test_chrono() {
        let chrono_tests = ChronoTests::default();
        let hasher = sha2::Sha256::new().into_base64();
        let result = hasher.digest(&chrono_tests);
        println!("{}", result)
    }
}
