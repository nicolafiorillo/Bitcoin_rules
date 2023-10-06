use rug::Integer;

use crate::{
    chain::tx::get_transaction,
    flags::network::Network,
    transaction::{
        script_sig::ScriptSig,
        tx_lib::{integer_to_le_32_bytes, u32_to_le_bytes},
    },
};

use super::{
    script_pub_key::ScriptPubKey,
    tx_error::TxError,
    tx_lib::{le_32_bytes_to_integer, le_bytes_to_u32},
};

#[derive(Debug, Clone)]
pub struct TxIn {
    pub previous_transaction_id: Integer, // will be u256
    pub previous_transaction_index: u32,
    pub previous_transaction_script_pubkey: Option<ScriptPubKey>,
    pub script_sig: ScriptSig,
    pub sequence: u32,
    pub network: Network, // TODO: to be removed when we can retreive transaction from real network
    pub amount: Option<u64>,
}

// TODO: manage errors with Result

impl TxIn {
    pub fn new(
        previous_transaction_id: Integer,
        previous_transaction_index: u32,
        script_sig: ScriptSig,
        sequence: u32,
        network: Network,
    ) -> TxIn {
        TxIn {
            previous_transaction_id,
            previous_transaction_index,
            script_sig,
            sequence,
            network,
            amount: None,
            previous_transaction_script_pubkey: None,
        }
    }

    pub fn remove_script(&mut self) {
        self.script_sig = ScriptSig::new(vec![0x00]);
    }

    pub fn substitute_script(&mut self, script_pub_key: ScriptPubKey) {
        self.script_sig = ScriptSig::new(script_pub_key.raw);
    }

    pub fn amount(&self) -> u64 {
        match self.amount {
            Some(amount) => amount,
            None => panic!("amount not calculated: first call calculate_amount()"),
        }
    }

    pub fn retreive_amount(&mut self) {
        log::debug!(
            "Searching for (previous) transaction_id {:x} (index {:?}) on network {:?}",
            &self.previous_transaction_id,
            self.previous_transaction_index,
            self.network
        );

        let tx = get_transaction(&self.previous_transaction_id, self.network);
        if tx.is_err() {
            panic!("(previous) transaction not found");
        }

        let previous_transaction = tx.unwrap();
        let amount = previous_transaction
            .outputs(self.previous_transaction_index as usize)
            .amount;

        self.amount = Some(amount);
    }

    // TODO: missing tests
    pub fn retreive_script_pubkey(&mut self) {
        log::debug!(
            "Searching for (previous) transaction_id {:x} (index {:?}) on network {:?}",
            &self.previous_transaction_id,
            self.previous_transaction_index,
            self.network
        );

        let tx = get_transaction(&self.previous_transaction_id, self.network);
        if tx.is_err() {
            panic!("(previous) transaction not found");
        }

        let previous_transaction = tx.unwrap();
        let script_pubkey = previous_transaction
            .outputs(self.previous_transaction_index as usize)
            .script_pub_key
            .clone();

        self.previous_transaction_script_pubkey = Some(script_pubkey);
    }

    pub fn from_serialized(serialized: &[u8], cursor: usize, network: Network) -> Result<(Self, usize), TxError> {
        let mut cur = cursor;

        let tx_in_previous_transaction_id = le_32_bytes_to_integer(serialized, cur)?;
        cur += 32;

        let tx_in_previous_transaction_index = le_bytes_to_u32(serialized, cur)?;
        cur += 4;

        let (script_sig, c) = ScriptSig::from_serialized(serialized, cur)?;
        cur = c;

        let tx_in_sequence = le_bytes_to_u32(serialized, cur)?;
        cur += 4;

        let tx_in: TxIn = TxIn::new(
            tx_in_previous_transaction_id,
            tx_in_previous_transaction_index,
            script_sig,
            tx_in_sequence,
            network,
        );

        Ok((tx_in, cur))
    }

    pub fn serialize(&self) -> Vec<u8> {
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
    use rug::Integer;

    use crate::{
        flags::network::Network,
        std_lib::integer_extended::IntegerExtended,
        std_lib::vector::string_to_bytes,
        transaction::{script_sig::ScriptSig, tx_in::TxIn},
    };

    #[test]
    fn test_tx_in_serialize() {
        let previous_transaction_id =
            Integer::from_hex_str("9E067AEDC661FCA148E13953DF75F8CA6EADA9CE3B3D8D68631769AC60999156");
        let previous_transaction_index: u32 = 1;
        let script_sig_content = string_to_bytes("47304402204585BCDEF85E6B1C6AF5C2669D4830FF86E42DD205C0E089BC2A821657E951C002201024A10366077F87D6BCE1F7100AD8CFA8A064B39D4E8FE4EA13A7B71AA8180F012102F0DA57E85EEC2934A82A585EA337CE2F4998B50AE699DD79F5880E253DAFAFB7").unwrap();
        let script_sig = ScriptSig::new(script_sig_content);
        let sequence: u32 = 4294967294;

        let tx_in = TxIn::new(
            previous_transaction_id,
            previous_transaction_index,
            script_sig,
            sequence,
            Network::Mainnet,
        );

        let tx_in_serialized = tx_in.serialize();

        let expected_tx_in_serialized =
            string_to_bytes("56919960AC691763688D3D3BCEA9AD6ECAF875DF5339E148A1FC61C6ED7A069E010000006A47304402204585BCDEF85E6B1C6AF5C2669D4830FF86E42DD205C0E089BC2A821657E951C002201024A10366077F87D6BCE1F7100AD8CFA8A064B39D4E8FE4EA13A7B71AA8180F012102F0DA57E85EEC2934A82A585EA337CE2F4998B50AE699DD79F5880E253DAFAFB7FEFFFFFF").unwrap();

        assert_eq!(tx_in_serialized, expected_tx_in_serialized);
    }
}
