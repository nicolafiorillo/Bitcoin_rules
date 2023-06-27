use rug::Integer;

use crate::transaction::lib::tx_lib::{integer_to_le_32_bytes, u32_to_le_bytes};
use crate::transaction::script_sig::ScriptSig;

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
        todo!(); // TODO: adding tests

        let previous_transaction_id_serialized = integer_to_le_32_bytes(&self.previous_transaction_id);
        let previous_transaction_index_serialized = u32_to_le_bytes(self.previous_transaction_index);
        let script_sig_serialized = self.script_sig.serialize();
        let sequence_serialized = u32_to_le_bytes(self.sequence);

        [
            previous_transaction_id_serialized.as_slice(),
            previous_transaction_index_serialized.as_slice(),
            script_sig_serialized.as_slice(),
            sequence_serialized.as_slice(),
        ]
        .concat()
    }
}

#[cfg(test)]
mod tx_in_test {

    // #[test]
    // fn test_tx_in_serialize() {}
}
