use serde_json::{Number, Value};

use crate::{digestible::core_types::digest_iter, DigestWriter, Digestible};
/// Implement [`Digestible`] for [`serde_json::Value`].
impl Digestible for Value {
    fn digest<B: byteorder::ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        match self {
            Value::Null => writer.write_u8(0),
            Value::Bool(b) => writer.write_bool(*b),
            Value::Number(n) => {
                n.digest::<B, W>(writer);
            }
            Value::String(s) => s.digest::<B, W>(writer),
            Value::Array(a) => a.digest::<B, W>(writer),
            Value::Object(o) => {
                digest_iter::<(&String, &Value), B, W, _>(o.iter(), writer);
            }
        }
    }
}

impl Digestible for Number {
    fn digest<B: byteorder::ByteOrder, W: DigestWriter>(&self, writer: &mut W) {
        if let Some(n) = self.as_i64() {
            writer.write_i64::<B>(n);
        } else if let Some(n) = self.as_u64() {
            writer.write_u64::<B>(n);
        } else if let Some(n) = self.as_f64() {
            writer.write_f64::<B>(n);
        }
    }
    fn digest_native<W: DigestWriter>(&self, writer: &mut W) {
        if let Some(n) = self.as_i64() {
            n.digest_native(writer);
        } else if let Some(n) = self.as_u64() {
            n.digest_native(writer);
        } else if let Some(n) = self.as_f64() {
            n.digest_native(writer);
        }
    }
}
