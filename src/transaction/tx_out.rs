use super::script_pub_key::ScriptPubKey;

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
        vec![]
    }
}
