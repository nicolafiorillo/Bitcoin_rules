use crate::scripting::script::Script;
use crate::{chain::tx::get_transaction, scripting::context::Context};

use super::{tx::Tx, tx_error::TxError};

pub fn verify_input(tx: &Tx, input_index: usize) -> Result<bool, TxError> {
    if tx.input_len() <= input_index {
        return Err(TxError::InputIndexOutOfBounds);
    }

    let input_transaction = tx.get_input(input_index)?;
    let previous_transaction = match get_transaction(&input_transaction.previous_transaction_id, tx.network) {
        Ok(tx) => tx,
        Err(_e) => return Err(TxError::TransactionNotFoundInChain),
    };

    let script_sig = match input_transaction.script_sig.script() {
        Ok(script) => script,
        Err(_e) => return Err(TxError::ScriptError),
    };

    let output_index = input_transaction.previous_transaction_index as usize;
    if previous_transaction.output_len() <= output_index {
        return Err(TxError::OutputIndexOutOfBounds);
    }

    let output_transaction = previous_transaction.get_output(output_index)?;

    let script_pub_key = match output_transaction.script_pub_key.script() {
        Ok(script) => script,
        Err(_e) => return Err(TxError::ScriptError),
    };

    let z = tx.hash_signature(input_index, output_transaction.script_pub_key.clone());
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
pub fn validate(tx: &Tx) -> Result<bool, TxError> {
    unimplemented!("Tx::validate");
}

#[cfg(test)]
mod verification_test {
    use rug::Integer;

    use crate::{
        chain::tx::get_transaction,
        flags::network::Network,
        std_lib::integer_ex::IntegerEx,
        transaction::{tx_error::TxError, verification::verify_input},
    };

    #[test]
    fn verify_first_transaction_ever() {
        let satoshi_transaction_id: Integer =
            IntegerEx::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let satoshi_transaction = get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

        let res = verify_input(satoshi_transaction, 0).unwrap();
        assert!(res);
    }

    #[test]
    fn verify_transaction_invalid_input_index() {
        let satoshi_transaction_id: Integer =
            IntegerEx::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
        let satoshi_transaction = get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

        let res = verify_input(satoshi_transaction, 1);
        assert_eq!(TxError::InputIndexOutOfBounds, res.expect_err("Err"));
    }
}
