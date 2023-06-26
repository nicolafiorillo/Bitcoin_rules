use rug::Integer;

use super::script_sig::ScriptSig;

#[derive(Debug)]
pub struct TxIn {
    previous_transaction_id: Integer,
    previous_transaction_index: u32,
    script_sig: ScriptSig,
    sequence: u32,
}

impl TxIn {
    pub fn new(
        previous_transaction_id: Integer,
        previous_transaction_index: u32,
        script_sig: ScriptSig,
        sequence: u32,
    ) -> TxIn {
        TxIn {
            previous_transaction_id,
            previous_transaction_index,
            script_sig,
            sequence,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}
