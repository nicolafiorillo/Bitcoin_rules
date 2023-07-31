use crate::transaction::varint::varint_encode;

use super::{tx_error::TxError, tx_lib::varint_decode};

#[derive(Debug)]
pub struct ScriptSig {
    content: Vec<u8>,
}

impl ScriptSig {
    pub fn new(content: Vec<u8>) -> Self {
        ScriptSig { content }
    }

    pub fn from_serialized(serialized: &[u8], cursor: usize) -> Result<(Self, usize), TxError> {
        let mut cur = cursor;

        let scriptsig_length = varint_decode(serialized, cur)?;
        cur += scriptsig_length.length;

        let scriptsig_content_serialized = &serialized[cur..cur + scriptsig_length.value as usize];
        let script_sig = ScriptSig::new(scriptsig_content_serialized.to_vec());

        cur += scriptsig_length.value as usize;

        Ok((script_sig, cur))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let length = varint_encode(self.content.len() as u64);
        [length.as_slice(), self.content.as_slice()].concat()
    }
}
