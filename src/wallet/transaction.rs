use rug::Integer;

use crate::{
    flags::network::Network,
    scripting::standard,
    std_lib::integer_extended::IntegerExtended,
    transaction::{script::Script, tx::Tx, tx_error::TxError, tx_in::TxIn, tx_out::TxOut},
};

pub fn new_tx_in(previous_tx_id_str: &str, previous_tx_index: u32, network: Network) -> TxIn {
    // TODO previous_tx_index to usize
    let previous_tx_id = Integer::from_hex_str(previous_tx_id_str);
    TxIn::new_with_empty_script(previous_tx_id, previous_tx_index, network)
}

pub fn new_out(amount: u64, address: &str, network: Network) -> Result<TxOut, TxError> {
    // TODO: ugly crate::keys::key::Key::...
    let address = crate::keys::key::Key::address_to_hash160(address, network);

    let script = standard::p2pkh_script(address);
    let tx_out = TxOut::new(amount, Script::new_from_script_lang(&script));

    Ok(tx_out)
}

pub fn new_tx(tx_ins: &[TxIn], tx_outs: &[TxOut], network: Network) -> Tx {
    Tx::new(tx_ins, tx_outs, network)
}

#[cfg(test)]
mod transaction_test {
    use rug::Integer;

    use crate::{
        chain,
        flags::network::Network,
        std_lib::{integer_extended::IntegerExtended, vector},
        transaction::{signing, verification},
        wallet,
    };

    #[test]
    fn create_a_new_transaction() {
        let network = Network::Testnet;

        let previous_transaction_id =
            Integer::from_hex_str("d896ef1f6c32fc3857b0116cab5067c862a3dc81f295e923e4b22be69115c849");
        let previous_tx_index: usize = 0;
        let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();
        let output_transaction = previous_transaction.get_output(previous_tx_index).unwrap();
        let script_pub_key = output_transaction.script_pub_key.clone();

        let private_key =
            Integer::from_dec_str("275665454735547573090156431398001801704654402004664009535475985449755313");

        let tx_in = wallet::transaction::new_tx_in(
            "d896ef1f6c32fc3857b0116cab5067c862a3dc81f295e923e4b22be69115c849",
            previous_tx_index as u32,
            network,
        );

        let tx_out1 = wallet::transaction::new_out(1000, "mty46U8fGsqxj7zaukWSJ2yzBZreuJoTRh", network).unwrap();
        let tx_out2 = wallet::transaction::new_out(4000, "n4AoVe3S9ovRxDmGkm5mbZz8zCpyzT4Q9N", network).unwrap();

        let mut tx = wallet::transaction::new_tx(&[tx_in], &[tx_out1, tx_out2], network);

        let script = signing::generate_input_signature(&tx, previous_tx_index, &private_key, script_pub_key).unwrap();
        tx.substitute_script(previous_tx_index, script);

        let res = vector::vect_to_hex_string(&tx.serialize());

        assert_eq!(res, "010000000149C81591E62BB2E423E995F281DCA362C86750AB6C11B05738FC326C1FEF96D8000000006A473044022074494219882616A1922C3067C042F900451E01BF43C0258446B948D05D9DE6E002201F11ECF14A2EF846305BB0FAFB5D02C184A4C4FCABE584FE824CC807E7178A6501210280FD09653481B15ECD969BDB36B6454EC082913FBC4C6E360C0196C313395827FFFFFFFF02E8030000000000001976A91493894AC0A123F716291374F8BB414B3532EB872A88ACA00F0000000000001976A914F87B3A4B4F29D7E379DBCF1E9CADB95611F0439D88AC00000000");
    }

    #[test]
    fn a_new_transaction_one_input_two_outputs() {
        let network = Network::Testnet;

        let previous_transaction_id =
            Integer::from_hex_str("d896ef1f6c32fc3857b0116cab5067c862a3dc81f295e923e4b22be69115c849");
        let previous_tx_index: usize = 0;
        let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();
        let output_transaction = previous_transaction.get_output(previous_tx_index).unwrap();
        let script_pub_key = output_transaction.script_pub_key.clone();

        let private_key =
            Integer::from_dec_str("275665454735547573090156431398001801704654402004664009535475985449755313");

        let tx_in = wallet::transaction::new_tx_in(
            "d896ef1f6c32fc3857b0116cab5067c862a3dc81f295e923e4b22be69115c849",
            previous_tx_index as u32,
            network,
        );

        let tx_out1 = wallet::transaction::new_out(1000, "mty46U8fGsqxj7zaukWSJ2yzBZreuJoTRh", network).unwrap();
        let tx_out2 = wallet::transaction::new_out(4000, "n4AoVe3S9ovRxDmGkm5mbZz8zCpyzT4Q9N", network).unwrap();

        let mut tx = wallet::transaction::new_tx(&[tx_in], &[tx_out1, tx_out2], network);

        let script = signing::generate_input_signature(&tx, previous_tx_index, &private_key, script_pub_key).unwrap();
        tx.substitute_script(previous_tx_index, script);

        let serialized = vector::vect_to_hex_string(&tx.serialize());
        assert_eq!(serialized, "010000000149C81591E62BB2E423E995F281DCA362C86750AB6C11B05738FC326C1FEF96D8000000006A473044022074494219882616A1922C3067C042F900451E01BF43C0258446B948D05D9DE6E002201F11ECF14A2EF846305BB0FAFB5D02C184A4C4FCABE584FE824CC807E7178A6501210280FD09653481B15ECD969BDB36B6454EC082913FBC4C6E360C0196C313395827FFFFFFFF02E8030000000000001976A91493894AC0A123F716291374F8BB414B3532EB872A88ACA00F0000000000001976A914F87B3A4B4F29D7E379DBCF1E9CADB95611F0439D88AC00000000");

        assert!(verification::validate(&tx).is_ok());
    }

    #[test]
    fn a_new_transaction_two_inputs_one_output() {
        let network = Network::Testnet;

        let previous_transaction_id =
            Integer::from_hex_str("66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c");
        let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();

        let output_transaction_0 = previous_transaction.get_output(0).unwrap();
        let script_pub_key_0 = output_transaction_0.script_pub_key.clone();

        let output_transaction_1 = previous_transaction.get_output(1).unwrap();
        let script_pub_key_1 = output_transaction_1.script_pub_key.clone();

        let private_key_0 =
            Integer::from_dec_str("421788365705557317699661811707659433049257527084948635109995507081033905");
        let private_key_1 =
            Integer::from_dec_str("1029357723937880961141771287078141638280703752019825886047717249989513591");

        let tx_in_0 = wallet::transaction::new_tx_in(
            "66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c",
            0,
            network,
        );

        let tx_in_1 = wallet::transaction::new_tx_in(
            "66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c",
            1,
            network,
        );

        let tx_out = wallet::transaction::new_out(4661, "muekgXwwwbFJTVq1JTbi7Lrwi7fyWY8PEZ", network).unwrap();

        let mut tx = wallet::transaction::new_tx(&[tx_in_0.clone(), tx_in_1.clone()], &[tx_out], network);

        let script_0 = signing::generate_input_signature(&tx, 0, &private_key_0, script_pub_key_0).unwrap();
        tx.substitute_script(0, script_0);

        let script_1 = signing::generate_input_signature(&tx, 1, &private_key_1, script_pub_key_1).unwrap();
        tx.substitute_script(1, script_1);

        let serialized = vector::vect_to_hex_string(&tx.serialize());
        assert_eq!(serialized, "01000000028C7DF57C4727FFD58AF8961197A24BBC437A4EFC3CC2C05D7F1F652EC32E1466000000006A473044022065AB7F50AA5E4A2FF0B1FEE463F55E6D51A5D8427D1205B1FA281E8E873C8E3902202B5D0B5451BB1AAC17B7C007BEA84F18104BB7205395998F278E6651B0B23DB70121031620D8DD422DC901A3B62973F8E9C0E10087DEA8D29B676DB432431053F20A1CFFFFFFFF8C7DF57C4727FFD58AF8961197A24BBC437A4EFC3CC2C05D7F1F652EC32E1466010000006B483045022100CD7A262042988F765FC11EE2C111BDFBBA24C2DAEDB2BE4149546D0558528BDE0220539A56666D147DBE5491B31BB370B044AC873EA0627B12BBEFD7EA7EA10EA05B012103EF5EDA9D7D4898493D6E49F853504C57B05FD94C920A937202FFD28DEACE1F45FFFFFFFF0135120000000000001976A9149B0B65266E7938E4EB5148CD90F3479126EE76F888AC00000000");

        assert!(verification::validate(&tx).is_ok());
    }
}
