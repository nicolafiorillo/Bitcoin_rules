use std::time::{Duration, SystemTime};

use diesel::prelude::*;

use core::std_lib::integer_extended::IntegerExtended;

use super::schema;

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::headers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Header {
    pub id: Vec<u8>,
    pub version: i32,
    pub previous_block: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub timestamp: std::time::SystemTime,
    pub bits: i32,
    pub nonce: i32,
}

#[derive(Debug, Clone, PartialEq, Insertable)]
#[diesel(table_name = schema::headers)]
pub struct NewHeader {
    pub id: Vec<u8>,
    pub version: i32,
    pub previous_block: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub timestamp: std::time::SystemTime,
    pub bits: i32,
    pub nonce: i32,
}

impl From<&core::block::header::Header> for NewHeader {
    fn from(header: &core::block::header::Header) -> Self {
        let t = SystemTime::UNIX_EPOCH + Duration::from_secs(header.timestamp as u64);

        Self {
            id: header.id().into(),
            version: header.version as i32,
            previous_block: header.previous_block.to_vec(),
            merkle_root: header.merkle_root.to_vec(),
            timestamp: t,
            bits: header.bits as i32,
            nonce: header.nonce as i32,
        }
    }
}

#[cfg(test)]
mod models_tests {
    use super::*;
    use core::{block::header::Header, std_lib::integer_extended::IntegerExtended};

    #[test]
    fn br_header_to_new_header() {
        //d: 00000000B873E79784647A6C82962C70D228557D24A747EA4D1B8BBE878E1206

        let br_header = Header {
            version: 1,
            previous_block: IntegerExtended::from_hex_str(
                "000000000933EA01AD0EE984209779BAAEC3CED90FA3F408719526F8D77F4943",
            ),
            merkle_root: IntegerExtended::from_hex_str(
                "F0315FFC38709D70AD5647E22048358DD3745F3CE3874223C80A7C92FAB0C8BA",
            ),
            timestamp: 1296688928,
            bits: 486604799,
            nonce: 1924588547,
        };

        let expected = NewHeader {
            id: br_header.id().into(),
            version: br_header.version as i32,
            previous_block: br_header.previous_block.to_vec(),
            merkle_root: br_header.merkle_root.to_vec(),
            timestamp: SystemTime::UNIX_EPOCH + Duration::from_secs(br_header.timestamp as u64),
            bits: br_header.bits as i32,
            nonce: br_header.nonce as i32,
        };

        assert_eq!(expected, (&br_header).into());
    }
}
