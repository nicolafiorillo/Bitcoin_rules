use crate::{
    chain::tx::get_transaction,
    scripting::{context::Context, script_lang::ScriptLang},
};

use super::{tx::Tx, tx_error::TxError};

fn verify_input(tx: &Tx, input_index: usize) -> Result<bool, TxError> {
    if tx.input_len() <= input_index {
        return Err(TxError::InputIndexOutOfBounds);
    }

    let input_transaction = tx.get_input(input_index)?;
    let previous_transaction = match get_transaction(&input_transaction.previous_transaction_id, tx.network) {
        Ok(tx) => tx,
        Err(_e) => return Err(TxError::TransactionNotFoundInChain),
    };

    let script_sig = &input_transaction.script_sig.script_lang;

    let output_index = input_transaction.previous_transaction_index as usize;
    if previous_transaction.output_len() <= output_index {
        return Err(TxError::OutputIndexOutOfBounds);
    }

    let output_transaction = previous_transaction.get_output(output_index)?;

    let script_pub_key = &output_transaction.script_pub_key.script_lang;

    let z = tx.hash_signature(input_index, output_transaction.script_pub_key.clone());
    let complete_script = ScriptLang::combine(script_sig.clone(), script_pub_key.clone());

    let mut context = Context::new(complete_script.tokens(), z);

    match complete_script.evaluate(&mut context) {
        Err(e) => {
            log::debug!("Script error: {:?}", e);
            Err(TxError::ScriptError)
        }
        Ok(val) => Ok(val),
    }
}

/*
    Validate a transactions.

    TODO
    Not all validations are implemented yet.
    When all validations are implemented, this function will be refactored.
*/
pub fn validate(tx: &Tx) -> Result<bool, TxError> {
    // 1. The input of the transaction are previously unspent, to avoid double-spending
    // TODO - waiting for loading chain and collect UTXO transactions

    // 2. The sum of the inputs is greater then or equal to the sum of the outputs. No new bitcoins must be created.
    //    The difference between the sum of the inputs and the sum of the outputs goes is the transaction fee for the miner.
    let fee = fee(tx)?;
    log::debug!("Tx fee: {:} ({:})", fee, tx.id());

    if fee < 0 {
        return Err(TxError::InvalidTransactionFee);
    }

    // 3. The ScriptSig in the input successfully unlocks the previous ScriptPubKey of the outputs.
    for i in 0..tx.input_len() {
        if !verify_input(tx, i)? {
            return Err(TxError::ScriptVerificationFailed);
        }
    }

    // Other validations: https://developer.bitcoin.org/devguide/transactions.html#non-standard-transactions
    // https://en.bitcoin.it/wiki/Protocol_rules#.22tx.22_messages
    //
    // 4. The transaction must be finalized: either its locktime must be in the past (or less than or equal to the current block height),
    // or all of its sequence numbers must be 0xffffffff.

    // 5. The transaction must be smaller than 100,000 bytes. That’s around 200 times larger than a typical single-input,
    // single-output P2PKH transaction.

    // 6. Each of the transaction’s signature scripts must be smaller than 1,650 bytes.
    // That’s large enough to allow 15-of-15 multisig transactions in P2SH using compressed public keys.

    // 7. Bare (non-P2SH) multisig transactions which require more than 3 public keys are currently non-standard.

    // 8. The transaction’s signature script must only push data to the script evaluation stack.
    // It cannot push new opcodes, with the exception of opcodes which solely push data to the stack.

    // 9. The transaction must not include any outputs which receive fewer than 1/3 as many satoshis as it would take
    // to spend it in a typical input. That’s currently 546 satoshis for a P2PKH or P2SH output on a Bitcoin Core node
    // with the default relay fee. Exception: standard null data outputs must receive zero satoshis.

    // 10. Max sigops: https://github.com/bitcoin/bitcoin/blob/d2b8c5e1234cdaff84bd1f60aea598d219cdac5e/src/policy/policy.h#L33

    Ok(true)
}

pub fn fee(tx: &Tx) -> Result<i128, TxError> {
    let mut input_amount: i128 = 0;

    for i in 0..tx.input_len() {
        let input_transaction = tx.get_input(i)?;
        let previous_transaction = match get_transaction(&input_transaction.previous_transaction_id, tx.network) {
            Ok(tx) => tx,
            Err(_e) => return Err(TxError::TransactionNotFoundInChain),
        };

        let output_index = input_transaction.previous_transaction_index as usize;
        if previous_transaction.output_len() <= output_index {
            return Err(TxError::OutputIndexOutOfBounds);
        }

        let output_transaction = previous_transaction.get_output(output_index)?;
        input_amount += output_transaction.amount as i128;
    }

    let mut output_amount: i128 = 0;

    for i in 0..tx.output_len() {
        let output_transaction = tx.get_output(i)?;
        output_amount += output_transaction.amount as i128;
    }

    Ok(input_amount - output_amount)
}

#[cfg(test)]
mod verification_test {
    use rug::Integer;

    use crate::{
        chain::tx::get_transaction,
        flags::network::Network,
        std_lib::integer_extended::IntegerExtended,
        transaction::{tx_error::TxError, verification::verify_input},
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
        let satoshi_transaction_id: Integer =
            Integer::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let satoshi_transaction = get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

        let res = verify_input(satoshi_transaction, 1);
        assert_eq!(TxError::InputIndexOutOfBounds, res.expect_err("Err"));
    }

    #[test]
    fn verify_first_transaction_ever() {
        let satoshi_transaction_id: Integer =
            Integer::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let satoshi_transaction = get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

        let res = validate(satoshi_transaction).unwrap();
        assert!(res);
    }
}
