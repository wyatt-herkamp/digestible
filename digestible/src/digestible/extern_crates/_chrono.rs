use core::hash::Hash;

use chrono::{
    DateTime, Days, Duration, Month, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Weekday,
};

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
impl Digestible for NaiveDate {
    fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        let mut hashable_hack = HashableHack(writer);
        self.hash(&mut hashable_hack);
    }
}

impl Digestible for Weekday {
    fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        writer.write_u8(*self as u8);
    }
}
impl Digestible for Month {
    fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        writer.write_u8(*self as u8);
    }
}

impl Digestible for Days {
    fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        let mut hashable_hack = HashableHack(writer);
        self.hash(&mut hashable_hack);
    }
}

impl Digestible for NaiveTime {
    fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
        let mut hashable_hack = HashableHack(writer);
        self.hash(&mut hashable_hack);
    }
}
