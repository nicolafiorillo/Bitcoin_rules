use std::fmt::{Display, Formatter};

use crate::{flags::network::Network, keys::key::Key, scripting::standard, transaction::script::Script};

use super::{tx_error::TxError, tx_lib::u64_le_bytes};

#[derive(Debug, Clone)]
pub struct TxOut {
    pub amount: u64,
    pub script_pub_key: Script,
}

impl TxOut {
    pub fn new(amount: u64, script_pub_key: Script) -> TxOut {
        TxOut { amount, script_pub_key }
    }

    pub fn new_for_tx(amount: u64, address: &str, network: Network) -> Result<TxOut, TxError> {
        let address = match Key::address_to_hash160(address, network) {
            Ok(address) => address,
            Err(_) => return Err(TxError::InvalidAddress),
        };

        let script = standard::p2pkh_script(address);
        let tx_out = TxOut::new(amount, Script::new_from_script_lang(&script));

        Ok(tx_out)
    }

    pub fn from_serialized(serialized: &[u8], cursor: usize) -> Result<(Self, usize), TxError> {
        let mut cur = cursor;

        let amount = u64_le_bytes(serialized, cur)?;
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
        writeln!(f, "amount: {:}\nScriptPubKey: {:}", self.amount, self.script_pub_key,)
    }
}

#[cfg(test)]
mod tx_out {
    use crate::{std_lib::vector::string_to_bytes, transaction::script::Script};

    use super::TxOut;

    #[test]
    fn test_tx_out_serialize() {
        let script_pub_key_content = string_to_bytes("76a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac").unwrap();

        let script_pub_key = Script::new_from_raw(script_pub_key_content);
        let tx_out = TxOut::new(40000000, script_pub_key);

        let tx_out_serialized = tx_out.serialize();

        let expected_tx_out_serialized =
            string_to_bytes("005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac").unwrap();

        assert_eq!(tx_out_serialized, expected_tx_out_serialized);
    }
}
