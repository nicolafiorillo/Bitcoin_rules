use crate::encoding::varint::varint_encode;

use super::{tx_error::TxError, tx_lib::varint_decode};

#[derive(Debug, Clone)]
pub struct ScriptPubKey {
    pub content: Vec<u8>,
}

impl ScriptPubKey {
    pub fn new(content: Vec<u8>) -> Self {
        ScriptPubKey { content }
    }

    pub fn from_serialized(serialized: &[u8], cursor: usize) -> Result<(Self, usize), TxError> {
        let mut cur = cursor;

        let scriptpubkey_length = varint_decode(serialized, cur)?;
        cur += scriptpubkey_length.length;

        let scriptpubkey_content_serialized = &serialized[cur..cur + scriptpubkey_length.value as usize];

        let script_pub_key = ScriptPubKey::new(scriptpubkey_content_serialized.to_vec());

        cur += scriptpubkey_length.value as usize;

        Ok((script_pub_key, cur))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let length = varint_encode(self.content.len() as u64);
        [length.as_slice(), self.content.as_slice()].concat()
    }
}
