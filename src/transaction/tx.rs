use std::fmt::{Display, Formatter};

use rug::{integer::Order, Integer};

use crate::bitcoin::network::Network;
use crate::hashing::hash256::hash256;

use crate::transaction::{
    tx_error::TxError,
    tx_in::TxIn,
    tx_lib::{le_bytes_to_u32, varint_decode},
    tx_out::TxOut,
};

use crate::encoding::varint::varint_encode;

// nLockTime
//      Block height or timestamp after which transaction can be added to the chain.
//      If >= 500000000 (Unix timestamp) -> timestamp; else -> block height.
//      Must be ignored when sequence numbers for all inputs are 0xFFFFFFFF.
#[derive(Debug)]
pub struct Tx {
    pub version: u32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    locktime: u32,
    network: Network,
}

impl Display for Tx {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Transaction: {:?}\nVersion: {:?}\nLocktime: {:?}\nNetwork: {:?}",
            self.id(),
            self.version,
            self.locktime,
            self.network
        )
    }
}

impl Tx {
    pub fn id(&self) -> String {
        format!("{:x}", self.hash())
    }

    fn hash(&self) -> Integer {
        let serialized = hash256(&self.serialize());
        Integer::from_digits(&serialized, Order::Lsf)
    }

    pub fn input_amount(&self) -> u64 {
        self.inputs.iter().fold(0u64, |acc, i: &TxIn| acc + i.amount())
    }

    pub fn output_amount(&self) -> u64 {
        self.outputs.iter().fold(0u64, |acc, i: &TxOut| acc + i.amount)
    }

    pub fn fee(&self) -> u64 {
        self.input_amount() - self.output_amount()
    }

    pub fn retrive_input_amount(&mut self) {
        for i in 0..self.inputs.len() {
            self.inputs[i].retreive_amount();
        }
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

        let mut inputs: Vec<TxIn> = vec![];

        for _ in 0..tx_in_count.value {
            let (tx_in, c) = TxIn::from_serialized(serialized, cursor, network)?;
            cursor = c;

            inputs.push(tx_in);
        }

        // Outputs
        let tx_out_count = varint_decode(serialized, cursor)?;
        cursor += tx_out_count.length;

        let mut outputs: Vec<TxOut> = vec![];

        for _ in 0..tx_out_count.value {
            let (tx_out, c) = TxOut::from_serialized(serialized, cursor)?;
            cursor = c;

            outputs.push(tx_out);
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
        let inputs_serialized: Vec<u8> = self.inputs.iter().flat_map(|i| i.serialize()).collect();
        let outputs_length = varint_encode(self.outputs.len() as u64);
        let outputs_serialized: Vec<u8> = self.outputs.iter().flat_map(|i| i.serialize()).collect();
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
}

#[cfg(test)]
mod tx_test {
    use crate::{bitcoin::network::Network, std_lib::vector::string_to_bytes, transaction::tx::Tx};

    pub const SERIALIZED_TRANSACTION: &str = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600";

    #[test]
    fn invalid_transaction_length() {
        let transaction: Vec<u8> = vec![0; 4];
        assert!(Tx::from_serialized(&transaction, Network::Mainnet).is_err());
    }

    #[test]
    fn deserialize_id() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION);

        let tx = Tx::from_serialized(&transaction, Network::Mainnet);
        assert_eq!(
            tx.unwrap().id(),
            "ee51510d7bbabe28052038d1deb10c03ec74f06a79e21913c6fcf48d56217c87"
        );
    }

    #[test]
    fn deserialize_version() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION);

        let tx = Tx::from_serialized(&transaction, Network::Mainnet);
        assert_eq!(tx.unwrap().version, 1);
    }

    #[test]
    fn deserialize_tx_ins() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION);

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.inputs.len(), 4);
    }

    #[test]
    fn deserialize_tx_outs() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION);

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.outputs[0].amount, 1000273);
        assert_eq!(tx.outputs[1].amount, 40000000);
    }

    #[test]
    fn deserialize_tx_outs_amount1() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION);

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.outputs.len(), 2);
    }

    #[test]
    fn deserialize_locktime() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION);

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.locktime, 410438);
    }

    #[test]
    fn deserialize_and_serialize() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION);

        let tx = Tx::from_serialized(&transaction, Network::Mainnet).unwrap();
        let tx_serialized = tx.serialize();

        assert_eq!(transaction, tx_serialized);
    }

    #[test]
    fn deserialize_and_get_fee() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION);

        let tx = Tx::from_serialized(&transaction, Network::Mainnet);
        let mut transaction = tx.unwrap();

        transaction.retrive_input_amount();

        assert_eq!(transaction.fee(), 140500);
    }
}
