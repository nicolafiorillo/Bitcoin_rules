use rug::{integer::Order, Integer};
use std::fmt::{Display, Formatter};

use crate::{
    flags::{network::Network, sighash::SigHash},
    hashing::hash256::hash256,
    std_lib::varint::varint_encode,
};

use super::{
    script::Script,
    tx_error::TxError,
    tx_in::TxIn,
    tx_ins::TxIns,
    tx_lib::{le_bytes_to_u32, varint_decode},
    tx_out::TxOut,
    tx_outs::TxOuts,
};

// nLockTime
//   Block height or timestamp after which transaction can be added to the chain.
//   If >= 500000000 (Unix timestamp) -> timestamp; else -> block height.
//   Must be ignored when sequence numbers for all inputs are 0xFFFFFFFF.
#[derive(Debug, Clone)]
pub struct Tx {
    version: u32,
    inputs: TxIns,
    outputs: TxOuts,
    locktime: u32,
    pub network: Network,
}

// Ref. https://github.com/bitcoin/bitcoin/blob/b66f6dcb26906ca8187c7e54735e21168b8101c7/src/primitives/transaction.cpp#L106
impl Display for Tx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Id: {:}\nVersion: {:}\nLocktime: {:}\nNetwork: {:}\nInputs:\n{:}\nOutputs:\n{:}",
            self.id(),
            self.version,
            self.locktime,
            self.network,
            self.inputs,
            self.outputs
        )
    }
}

impl Tx {
    pub fn new(ins: &[TxIn], outs: &[TxOut], network: Network) -> Tx {
        let inputs = TxIns::new(ins.to_vec());
        let outputs = TxOuts::new(outs.to_vec());

        Tx {
            version: 1,
            inputs,
            outputs,
            locktime: 0,
            network,
        }
    }

    pub fn substitute_script(&mut self, index: usize, script: Script) {
        self.inputs.substitute_script(index, script);
    }

    pub fn id(&self) -> String {
        format!("{:02X}", Self::hash(&self.serialize()))
    }

    pub fn outputs(&self, index: usize) -> &TxOut {
        &self.outputs[index]
    }

    fn hash(bin: &[u8]) -> Integer {
        let serialized = hash256(bin);
        Integer::from_digits(&serialized, Order::Lsf)
    }

    // TODO renaming
    pub fn get_input(&self, index: usize) -> Result<&TxIn, TxError> {
        if self.inputs.len() <= index {
            return Err(TxError::InputIndexOutOfBounds);
        }

        Ok(&self.inputs[index])
    }

    // TODO renaming
    pub fn get_output(&self, index: usize) -> Result<&TxOut, TxError> {
        if self.outputs.len() <= index {
            return Err(TxError::OutputIndexOutOfBounds);
        }

        Ok(&self.outputs[index])
    }

    pub fn input_len(&self) -> usize {
        self.inputs.len()
    }

    pub fn output_len(&self) -> usize {
        self.outputs.len()
    }

    pub fn output_amount(&self) -> u64 {
        self.outputs.amount()
    }

    // TODO: implement with stream
    pub fn from_serialized(serialized: &[u8], network: Network) -> Result<Self, TxError> {
        if serialized.len() < 5 {
            return Err(TxError::InvalidTransactionLength);
        }

        let mut cursor: usize = 0;

        // Version
        let version = le_bytes_to_u32(serialized, cursor)?;
        cursor += 4;

        // Ref: https://en.bitcoin.it/wiki/Protocol_documentation#Message_structure
        let has_witness = serialized[cursor] == 0x00 && serialized[cursor + 1] == 0x01;
        if has_witness {
            cursor += 2;
        }

        // Inputs
        let tx_in_count = varint_decode(serialized, cursor)?;
        cursor += tx_in_count.length;

        let mut txs_in: Vec<TxIn> = vec![];

        for _ in 0..tx_in_count.value {
            let (tx_in, c) = TxIn::from_serialized(serialized, cursor, network)?;
            cursor = c;

            txs_in.push(tx_in);
        }

        // Outputs
        let tx_out_count = varint_decode(serialized, cursor)?;
        cursor += tx_out_count.length;

        let mut txs_out: Vec<TxOut> = vec![];

        for _ in 0..tx_out_count.value {
            let (tx_out, c) = TxOut::from_serialized(serialized, cursor)?;
            cursor = c;

            txs_out.push(tx_out);
        }

        // TODO: refactor when implementing SegWit
        if has_witness {
            for tx_in in txs_in.iter_mut() {
                let witness_count = varint_decode(serialized, cursor)?;
                cursor += witness_count.length;

                for _ in 0..witness_count.value {
                    let witness_length = varint_decode(serialized, cursor)?;
                    cursor += witness_length.length;

                    let witness = &serialized[cursor..cursor + witness_length.value as usize];
                    cursor += witness_length.value as usize;

                    tx_in.witnesses.push(witness.to_vec());
                }
            }
        }

        // Locktime
        let locktime = le_bytes_to_u32(serialized, cursor)?;
        cursor += 4;

        // final verification
        if cursor != serialized.len() {
            log::error!(
                "Transaction partially read. Cursor: {:?}, Serialized length: {:?}",
                cursor,
                serialized.len()
            );
            return Err(TxError::PartiallyReadTransaction);
        }

        let inputs = TxIns::new(txs_in);
        let outputs = TxOuts::new(txs_out);

        // Result transaction
        Ok(Tx {
            version,
            inputs,
            outputs,
            locktime,
            network,
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let version_serialized = self.version.to_le_bytes();
        let inputs_length = varint_encode(self.inputs.len() as u64);
        let inputs_serialized: Vec<u8> = self.inputs.serialize();
        let outputs_length = varint_encode(self.outputs.len() as u64);
        let outputs_serialized: Vec<u8> = self.outputs.serialize();
        let locktime_serialized = self.locktime.to_le_bytes();

        [
            version_serialized.as_slice(),
            inputs_length.as_slice(),
            inputs_serialized.as_slice(),
            outputs_length.as_slice(),
            outputs_serialized.as_slice(),
            locktime_serialized.as_slice(),
        ]
        .concat()
    }

    /*
       Validating a transaction involve validating the signature of each input.
       The signature of each input is calculated as follows:
        1. take the transaction
        2. remove all the ScriptSig of each input
        3. set the ScriptPubKey corresponding to the output the input is pointing to (the one you are spending) instead of its ScriptSig
        4. serialize the modified transaction
        5. append the hash type
        6. hash (hash256) the entire transaction
        And we have the transaction signature for input i.
       This signature, if correct, "unlocks" via OP_CHECKSIG the ScriptPubKey of the output that input i is pointing to.
    */
    pub fn hash_signature(&self, input_index: usize, script_pub_key: Script) -> Integer {
        // 1. take the transaction
        let mut tx: Tx = self.clone();

        // 2. remove all the ScriptSig of each input
        tx.inputs.remove_script();

        // 3. set the ScriptPubKey corresponding to the output the input is pointing to (the one you are spending) instead of its ScriptSig
        tx.inputs.substitute_script(input_index, script_pub_key);

        // 4. serialize the modified transaction
        let mut tx_serialized = tx.serialize();

        // 5. append the hash type
        let hash_type = (SigHash::All as u32).to_le_bytes().to_vec(); //TODO parametrize SIGHASH
        tx_serialized = [tx_serialized, hash_type].concat();

        // 6. hash (hash256) the entire transaction
        let tx_hash = hash256(&tx_serialized);

        Integer::from_digits(&tx_hash, Order::Msf)
    }
}

#[cfg(test)]
mod tx_test {

    use crate::{
        flags::network::Network,
        std_lib::vector::string_to_bytes,
        transaction::{tx::Tx, verification::fee},
    };

    pub const SERIALIZED_TRANSACTION: &str = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600";

    #[test]
    fn invalid_transaction_length() {
        let transaction: Vec<u8> = vec![0; 4];
        assert!(Tx::from_serialized(&transaction, Network::Mainnet).is_err());
    }

    #[test]
    fn deserialize_id() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::from_serialized(&transaction, Network::Mainnet);
        assert_eq!(
            tx.unwrap().id(),
            "EE51510D7BBABE28052038D1DEB10C03EC74F06A79E21913C6FCF48D56217C87"
        );
    }

    #[test]
    fn deserialize_version() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::from_serialized(&transaction, Network::Mainnet);
        assert_eq!(tx.unwrap().version, 1);
    }

    #[test]
    fn deserialize_tx_ins() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.inputs.len(), 4);
    }

    #[test]
    fn deserialize_tx_outs() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.outputs[0].amount, 1000273);
        assert_eq!(tx.outputs[1].amount, 40000000);
    }

    #[test]
    fn deserialize_tx_outs_amount1() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.outputs.len(), 2);
    }

    #[test]
    fn deserialize_locktime() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.locktime, 410438);
    }

    #[test]
    fn deserialize_and_serialize() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();
        let tx_serialized = tx.serialize();

        assert_eq!(transaction, tx_serialized);
    }

    #[test]
    fn deserialize_and_get_fee() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::from_serialized(&transaction, Network::Mainnet);
        let transaction = tx.unwrap();

        assert_eq!(fee(&transaction).unwrap(), 140500);
    }
}
