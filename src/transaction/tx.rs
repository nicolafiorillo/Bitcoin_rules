use std::fmt::{Display, Formatter};

use rug::{integer::Order, Integer};

use crate::chain::tx::get_transaction;
use crate::flags::network::Network;
use crate::hashing::hash256::hash256;

use crate::scripting::context::Context;
use crate::scripting::script::Script;
use crate::transaction::{
    tx_error::TxError,
    tx_in::TxIn,
    tx_lib::{le_bytes_to_u32, varint_decode},
    tx_out::TxOut,
};

use crate::std_lib::varint::varint_encode;

use super::script_pub_key::ScriptPubKey;
use super::sighash::SIGHASH;
use super::tx_ins::TxIns;
use super::tx_outs::TxOuts;

// nLockTime
//      Block height or timestamp after which transaction can be added to the chain.
//      If >= 500000000 (Unix timestamp) -> timestamp; else -> block height.
//      Must be ignored when sequence numbers for all inputs are 0xFFFFFFFF.
#[derive(Debug, Clone)]
pub struct Tx {
    version: u32,
    inputs: TxIns,
    outputs: TxOuts,
    locktime: u32,
    network: Network,
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

    pub fn input_amount(&self) -> u64 {
        self.inputs.amount()
    }

    pub fn output_amount(&self) -> u64 {
        self.outputs.amount()
    }

    pub fn fee(&self) -> u64 {
        self.input_amount() - self.output_amount()
    }

    pub fn retrive_input_amount(&mut self) {
        self.inputs.retreive_amount()
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

    fn serialize(&self) -> Vec<u8> {
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
    fn hash_signature(&self, input_index: usize, script_pub_key: ScriptPubKey) -> Integer {
        // 1. take the transaction
        let mut tx: Tx = self.clone();

        // 2. remove all the ScriptSig of each input
        tx.inputs.remove_script();

        // 3. set the ScriptPubKey corresponding to the output the input is pointing to (the one you are spending) instead of its ScriptSig
        tx.inputs.substitute_script(input_index, script_pub_key);

        // 4. serialize the modified transaction
        let mut tx_serialized = tx.serialize();

        // 5. append the hash type
        let hash_type = (SIGHASH::All as u32).to_le_bytes().to_vec();
        tx_serialized = [tx_serialized, hash_type].concat();

        // 6. hash (hash256) the entire transaction
        let tx_hash = hash256(&tx_serialized);

        Integer::from_digits(&tx_hash, Order::Msf)
    }

    pub fn verify_input(&self, input_index: usize) -> Result<bool, TxError> {
        if &self.inputs.len() <= &input_index {
            return Err(TxError::InputIndexOutOfBounds);
        }

        let input_transaction = &self.inputs[input_index];
        let previous_transaction = match get_transaction(&input_transaction.previous_transaction_id, self.network) {
            Ok(tx) => tx,
            Err(_e) => return Err(TxError::TransactionNotFoundInChain),
        };

        let script_sig = match input_transaction.script_sig.script() {
            Ok(script) => script,
            Err(_e) => return Err(TxError::ScriptError),
        };

        let output_index = input_transaction.previous_transaction_index as usize;
        if &previous_transaction.outputs.len() <= &output_index {
            return Err(TxError::OutputIndexOutOfBounds);
        }

        let output_transaction = &previous_transaction.outputs[output_index];

        let script_pub_key = match output_transaction.script_pub_key.script() {
            Ok(script) => script,
            Err(_e) => return Err(TxError::ScriptError),
        };

        let z = self.hash_signature(input_index, output_transaction.script_pub_key.clone());
        let complete_script = Script::combine(script_sig, script_pub_key);

        let mut context = Context::new(complete_script.tokens(), z);

        match complete_script.evaluate(&mut context) {
            Err(e) => return Err(TxError::ScriptError),
            Ok(val) => Ok(val),
        }
    }

    /*
        Criteria to validate a transaction:

            1. The input of the transaction are previously unspent, to avoid double-spending
            2. The sum of the inputs is greater then or equal to the sum of the outputs. No new bitcoins must be created.
               The difference between the sum of the inputs and the sum of the outputs goes is the transaction fee for the miner.
            3. The ScriptSig in the input successfully unlocks the previous ScriptPubKey of the outputs.

        Other validations: https://developer.bitcoin.org/devguide/transactions.html#non-standard-transactions

            1. The transaction must be finalized: either its locktime must be in the past (or less than or equal to the current block height),
            or all of its sequence numbers must be 0xffffffff.

            2. The transaction must be smaller than 100,000 bytes. That’s around 200 times larger than a typical single-input,
            single-output P2PKH transaction.

            3. Each of the transaction’s signature scripts must be smaller than 1,650 bytes.
            That’s large enough to allow 15-of-15 multisig transactions in P2SH using compressed public keys.

            4. Bare (non-P2SH) multisig transactions which require more than 3 public keys are currently non-standard.

            5. The transaction’s signature script must only push data to the script evaluation stack.
            It cannot push new opcodes, with the exception of opcodes which solely push data to the stack.

            6. The transaction must not include any outputs which receive fewer than 1/3 as many satoshis as it would take
            to spend it in a typical input. That’s currently 546 satoshis for a P2PKH or P2SH output on a Bitcoin Core node
            with the default relay fee. Exception: standard null data outputs must receive zero satoshis.
    */
    pub fn validate(&self) -> Result<bool, TxError> {
        unimplemented!("Tx::validate");
    }
}

#[cfg(test)]
mod tx_test {
    use rug::Integer;

    use crate::{
        chain::tx::get_transaction,
        flags::network::Network,
        std_lib::{integer_ex::IntegerEx, vector::string_to_bytes},
        transaction::{tx::Tx, tx_error::TxError},
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
        let mut transaction = tx.unwrap();

        transaction.retrive_input_amount();

        assert_eq!(transaction.fee(), 140500);
    }

    #[test]
    fn verify_first_transaction_ever() {
        let satoshi_transaction_id: Integer =
            IntegerEx::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let satoshi_transaction = get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

        let res = satoshi_transaction.verify_input(0).unwrap();
        assert!(res);
    }

    #[test]
    fn verify_transaction_invalid_input_index() {
        let satoshi_transaction_id: Integer =
            IntegerEx::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let satoshi_transaction = get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

        let res = satoshi_transaction.verify_input(1);
        assert_eq!(TxError::InputIndexOutOfBounds, res.expect_err("Err"));
    }
}
