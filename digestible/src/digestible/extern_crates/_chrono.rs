use core::hash::Hash;

use chrono::{DateTime, Duration, NaiveDateTime, TimeZone};

use crate::{hash_digester::HashableHack, Digestible};

impl Digestible for NaiveDateTime {
    fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        let mut hashable_hack = HashableHack(writer);
        self.hash(&mut hashable_hack);
    }
}
impl<Tz: TimeZone> Digestible for DateTime<Tz> {
    fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        let mut hashable_hack = HashableHack(writer);
        self.hash(&mut hashable_hack);
    }
}
impl Digestible for Duration {
    fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        let mut hashable_hack = HashableHack(writer);
        self.hash(&mut hashable_hack);
    }
}
