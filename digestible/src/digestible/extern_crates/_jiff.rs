use core::hash::Hash;

use jiff::{
    civil::{Date, DateTime, Era, ISOWeekDate, Time, Weekday},
    tz::{Dst, Offset},
    SignedDuration, SpanFieldwise, Timestamp, Unit, ZonedDateTime,
};

use crate::{hash_digester::HashableHack, Digestible};

macro_rules! digest_via_hash {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Digestible for $ty {
                fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
                    let mut hashable_hack = HashableHack(writer);
                    self.hash(&mut hashable_hack);
                }
            }
        )*
    };
}

macro_rules! digest_as_u8 {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Digestible for $ty {
                fn digest<B: byteorder::ByteOrder, W: crate::DigestWriter>(&self, writer: &mut W) {
                    writer.write_u8(*self as u8);
                }
            }
        )*
    };
}

// `Span` is intentionally excluded: jiff doesn't implement `Hash`/`Eq` for it because
// spans like `1 hour` and `60 minutes` are ambiguous. Use `SpanFieldwise` instead.
digest_via_hash!(
    Timestamp,
    ZonedDateTime,
    SignedDuration,
    SpanFieldwise,
    Date,
    Time,
    DateTime,
    ISOWeekDate,
    Offset,
);

digest_as_u8!(Weekday, Unit, Dst, Era);
