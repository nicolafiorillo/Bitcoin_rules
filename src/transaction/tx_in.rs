use rug::Integer;

use crate::transaction::lib::tx_lib::{integer_to_le_32_bytes, u32_to_le_bytes};
use crate::transaction::script_sig::ScriptSig;

use super::lib::tx_lib::{le_32_bytes_to_integer, le_bytes_to_u32, varint_decode};
use super::tx_error::TxError;

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

    pub fn from_serialized(serialized: &[u8], mut cursor: usize) -> Result<(Self, usize), TxError> {
        let tx_in_previous_transaction_id = le_32_bytes_to_integer(serialized, cursor)?;
        cursor += 32;

        let tx_in_previous_transaction_index = le_bytes_to_u32(serialized, cursor)?;
        cursor += 4;

        let tx_in_scriptsig_length = varint_decode(serialized, cursor)?;
        cursor += tx_in_scriptsig_length.length;

        let tx_in_scriptsig_content_serialized = &serialized[cursor..cursor + tx_in_scriptsig_length.value as usize];
        let script_sig = ScriptSig::new(tx_in_scriptsig_content_serialized.to_vec());

        cursor += tx_in_scriptsig_length.value as usize;

        let tx_in_sequence = le_bytes_to_u32(serialized, cursor)?;
        cursor += 4;

        let tx_in: TxIn = TxIn::new(
            tx_in_previous_transaction_id,
            tx_in_previous_transaction_index,
            script_sig,
            tx_in_sequence,
        );

        Ok((tx_in, cursor))
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
