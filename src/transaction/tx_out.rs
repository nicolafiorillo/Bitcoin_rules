use crate::transaction::script_pub_key::ScriptPubKey;

use super::{tx_error::TxError, tx_lib::tx_lib::u64_le_bytes};

#[derive(Debug)]
pub struct TxOut {
    pub amount: u64,
    pub script_pub_key: ScriptPubKey,
}

impl TxOut {
    pub fn new(amount: u64, script_pub_key: ScriptPubKey) -> TxOut {
        TxOut { amount, script_pub_key }
    }

    pub fn from_serialized(serialized: &[u8], cursor: usize) -> Result<(Self, usize), TxError> {
        let mut cur = cursor;

        let amount = u64_le_bytes(serialized, cur)?;
        cur += 8;

        let (script_pub_key, c) = ScriptPubKey::from_serialized(serialized, cur)?;
        cur = c;

        let tx_out = TxOut::new(amount, script_pub_key);

        Ok((tx_out, cur))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let amount_serialized = self.amount.to_le_bytes();
        let script_pub_key_serialized = self.script_pub_key.serialize();
        [amount_serialized.as_slice(), script_pub_key_serialized.as_slice()].concat()
    }
}

#[cfg(test)]
mod tx_out {
    use crate::{std_lib::vector::string_to_bytes, transaction::script_pub_key::ScriptPubKey};

    use super::TxOut;

    #[test]
    fn test_tx_out_serialize() {
        let script_pub_key_content = string_to_bytes("76a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac");

        let script_pub_key = ScriptPubKey::new(script_pub_key_content);
        let tx_out = TxOut::new(40000000, script_pub_key);

        let tx_out_serialized = tx_out.serialize();

        let expected_tx_out_serialized =
            string_to_bytes("005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac");

        assert_eq!(tx_out_serialized, expected_tx_out_serialized);
    }
}
