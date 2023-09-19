use super::{tx_error::TxError, tx_lib::varint_decode};
use crate::{
    encoding::varint::varint_encode,
    scripting::script::{Script, ScriptError},
};

use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct ScriptSig {
    raw: Vec<u8>,
}

impl ScriptSig {
    pub fn new(content: Vec<u8>) -> Self {
        ScriptSig { raw: content }
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
        let length = varint_encode(self.raw.len() as u64);
        [length.as_slice(), self.raw.as_slice()].concat()
    }

    pub fn script(&self) -> Result<Script, ScriptError> {
        Script::deserialize(&self.raw, self.raw.len() as u64, 0)
    }
}

impl Display for ScriptSig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self.script() {
            Ok(s) => s.representation(),
            Err(e) => format!("Cannot represent script: {:?}", e),
        };
        writeln!(f, "{:}", s)
    }
}
