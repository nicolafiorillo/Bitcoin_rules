/*
   Criticisms of Bitcoin’s raw txn format: https://web.archive.org/web/20140304092315/https://exiledbear.wordpress.com/2013/06/06/criticisms-of-bitcoins-raw-txn-format/
*/
use rug::{integer::Order, Integer};
use std::fmt::{Display, Formatter};

use crate::{
    flags::{network::Network, sighash::SigHash},
    hashing::hash256::hash256,
    std_lib::{std_result::StdResult, varint::encode},
};

use super::{
    script::Script,
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
    pub fn new(network: Network) -> Tx {
        let inputs = TxIns::new(Vec::new());
        let outputs = TxOuts::new(Vec::new());

        Tx {
            version: 1,
            inputs,
            outputs,
            locktime: 0,
            network,
        }
    }

    pub fn add_input(&mut self, i: TxIn) {
        self.inputs.push(i);
    }

    pub fn add_output(&mut self, o: TxOut) {
        self.outputs.push(o);
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

    pub fn input(&self, index: usize) -> StdResult<&TxIn> {
        if self.inputs.len() <= index {
            Err("input_index_out_of_bounds")?;
        }

        Ok(&self.inputs[index])
    }

    pub fn output(&self, index: usize) -> StdResult<&TxOut> {
        if self.outputs.len() <= index {
            Err("input_index_out_of_bounds")?;
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
    pub fn deserialize(serialized: &[u8], network: Network) -> StdResult<Self> {
        if serialized.len() < 5 {
            Err("invalid_transaction_length")?;
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
            let (tx_in, c) = TxIn::deserialize(serialized, cursor, network)?;
            cursor = c;

            txs_in.push(tx_in);
        }

        // Outputs
        let tx_out_count = varint_decode(serialized, cursor)?;
        cursor += tx_out_count.length;

        let mut txs_out: Vec<TxOut> = vec![];

        for _ in 0..tx_out_count.value {
            let (tx_out, c) = TxOut::deserialize(serialized, cursor)?;
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
            Err("partially_read_transaction")?;
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
        let inputs_length = encode(self.inputs.len() as u64);
        let inputs_serialized: Vec<u8> = self.inputs.serialize();
        let outputs_length = encode(self.outputs.len() as u64);
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
       This signature, if correct, "unlocks" via OP_CHECKSIG (or related ones) the ScriptPubKey of the output that input i is pointing to.
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

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && self.inputs[0].is_coinbase()
    }

    pub fn coinbase_scripsig(&self) -> &[u8] {
        if !self.is_coinbase() {
            panic!("not a coinbase transaction");
        }

        &self.input(0).unwrap().script_sig.raw
    }

    pub fn coinbase_height(&self) -> u64 {
        // Coinbase heigth is the heigth of the block this transaction is included in.
        // It is encoded in the coinbase transaction as the first element of the coinbase scriptSig.
        // This is applicalbe when block version is equal or greater than 2.

        // TODO: verify that this is applicable matching block version.

        if !self.is_coinbase() {
            panic!("not a coinbase transaction");
        }

        let scripsig = self.coinbase_scripsig();

        match varint_decode(scripsig, 0) {
            Ok(h) => h.value,
            Err(_) => 0,
        }
    }
}

#[cfg(test)]
mod tx_test {

    use rug::Integer;

    use crate::{
        chain,
        std_lib::{integer_extended::IntegerExtended, vector, vector::string_to_bytes},
        transaction::{script::Script, signing, tx::Tx, tx_in::TxIn, tx_out::TxOut},
        validate::tx::{analyze, fee},
        {flags::network::Network, keys::key::Key, scripting::standard},
    };

    pub const SERIALIZED_TRANSACTION: &str = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600";

    #[test]
    fn invalid_transaction_length() {
        let transaction: Vec<u8> = vec![0; 4];
        assert!(Tx::deserialize(&transaction, Network::Mainnet).is_err());
    }

    #[test]
    fn deserialize_id() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::deserialize(&transaction, Network::Mainnet);
        assert_eq!(
            tx.unwrap().id(),
            "EE51510D7BBABE28052038D1DEB10C03EC74F06A79E21913C6FCF48D56217C87"
        );
    }

    #[test]
    fn deserialize_version() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::deserialize(&transaction, Network::Mainnet);
        assert_eq!(tx.unwrap().version, 1);
    }

    #[test]
    fn deserialize_tx_ins() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::deserialize(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.inputs.len(), 4);
    }

    #[test]
    fn deserialize_tx_outs() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::deserialize(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.outputs[0].amount, 1000273);
        assert_eq!(tx.outputs[1].amount, 40000000);
    }

    #[test]
    fn deserialize_tx_outs_amount1() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::deserialize(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.outputs.len(), 2);
    }

    #[test]
    fn deserialize_locktime() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::deserialize(&transaction, Network::Mainnet).unwrap();

        assert_eq!(tx.locktime, 410438);
    }

    #[test]
    fn deserialize_and_serialize() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::deserialize(&transaction, Network::Mainnet).unwrap();
        let tx_serialized = tx.serialize();

        assert_eq!(transaction, tx_serialized);
    }

    #[test]
    fn deserialize_and_get_fee() {
        let transaction: Vec<u8> = string_to_bytes(SERIALIZED_TRANSACTION).unwrap();

        let tx = Tx::deserialize(&transaction, Network::Mainnet);
        let transaction = tx.unwrap();

        assert_eq!(fee(&transaction).unwrap(), 140500);
    }

    #[test]
    fn new_p2pkh_transaction_one_input_two_outputs_1() {
        let network = Network::Testnet;

        let previous_transaction_id =
            Integer::from_hex_str("d896ef1f6c32fc3857b0116cab5067c862a3dc81f295e923e4b22be69115c849");
        let previous_tx_index: usize = 0;
        let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();
        let output_transaction = previous_transaction.output(previous_tx_index).unwrap();
        let script_pub_key = output_transaction.script_pub_key.clone();

        let private_key =
            Integer::from_dec_str("275665454735547573090156431398001801704654402004664009535475985449755313");

        let mut tx = Tx::new(network);

        let tx_in = TxIn::new_with_previous_transaction(
            "d896ef1f6c32fc3857b0116cab5067c862a3dc81f295e923e4b22be69115c849",
            previous_tx_index as u32,
            network,
        );

        tx.add_input(tx_in);

        let address1 = Key::address_to_hash160("mty46U8fGsqxj7zaukWSJ2yzBZreuJoTRh", network).unwrap();
        let script1 = standard::p2pkh_script(&address1);
        let tx_out1 = TxOut::new(1000, Script::new_from_script_lang(&script1));

        let address2 = Key::address_to_hash160("n4AoVe3S9ovRxDmGkm5mbZz8zCpyzT4Q9N", network).unwrap();
        let script2 = standard::p2pkh_script(&address2);
        let tx_out2 = TxOut::new(4000, Script::new_from_script_lang(&script2));

        tx.add_output(tx_out1);
        tx.add_output(tx_out2);

        let script = signing::generate_input_signature(&tx, 0, &private_key, script_pub_key).unwrap();
        tx.substitute_script(0, script);

        let res = vector::bytes_to_string(&tx.serialize());

        assert_eq!(res, "010000000149C81591E62BB2E423E995F281DCA362C86750AB6C11B05738FC326C1FEF96D8000000006A473044022074494219882616A1922C3067C042F900451E01BF43C0258446B948D05D9DE6E002201F11ECF14A2EF846305BB0FAFB5D02C184A4C4FCABE584FE824CC807E7178A6501210280FD09653481B15ECD969BDB36B6454EC082913FBC4C6E360C0196C313395827FFFFFFFF02E8030000000000001976A91493894AC0A123F716291374F8BB414B3532EB872A88ACA00F0000000000001976A914F87B3A4B4F29D7E379DBCF1E9CADB95611F0439D88AC00000000");
    }

    #[test]
    fn new_p2pkh_transaction_one_input_two_outputs_2() {
        let network = Network::Testnet;

        let previous_transaction_id =
            Integer::from_hex_str("d896ef1f6c32fc3857b0116cab5067c862a3dc81f295e923e4b22be69115c849");
        let previous_tx_index: usize = 0;
        let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();
        let output_transaction = previous_transaction.output(previous_tx_index).unwrap();
        let script_pub_key = output_transaction.script_pub_key.clone();

        let private_key =
            Integer::from_dec_str("275665454735547573090156431398001801704654402004664009535475985449755313");

        let mut tx = Tx::new(network);

        let tx_in = TxIn::new_with_previous_transaction(
            "d896ef1f6c32fc3857b0116cab5067c862a3dc81f295e923e4b22be69115c849",
            previous_tx_index as u32,
            network,
        );

        tx.add_input(tx_in);

        let address1 = Key::address_to_hash160("mty46U8fGsqxj7zaukWSJ2yzBZreuJoTRh", network).unwrap();
        let script1 = standard::p2pkh_script(&address1);
        let tx_out1 = TxOut::new(1000, Script::new_from_script_lang(&script1));

        let address2 = Key::address_to_hash160("n4AoVe3S9ovRxDmGkm5mbZz8zCpyzT4Q9N", network).unwrap();
        let script2 = standard::p2pkh_script(&address2);
        let tx_out2 = TxOut::new(4000, Script::new_from_script_lang(&script2));

        tx.add_output(tx_out1);
        tx.add_output(tx_out2);

        let script = signing::generate_input_signature(&tx, 0, &private_key, script_pub_key).unwrap();
        tx.substitute_script(0, script);

        let serialized = vector::bytes_to_string(&tx.serialize());
        assert_eq!(serialized, "010000000149C81591E62BB2E423E995F281DCA362C86750AB6C11B05738FC326C1FEF96D8000000006A473044022074494219882616A1922C3067C042F900451E01BF43C0258446B948D05D9DE6E002201F11ECF14A2EF846305BB0FAFB5D02C184A4C4FCABE584FE824CC807E7178A6501210280FD09653481B15ECD969BDB36B6454EC082913FBC4C6E360C0196C313395827FFFFFFFF02E8030000000000001976A91493894AC0A123F716291374F8BB414B3532EB872A88ACA00F0000000000001976A914F87B3A4B4F29D7E379DBCF1E9CADB95611F0439D88AC00000000");

        assert!(analyze(&tx).is_ok());
    }

    #[test]
    fn new_p2pkh_transaction_two_inputs_one_output() {
        let network = Network::Testnet;

        let previous_transaction_id =
            Integer::from_hex_str("66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c");
        let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();

        let output_transaction_0 = previous_transaction.output(0).unwrap();
        let script_pub_key_0 = output_transaction_0.script_pub_key.clone();

        let output_transaction_1 = previous_transaction.output(1).unwrap();
        let script_pub_key_1 = output_transaction_1.script_pub_key.clone();

        let private_key_0 =
            Integer::from_dec_str("421788365705557317699661811707659433049257527084948635109995507081033905");
        let private_key_1 =
            Integer::from_dec_str("1029357723937880961141771287078141638280703752019825886047717249989513591");

        let mut tx = Tx::new(network);

        let tx_in_0 = TxIn::new_with_previous_transaction(
            "66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c",
            0,
            network,
        );

        let tx_in_1 = TxIn::new_with_previous_transaction(
            "66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c",
            1,
            network,
        );

        tx.add_input(tx_in_0);
        tx.add_input(tx_in_1);

        let address = Key::address_to_hash160("muekgXwwwbFJTVq1JTbi7Lrwi7fyWY8PEZ", network).unwrap();
        let script = standard::p2pkh_script(&address);
        let tx_out = TxOut::new(4661, Script::new_from_script_lang(&script));

        tx.add_output(tx_out);

        let script_0 = signing::generate_input_signature(&tx, 0, &private_key_0, script_pub_key_0).unwrap();
        tx.substitute_script(0, script_0);

        let script_1 = signing::generate_input_signature(&tx, 1, &private_key_1, script_pub_key_1).unwrap();
        tx.substitute_script(1, script_1);

        let serialized = vector::bytes_to_string(&tx.serialize());
        assert_eq!(serialized, "01000000028C7DF57C4727FFD58AF8961197A24BBC437A4EFC3CC2C05D7F1F652EC32E1466000000006A473044022065AB7F50AA5E4A2FF0B1FEE463F55E6D51A5D8427D1205B1FA281E8E873C8E3902202B5D0B5451BB1AAC17B7C007BEA84F18104BB7205395998F278E6651B0B23DB70121031620D8DD422DC901A3B62973F8E9C0E10087DEA8D29B676DB432431053F20A1CFFFFFFFF8C7DF57C4727FFD58AF8961197A24BBC437A4EFC3CC2C05D7F1F652EC32E1466010000006B483045022100CD7A262042988F765FC11EE2C111BDFBBA24C2DAEDB2BE4149546D0558528BDE0220539A56666D147DBE5491B31BB370B044AC873EA0627B12BBEFD7EA7EA10EA05B012103EF5EDA9D7D4898493D6E49F853504C57B05FD94C920A937202FFD28DEACE1F45FFFFFFFF0135120000000000001976A9149B0B65266E7938E4EB5148CD90F3479126EE76F888AC00000000");

        assert!(analyze(&tx).is_ok());
    }

    #[test]
    fn new_data_transaction_one_input_two_outputs() {
        let network = Network::Testnet;

        let previous_transaction_id =
            Integer::from_hex_str("c843441a5e6d6a3b47a686cafa862951d649fea242f016d486dc20d74fa9f61c");
        let previous_tx_index: usize = 0;
        let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();
        let output_transaction = previous_transaction.output(previous_tx_index).unwrap();
        let script_pub_key = output_transaction.script_pub_key.clone();

        let private_key =
            Integer::from_dec_str("1051734479676813432409430359100974956366171456908644401343499951103054813");

        let mut tx = Tx::new(network);

        let tx_in = TxIn::new_with_previous_transaction(
            "c843441a5e6d6a3b47a686cafa862951d649fea242f016d486dc20d74fa9f61c",
            previous_tx_index as u32,
            network,
        );

        tx.add_input(tx_in);

        let data = "Hello Bitcoin_rules!".as_bytes();
        let script1 = standard::data_script(data);
        let tx_out1 = TxOut::new(0, Script::new_from_script_lang(&script1));

        let address2 = Key::address_to_hash160("mqi7KkHZEHJAzoRvamrziHVLZPSVkxLmPZ", network).unwrap();
        let script2 = standard::p2pkh_script(&address2);
        let tx_out2 = TxOut::new(4000, Script::new_from_script_lang(&script2));

        tx.add_output(tx_out1);
        tx.add_output(tx_out2);

        let script = signing::generate_input_signature(&tx, 0, &private_key, script_pub_key).unwrap();
        tx.substitute_script(0, script);

        let res = vector::bytes_to_string(&tx.serialize());

        assert_eq!(res, "01000000011CF6A94FD720DC86D416F042A2FE49D6512986FACA86A6473B6A6D5E1A4443C80000000069463043021F4F5D3404C0E0E949F54150747FAE09C5D6D01482C9055AB2D51B4436336B8702201D679F8885A3E894DC68E1A85B1B84EB7AE445F5733CE4C512DDE34E18B1F06B012102B32BDD5A9F0DFF17AC7E92A920666081BDAD54356FCAB3CF7343963ABDB16195FFFFFFFF020000000000000000166A1448656C6C6F20426974636F696E5F72756C657321A00F0000000000001976A9146FCD5EE01651D668A50B5529ACF57FB0A28C948488AC00000000");

        assert!(analyze(&tx).is_ok());
    }

    #[test]
    fn new_p2pk_transaction_one_input_one_output() {
        let network = Network::Testnet;

        let previous_transaction_id =
            Integer::from_hex_str("b3da0e148a202833a4dca8a3f5d0cc336c26e0cd4e213b9cc8c6b6bce69423ce");
        let previous_tx_index: usize = 1;
        let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();
        let output_transaction = previous_transaction.output(previous_tx_index).unwrap();
        let script_pub_key = output_transaction.script_pub_key.clone();

        let private_key =
            Integer::from_dec_str("111462511462181760351805850201853994070745575482638197696888757036583703592764");
        let key = crate::keys::key::Key::new(private_key);

        let mut tx = Tx::new(network);

        let tx_in = TxIn::new_with_previous_transaction(
            "b3da0e148a202833a4dca8a3f5d0cc336c26e0cd4e213b9cc8c6b6bce69423ce",
            previous_tx_index as u32,
            network,
        );

        tx.add_input(tx_in);

        let script = standard::p2pk_script(&key.public_key_sec());
        let tx_out = TxOut::new(14483, Script::new_from_script_lang(&script));

        tx.add_output(tx_out);

        let script = signing::generate_input_signature(&tx, 0, &key.private_key(), script_pub_key).unwrap();
        tx.substitute_script(0, script);

        let res = vector::bytes_to_string(&tx.serialize());
        assert_eq!(res, "0100000001CE2394E6BCB6C6C89C3B214ECDE0266C33CCD0F5A3A8DCA43328208A140EDAB3010000006A473044022020867C3596C2CC01AB8E04EEF81B7B965E353BF95D9667AFF6A86DDA07AC8A0A022014C5EFF1C54E95084C660CE545D639EBB10DA670F8228C1E9963E30E683894190121030C4FE97C6E5397A7E7BE16D89D02627F248C5CB3ED1EFD1B517304EC844EFAFBFFFFFFFF0193380000000000002321030C4FE97C6E5397A7E7BE16D89D02627F248C5CB3ED1EFD1B517304EC844EFAFBAC00000000");

        assert!(analyze(&tx).is_ok());
    }
}
