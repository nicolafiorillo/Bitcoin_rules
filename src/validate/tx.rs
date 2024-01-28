use rug::Integer;

use crate::{
    chain::transaction::get_transaction,
    scripting::{
        context::Context,
        script_lang::ScriptLang,
        standard::{standard_type, StandardType},
    },
    std_lib::std_result::StdResult,
    transaction::{tx::Tx, tx_out::TxOut},
};

const MIN_COINBASE_LENGTH: usize = 2;
const MAX_COINBASE_LENGTH: usize = 100;

#[derive(Debug, Clone)]
pub struct Output {
    pub standard: StandardType,
    pub data: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub valid: bool,
    pub fee: i128,
    pub outputs: Vec<Output>,
}

fn verify_input(tx: &Tx, input_index: usize) -> StdResult<bool> {
    if tx.input_len() <= input_index {
        Err("input_index_out_of_bounds")?;
    }

    let input_transaction = tx.input(input_index)?;
    let previous_transaction = match get_transaction(&input_transaction.previous_transaction_id, tx.network) {
        Ok(tx) => tx,
        Err(_e) => Err("transaction_not_found_in_chain")?,
    };

    let script_sig = &input_transaction.script_sig.script_lang;

    let output_index = input_transaction.previous_transaction_index as usize;
    if previous_transaction.output_len() <= output_index {
        Err("output_index_out_of_bounds")?;
    }

    let output_transaction = previous_transaction.output(output_index)?;

    let script_pub_key = &output_transaction.script_pub_key.script_lang;

    let z = tx.hash_signature(input_index, output_transaction.script_pub_key.clone());
    let complete_script = ScriptLang::combine(script_sig.clone(), script_pub_key.clone());

    let mut context = Context::new(complete_script.tokens(), z);

    match complete_script.evaluate(&mut context) {
        Err(e) => {
            log::debug!("Script error: {:?}", e);
            Err("script_error")?
        }
        Ok(val) => Ok(val),
    }
}

pub fn fee(tx: &Tx) -> StdResult<i128> {
    let mut input_amount: i128 = 0;

    for i in 0..tx.input_len() {
        let input_transaction = tx.input(i)?;
        let previous_transaction = match get_transaction(&input_transaction.previous_transaction_id, tx.network) {
            Ok(tx) => tx,
            Err(_e) => Err("transaction_not_found_in_chain")?,
        };

        let output_index = input_transaction.previous_transaction_index as usize;
        if previous_transaction.output_len() <= output_index {
            Err("output_index_out_of_bounds")?;
        }

        let output_transaction = previous_transaction.output(output_index)?;
        input_amount += output_transaction.amount as i128;
    }

    let mut output_amount: i128 = 0;

    for i in 0..tx.output_len() {
        let output_transaction = tx.output(i)?;
        output_amount += output_transaction.amount as i128;
    }

    Ok(input_amount - output_amount)
}

fn analyze_output(output: &TxOut) -> Output {
    let script_pub_key = output.script_pub_key.clone();
    let tokens = script_pub_key.script_lang.tokens();

    let mut context = Context::new(tokens, Integer::from(0));
    let _res = script_pub_key.script_lang.evaluate(&mut context);

    let standard: StandardType = standard_type(&script_pub_key.script_lang);

    Output {
        standard,
        data: context.data().clone(),
    }
}

pub fn verify_coinbase(tx: &Tx) -> bool {
    let scripsig = tx.coinbase_scripsig();
    scripsig.len() >= MIN_COINBASE_LENGTH && scripsig.len() <= MAX_COINBASE_LENGTH
}

/*
    Analyze and validate a transactions.

    TODO
    Not all validations are implemented yet.
    When all validations are implemented, this function will be refactored.
*/
pub fn analyze(tx: &Tx) -> StdResult<AnalysisResult> {
    let mut tx_fee: i128 = 0;

    if tx.is_coinbase() {
        if !verify_coinbase(tx) {
            Err("coinbase_verification_failed")?;
        }
    } else {
        // * The input of the transaction are previously unspent, to avoid double-spending
        // TODO - waiting for loading chain and collect UTXO transactions

        // * The sum of the inputs is greater then or equal to the sum of the outputs. No new bitcoins must be created.
        // The difference between the sum of the inputs and the sum of the outputs goes is the transaction fee for the miner.
        tx_fee = fee(tx)?;
        log::debug!("Tx fee: {:} ({:})", tx_fee, tx.id());

        if tx_fee < 0 {
            Err("invalid_transaction_fee")?;
        }

        // * The ScriptSig in the input successfully unlocks the previous ScriptPubKey of the outputs.
        for i in 0..tx.input_len() {
            if !verify_input(tx, i)? {
                Err("script_verification_failed")?;
            }
        }
    }

    // * Some analisys on outputs:
    let mut outputs: Vec<Output> = Vec::new();

    for index in 0..tx.output_len() {
        let output = tx.outputs(index);
        let res = analyze_output(output);
        outputs.push(res);
    }

    /*
        Other validations: https://developer.bitcoin.org/devguide/transactions.html#non-standard-transactions
        https://en.bitcoin.it/wiki/Protocol_rules#.22tx.22_messages

        * The transaction must be finalized: either its locktime must be in the past (or less than or equal to the current block height),
        or all of its sequence numbers must be 0xffffffff.

        * The transaction must be smaller than 100,000 bytes. That’s around 200 times larger than a typical single-input,
        single-output P2PKH transaction.

        * Each of the transaction’s signature scripts must be smaller than 1,650 bytes.
        That’s large enough to allow 15-of-15 multisig transactions in P2SH using compressed public keys.

        * Bare (non-P2SH) multisig transactions which require more than 3 public keys are currently non-standard.

        * The transaction’s signature script must only push data to the script evaluation stack.
        It cannot push new opcodes, with the exception of opcodes which solely push data to the stack.

        * The transaction must not include any outputs which receive fewer than 1/3 as many satoshis as it would take
        to spend it in a typical input. That’s currently 546 satoshis for a P2PKH or P2SH output on a Bitcoin Core node
        with the default relay fee. Exception: standard null data outputs must receive zero satoshis.

        * A transaction can only include one NULL DATA locking script for it to be considered a standard transaction (meaning that it will be relayed by nodes).
        This is because a transaction with multiple NULL DATA outputs is considered a non-standard transaction, and will not be relayed by most nodes.

        * Transaction must not contain more than one OP_RETURN output.

        * Max sigops: https://github.com/bitcoin/bitcoin/blob/d2b8c5e1234cdaff84bd1f60aea598d219cdac5e/src/policy/policy.h#L33
    */

    Ok(AnalysisResult {
        valid: true,
        fee: tx_fee,
        outputs,
    })
}

#[cfg(test)]
mod verification_test {
    use rug::Integer;

    use crate::{
        chain::transaction::get_transaction, flags::network::Network, scripting::token::Token,
        std_lib::integer_extended::IntegerExtended,
    };

    use super::*;

    #[test]
    fn verify_input_of_first_transaction_ever() {
        let satoshi_transaction_id: Integer =
            Integer::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let satoshi_transaction = get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

        let res = verify_input(satoshi_transaction, 0).unwrap();
        assert!(res);
    }

    #[test]
    fn verify_transaction_invalid_input_index() {
        let transaction_id: Integer =
            Integer::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let transaction = get_transaction(&transaction_id, Network::Mainnet).unwrap();

        let res = verify_input(transaction, 1);
        assert_eq!("input_index_out_of_bounds", res.expect_err("Err").to_string());
    }

    #[test]
    fn verify_first_transaction_ever() {
        let transaction_id: Integer =
            Integer::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let transaction = get_transaction(&transaction_id, Network::Mainnet).unwrap();

        let res = analyze(transaction).unwrap();

        assert!(res.valid);
        assert_eq!(res.fee, 0);

        assert_eq!(res.outputs.len(), 2);
        assert_eq!(res.outputs[0].standard, StandardType::P2pk);
        assert_eq!(res.outputs[1].standard, StandardType::P2pk);
    }

    #[test]
    fn verify_transaction_with_return_data() {
        let transaction_id: Integer =
            Integer::from_hex_str("98ca9c4cae0b444c31c73b3fc0b6c6f897c1667ebd521a046ca4c3ade3e36153");
        let transaction = get_transaction(&transaction_id, Network::Testnet).unwrap();

        let res = analyze(transaction).unwrap();

        assert!(res.valid);
        assert_eq!(res.fee, 661);
        assert_eq!(res.outputs.len(), 2);

        assert_eq!(res.outputs[0].standard, StandardType::Data);
        let data = res.outputs[0].clone().data.unwrap();
        assert_eq!("Hello Bitcoin_rules!", String::from_utf8(data).unwrap());

        assert_eq!(res.outputs[1].standard, StandardType::P2pkh);
    }

    #[test]
    fn verify_transaction_p2pkh_type_script() {
        let transaction_id: Integer =
            Integer::from_hex_str("c843441a5e6d6a3b47a686cafa862951d649fea242f016d486dc20d74fa9f61c");
        let transaction = get_transaction(&transaction_id, Network::Testnet).unwrap();

        let res = analyze(transaction).unwrap();

        assert!(res.valid);
        assert_eq!(res.fee, 339);
        assert_eq!(res.outputs.len(), 1);

        assert_eq!(res.outputs[0].standard, StandardType::P2pkh);
    }

    #[test]
    fn verify_transaction_checking_p2ms_type_script() {
        let transaction_id: Integer =
            Integer::from_hex_str("23b397edccd3740a74adb603c9756370fafcde9bcc4483eb271ecad09a94dd63");
        let transaction = get_transaction(&transaction_id, Network::Mainnet).unwrap();

        let res = analyze(transaction).unwrap();

        assert!(res.valid);
        assert_eq!(res.fee, 0);
        assert_eq!(res.outputs.len(), 1);

        assert_eq!(res.outputs[0].standard, StandardType::P2pkh);
    }

    #[test]
    fn verify_transaction_is_coinbase() {
        let transaction_id: Integer =
            Integer::from_hex_str("0437cd7f8525ceed2324359c2d0ba26006d92d856a9c20fa0241106ee5a597c9");
        let transaction = get_transaction(&transaction_id, Network::Mainnet).unwrap();

        assert!(transaction.is_coinbase());
    }

    #[test]
    fn verify_transaction_is_not_coinbase() {
        let transaction_id: Integer =
            Integer::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let transaction = get_transaction(&transaction_id, Network::Mainnet).unwrap();

        assert!(!transaction.is_coinbase());
    }

    #[test]
    fn verify_first_coinbase_transaction() {
        let transaction_id: Integer =
            Integer::from_hex_str("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b");
        let transaction = get_transaction(&transaction_id, Network::Mainnet).unwrap();

        assert!(transaction.is_coinbase());
        assert!(analyze(&transaction).is_ok());

        let satoshi = &transaction.input(0).unwrap().script_sig.script_lang.tokens()[2];
        let Token::Element(bytes) = satoshi else { todo!() };

        assert_eq!(
            "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks",
            std::str::from_utf8(bytes).unwrap()
        );
    }
}
