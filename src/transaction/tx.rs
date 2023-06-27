use std::fmt::{Display, Formatter};

use rug::{integer::Order, Integer};

use crate::{
    btc_ecdsa::Network,
    hashing::hash256,
    varint::{varint_decode, VarInt},
};

use super::{script_pub_key::ScriptPubKey, script_sig::ScriptSig, tx_error::TxError, tx_in::TxIn, tx_out::TxOut};

#[derive(Debug)]
pub struct Tx {
    pub version: u32,
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
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
        Integer::from_digits(&serialized, Order::Msf)
    }

    fn u32_le_bytes(bytes: &[u8], from: usize) -> Result<u32, TxError> {
        if bytes.len() < (from + 4) {
            return Err(TxError::Invalid4BytesLength);
        }

        let mut v: [u8; 4] = [0; 4];
        v.copy_from_slice(&bytes[from..(from + 4)]);
        Ok(u32::from_le_bytes(v))
    }

    fn u64_le_bytes(bytes: &[u8], from: usize) -> Result<u64, TxError> {
        if bytes.len() < (from + 8) {
            return Err(TxError::Invalid8BytesLength);
        }

        let mut v: [u8; 8] = [0; 8];
        v.copy_from_slice(&bytes[from..(from + 8)]);
        Ok(u64::from_le_bytes(v))
    }

    fn varint_decode(bytes: &[u8], from: usize) -> Result<VarInt, TxError> {
        let vi = varint_decode(bytes, from);
        if let Err(_e) = vi {
            return Err(TxError::VarIntError);
        }

        Ok(vi.unwrap())
    }

    fn integer_le_32_bytes(bytes: &[u8], from: usize) -> Result<Integer, TxError> {
        if bytes.len() < (from + 32) {
            return Err(TxError::Invalid32BytesLength);
        }

        let slice = &bytes[from..(from + 32)];
        Ok(Integer::from_digits(slice, Order::Lsf))
    }

    // TODO: implement with stream
    pub fn from_serialized(serialized: &[u8], network: Network) -> Result<Self, TxError> {
        if serialized.len() < 5 {
            return Err(TxError::InvalidTransactionLength);
        }
        let mut cursor: usize = 0;

        // Version
        let version = Self::u32_le_bytes(serialized, cursor)?;
        cursor += 4;

        // Inputs
        let tx_in_count = Self::varint_decode(serialized, cursor)?;
        cursor += tx_in_count.length;

        let mut inputs: Vec<TxIn> = vec![];
        let mut input_index = 0;
        while input_index < tx_in_count.value {
            let tx_in_previous_transaction_id = Self::integer_le_32_bytes(serialized, cursor)?;
            cursor += 32;

            let tx_in_previous_transaction_index = Self::u32_le_bytes(serialized, cursor)?;
            cursor += 4;

            let tx_in_scriptsig_length = Self::varint_decode(serialized, cursor)?;
            cursor += tx_in_scriptsig_length.length;

            let tx_in_scriptsig_content_serialized =
                &serialized[cursor..cursor + tx_in_scriptsig_length.value as usize];
            let script_sig = ScriptSig::new(tx_in_scriptsig_content_serialized.to_vec());

            cursor += tx_in_scriptsig_length.value as usize;

            let tx_in_sequence = Self::u32_le_bytes(serialized, cursor)?;
            cursor += 4;

            let tx_in = TxIn::new(
                tx_in_previous_transaction_id,
                tx_in_previous_transaction_index,
                script_sig,
                tx_in_sequence,
            );

            inputs.push(tx_in);
            input_index += 1;
        }

        // Outputs
        let tx_out_count = Self::varint_decode(serialized, cursor)?;
        cursor += tx_out_count.length;

        let mut outputs: Vec<TxOut> = vec![];
        let mut output_index = 0;
        while output_index < tx_out_count.value {
            let amount = Self::u64_le_bytes(serialized, cursor)?;
            cursor += 8;

            let tx_out_scriptpubkey_length = Self::varint_decode(serialized, cursor)?;
            cursor += tx_out_scriptpubkey_length.length;

            let tx_out_scriptpubkey_content_serialized =
                &serialized[cursor..cursor + tx_out_scriptpubkey_length.value as usize];
            let script_pub_key = ScriptPubKey::new(tx_out_scriptpubkey_content_serialized.to_vec());

            cursor += tx_out_scriptpubkey_length.value as usize;

            let tx_out = TxOut::new(amount, script_pub_key);

            outputs.push(tx_out);
            output_index += 1;
        }

        // Locktime
        let locktime = Self::u32_le_bytes(serialized, cursor)?;
        cursor += 4;

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
        let inputs_serialized: Vec<u8> = self.inputs.iter().flat_map(|i| i.serialize()).collect();
        let outputs_serialized: Vec<u8> = self.outputs.iter().flat_map(|i| i.serialize()).collect();
        let locktime_serialized = self.locktime.to_le_bytes();
        let network_serialized = self.network as u8;

        [
            version_serialized.as_slice(),
            inputs_serialized.as_slice(),
            outputs_serialized.as_slice(),
            locktime_serialized.as_slice(),
            [network_serialized].as_slice(),
        ]
        .concat()
    }
}

#[cfg(test)]
mod tx_test {
    use crate::{btc_ecdsa::Network, helper::vector::string_to_bytes, transaction::tx::Tx};

    pub const SERIALIZED_TRANSACTION: &str = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600";

    #[test]
    fn invalid_transaction_length() {
        let transaction: Vec<u8> = vec![0; 4];
        assert!(Tx::from_serialized(&transaction, Network::Mainnet).is_err());
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
}
