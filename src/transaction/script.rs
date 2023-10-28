use std::fmt::{Display, Formatter};

use crate::{scripting::script_lang::ScriptLang, std_lib::varint::varint_encode};

use super::{tx_error::TxError, tx_lib::varint_decode};

#[derive(Debug, Clone)]
pub struct Script {
    pub raw: Vec<u8>,
    pub script_lang: ScriptLang,
}

impl Script {
    pub fn new_from_raw(raw: Vec<u8>) -> Self {
        let script_lang = ScriptLang::deserialize(&raw, raw.len() as u64, 0).unwrap();

        Script { raw, script_lang }
    }

    pub fn new_from_script_lang(scrip_lang: &ScriptLang) -> Self {
        let raw = scrip_lang.serialize().unwrap();

        Script {
            raw,
            script_lang: scrip_lang.clone(),
        }
    }

    pub fn new_empty() -> Self {
        Script::new_from_raw(Vec::<u8>::new())
    }

    pub fn deserialize(serialized: &[u8], cursor: usize) -> Result<(Self, usize), TxError> {
        let mut cur = cursor;

        let scriptsig_length = varint_decode(serialized, cur)?;
        cur += scriptsig_length.length;

        let scriptsig_content_serialized = &serialized[cur..cur + scriptsig_length.value as usize];
        let script_sig = Script::new_from_raw(scriptsig_content_serialized.to_vec());

        cur += scriptsig_length.value as usize;

        Ok((script_sig, cur))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let length = varint_encode(self.raw.len() as u64);
        [length.as_slice(), self.raw.as_slice()].concat()
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.script_lang.representation();
        write!(f, "{:}", s)
    }
}
