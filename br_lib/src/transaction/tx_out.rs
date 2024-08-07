use std::fmt::{Display, Formatter};

use crate::{std_lib::std_result::StdResult, transaction::script::Script};

use super::tx_lib::le_bytes_to_u64;

#[derive(Debug, Clone)]
pub struct TxOut {
    pub amount: u64,
    pub script_pub_key: Script,
}

impl TxOut {
    pub fn new(amount: u64, script_pub_key: Script) -> TxOut {
        TxOut { amount, script_pub_key }
    }

    pub fn deserialize(serialized: &[u8], cursor: usize) -> StdResult<(Self, usize)> {
        let mut cur = cursor;

        let amount = le_bytes_to_u64(serialized, cur)?;
        cur += 8;

        let (script_pub_key, c) = Script::deserialize(serialized, cur)?;
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

impl Display for TxOut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "    amount: {:}\n    ScriptPubKey: {:}",
            self.amount, self.script_pub_key,
        )
    }
}

#[cfg(test)]
mod tx_out {
    use crate::{std_lib::vector::hex_string_to_bytes, transaction::script::Script};

    use super::TxOut;

    #[test]
    fn test_tx_out_serialize() {
        let script_pub_key_content = hex_string_to_bytes("76a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac").unwrap();

        let script_pub_key = Script::new_from_raw(script_pub_key_content);
        let tx_out = TxOut::new(40000000, script_pub_key);

        let tx_out_serialized = tx_out.serialize();

        let expected_tx_out_serialized =
            hex_string_to_bytes("005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac").unwrap();

        assert_eq!(tx_out_serialized, expected_tx_out_serialized);
    }
}
