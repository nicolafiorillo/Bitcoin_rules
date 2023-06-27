use crate::transaction::script_pub_key::ScriptPubKey;

#[derive(Debug)]
pub struct TxOut {
    pub amount: u64,
    script_pub_key: ScriptPubKey,
}

impl TxOut {
    pub fn new(amount: u64, script_pub_key: ScriptPubKey) -> TxOut {
        TxOut { amount, script_pub_key }
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
