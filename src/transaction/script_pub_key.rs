use crate::{
    scripting::script::{Script, ScriptError},
    std_lib::varint::varint_encode,
};
use std::fmt::{Display, Formatter};

use super::{tx_error::TxError, tx_lib::varint_decode};

#[derive(Debug, Clone)]
pub struct ScriptPubKey {
    pub raw: Vec<u8>,
}

impl ScriptPubKey {
    pub fn new(content: Vec<u8>) -> Self {
        ScriptPubKey { raw: content }
    }

    pub fn from_serialized(serialized: &[u8], cursor: usize) -> Result<(Self, usize), TxError> {
        let mut cur = cursor;

        let scriptpubkey_length = varint_decode(serialized, cur)?;
        cur += scriptpubkey_length.length;

        let scriptpubkey_content_serialized = &serialized[cur..cur + scriptpubkey_length.value as usize];

        cur += scriptpubkey_length.value as usize;

        let script_pub_key = ScriptPubKey {
            raw: scriptpubkey_content_serialized.to_vec(),
        };

        Ok((script_pub_key, cur))
    }

    pub fn script(&self) -> Result<Script, ScriptError> {
        Script::deserialize(&self.raw, self.raw.len() as u64, 0)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let length = varint_encode(self.raw.len() as u64);
        [length.as_slice(), self.raw.as_slice()].concat()
    }
}

impl Display for ScriptPubKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self.script() {
            Ok(s) => s.representation(),
            Err(e) => format!("Cannot represent script: {:?}", e),
        };
        writeln!(f, "{:}", s)
    }
}
