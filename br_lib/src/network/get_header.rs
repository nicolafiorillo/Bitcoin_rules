use crate::{
    hashing::hash256::Hash256,
    std_lib::varint::{encode, VarInt},
};

use super::constants;

// https://en.bitcoin.it/wiki/Protocol_documentation#getheaders
#[derive(Debug, PartialEq)]
pub struct GetHeader {
    pub version: u32, // LE
    hashes: VarInt,
    start_block: Hash256,
    end_block: Hash256,
}

impl GetHeader {
    pub fn new(start_block: Hash256, end_block: Hash256) -> Self {
        let version = constants::LAST_VERSION;
        let hashes = VarInt::new(1, 1);

        Self {
            version,
            hashes,
            start_block,
            end_block,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];

        v.extend_from_slice(&self.version.to_le_bytes());
        v.extend_from_slice(&encode(self.hashes.value));

        v.extend_from_slice(&self.start_block.0);
        v.extend_from_slice(&self.end_block.0);
        v
    }
}
