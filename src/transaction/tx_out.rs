use crate::transaction::script_pub_key::ScriptPubKey;

use super::{
    lib::tx_lib::{u64_le_bytes, varint_decode},
    tx_error::TxError,
};

#[derive(Debug)]
pub struct TxOut {
    pub amount: u64,
    script_pub_key: ScriptPubKey,
}

impl TxOut {
    pub fn new(amount: u64, script_pub_key: ScriptPubKey) -> TxOut {
        TxOut { amount, script_pub_key }
    }

    pub fn from_serialized(serialized: &[u8], mut cursor: usize) -> Result<(Self, usize), TxError> {
        let amount = u64_le_bytes(serialized, cursor)?;
        cursor += 8;

        let tx_out_scriptpubkey_length = varint_decode(serialized, cursor)?;
        cursor += tx_out_scriptpubkey_length.length;

        let tx_out_scriptpubkey_content_serialized =
            &serialized[cursor..cursor + tx_out_scriptpubkey_length.value as usize];
        let script_pub_key = ScriptPubKey::new(tx_out_scriptpubkey_content_serialized.to_vec());

        cursor += tx_out_scriptpubkey_length.value as usize;

        let tx_out = TxOut::new(amount, script_pub_key);

        Ok((tx_out, cursor))
    }

    pub fn serialize(&self) -> Vec<u8> {
        todo!(); // TODO: adding tests

        let amount_serialized = self.amount.to_le_bytes();
        let script_pub_key_serialized = self.script_pub_key.serialize();
        [amount_serialized.as_slice(), script_pub_key_serialized.as_slice()].concat()
    }
}

#[cfg(test)]
mod tx_out {

    // #[test]
    // fn test_tx_out_serialize() {}
}
