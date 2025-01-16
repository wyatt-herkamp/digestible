use uuid::Uuid;

use crate::digestible::internal_macros::impl_for_as_ref_u8;

impl_for_as_ref_u8!(Uuid);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Digestible;
    use byteorder::LittleEndian;

    #[test]
    fn test_uuid() {
        let uuid = Uuid::new_v4();
        let mut writer: Vec<_> = Vec::new();
        uuid.digest::<LittleEndian, _>(&mut writer);
        assert_eq!(writer.len(), 16);
    }
}
