/*
    TODO:
    All these function are used to validate
        1) old transactions (txs already in the chain)
        2) new transactions (txs in pool wating for validation)
    In the first case validation should be more flexible in order to accept old transactions already
    in chain but not compliant with the new rules, in the second case validation should be more strict in order
    to reject new transactions that are not compliant with the new rules.

    CURRENTLY WE ARE USING THE SAME VALIDATION RULES FOR BOTH CASES BUT, AT THE END OF THE DAY, WE SHOULD
    SPLIT VALIDATION IN TWO DIFFERENT VERIFICATION APPROACHES.
*/

use rug::Integer;

use crate::{
    ecdsa::point::Point,
    hashing::{hash160::hash160, hash256::hash256, ripemd160::ripemd160, sha1::sha1, sha256::sha256},
    keys::{key::Key, signature::Signature},
    std_lib::std_result::StdResult,
};

use super::{constants::MAX_RETURN_DATA_LENGTH, context::Context, token::*};

/*
   Ref: https://en.bitcoin.it/wiki/Script
*/
macro_rules! op_n {
    ($n:tt, $f:ident) => {
        pub fn $f(context: &mut Context) -> StdResult<bool> {
            context.stack_push(Token::Element(element_encode($n)));

            Ok(true)
        }
    };
}

// TODO: put function in code order
// TODO: split in more files, one for each category
// TODO: add documentation for each function

op_n!(2, op_2);
op_n!(3, op_3);
op_n!(4, op_4);
op_n!(5, op_5);
op_n!(6, op_6);
op_n!(7, op_7);
op_n!(8, op_8);
op_n!(9, op_9);
op_n!(10, op_10);
op_n!(11, op_11);
op_n!(12, op_12);
op_n!(13, op_13);
op_n!(14, op_14);
op_n!(15, op_15);
op_n!(16, op_16);

fn element_value_by_result(res: bool) -> Vec<u8> {
    if res {
        ELEMENT_ONE.to_vec()
    } else {
        ELEMENT_ZERO.to_vec()
    }
}

pub fn op_0(context: &mut Context) -> StdResult<bool> {
    context.stack_push(Token::Element(ELEMENT_ZERO.to_vec()));

    Ok(true)
}

pub fn op_1(context: &mut Context) -> StdResult<bool> {
    context.stack_push(Token::Element(element_encode(1)));

    Ok(true)
}

pub fn op_1negate(context: &mut Context) -> StdResult<bool> {
    context.stack_push(Token::Element(ELEMENT_ONE_NEGATE.to_vec()));

    Ok(true)
}

pub fn op_nop(_context: &mut Context) -> StdResult<bool> {
    Ok(true)
}

///
/// OP_RETURN
/// Script containing OP_RETURN is considered as a particular script and is used to add data to a transaction.
/// This always causes the script to fail.
/// For validation purposes the data after the OP_RETURN is ignored.
/// TODO: We will read the extra data in adifferent context (see
/// https://github.com/coinspark/python-OP_RETURN/blob/07e1dcd0d61b26b9b554ee0095cb4945f04a5ac7/OP_RETURN.py#L331
/// for structure: 0x6A + varint + data)
///
/// Examples:
/// https://blockchain.info/tx/d276abe15791941649c3ca8425d79167cc1cf801f83aa99753fe7f42740c0f23
/// https://blockchain.info/tx/728e24b2e7dd137e574c433a8db08ac2aa0bf0588ad7716e4c5a7da45dbb5933
/// https://blockchain.info/tx/52dd20f60d6e14e5a783e7668cf410efdea40cd9a92479b0f2423d0bc63575fa
///
pub fn op_return(context: &mut Context) -> StdResult<bool> {
    context.set_data(Vec::new());

    if !context.tokens_are_over() {
        let token = context.next_token();

        if let Token::Element(data) = token {
            let d = data.clone();

            if d.len() > MAX_RETURN_DATA_LENGTH {
                Err("return_data_too_long")?;
            }

            context.set_data(d);
        }
    }

    Err("exit_by_return")?
}

pub fn op_if(context: &mut Context) -> StdResult<bool> {
    let mut exec = false;

    if context.executing() {
        if !context.stack_has_enough_items(1) {
            Err("not_enough_items_in_stack")?;
        }

        let token = context.stack_pop_as_element()?;
        exec = token.as_bool();
    }

    context.set_execute(exec);

    Ok(true)
}

pub fn op_notif(context: &mut Context) -> StdResult<bool> {
    let mut exec = false;

    if context.executing() {
        if !context.stack_has_enough_items(1) {
            Err("not_enough_items_in_stack")?;
        }

        let token = context.stack_pop_as_element()?;
        exec = !token.as_bool();
    }

    context.set_execute(exec);

    Ok(true)
}

pub fn op_endif(context: &mut Context) -> StdResult<bool> {
    if !context.in_condition() {
        Err("unexpected_end_if")?;
    }

    context.unset_execute();
    Ok(true)
}

pub fn op_else(context: &mut Context) -> StdResult<bool> {
    if !context.in_condition() {
        Err("unexpected_else")?;
    }

    context.toggle_execute();
    Ok(true)
}

pub fn op_add(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(a), Token::Element(b)) = (elem1, elem2) {
        if a.len() > 4 || b.len() > 4 {
            Err("input_length_too_long")?;
        }

        let left = element_decode(a);
        let right = element_decode(b);

        let sum = left + right;
        context.stack_push(Token::Element(element_encode(sum)));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_sub(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let b = context.stack_pop_as_element()?;
    let a = context.stack_pop_as_element()?;

    if let (Token::Element(a), Token::Element(b)) = (a, b) {
        if a.len() > 4 || b.len() > 4 {
            Err("input_length_too_long")?;
        }

        let left = element_decode(a);
        let right = element_decode(b);

        let sub = left.checked_sub(right).ok_or("overflow")?;
        context.stack_push(Token::Element(element_encode(sub)));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_mul(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let a = context.stack_pop_as_element()?;
    let b = context.stack_pop_as_element()?;

    if let (Token::Element(a), Token::Element(b)) = (a, b) {
        let left = element_decode(a);
        let right = element_decode(b);

        let sum = left * right;
        context.stack_push(Token::Element(element_encode(sum)));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_equal(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let a = context.stack_pop_as_element()?;
    let b = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (a, b) {
        let equals = if left == right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.stack_push(Token::Element(equals.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

/*
   https://en.bitcoin.it/wiki/OP_CHECKSIG

   TODO: consider OP_CODESEPARATOR
*/
pub fn op_checksig(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let pub_key = context.stack_pop_as_element()?;
    let sig = context.stack_pop_as_element()?;

    if let Token::Element(public_key) = pub_key {
        if let Token::Element(mut der) = sig {
            // Removing last byte, that is the signature hash (SIGHASH): https://learn.saylor.org/mod/book/view.php?id=36341&chapterid=18919
            der.pop();

            let res = signature_is_valid(&der, &public_key, &context.z);

            let element_value = element_value_by_result(res);
            context.stack_push(Token::Element(element_value));
        }
    }

    Ok(true)
}

/*
   https://en.bitcoin.it/wiki/OP_CHECKMULTISIG

   OP_CHECKMULTISIG is a Bitcoin standard script transaction.
   It is request to provide N public keys that must match M signatures in ScriptPubKey to authorize a transaction.
   In other words, all provided signatures must match at least one of the public keys in the redeem script.

   The script is:
   OP_0 <sig1> [sig2] [sig3] ... [sigM] <M> <pubkey1> [pubkey2] [pubkey3] ... [pubkeyN] <N> OP_CHECKMULTISIG

   This script is rarely used directly but it is wrapped by a more user friendly script called P2SH (Pay To Script Hash).

   Oh, I was forgetting: Bitcoin_rules! now supports OP_CHECKMULTISIG transaction scripts.

*/
pub fn op_checkmultisig(context: &mut Context) -> StdResult<bool> {
    log::debug!("MS: Multisignature check start");

    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    /*
       n
    */
    let elem_n = context.stack_pop_as_element()?;
    let n = elem_n.as_number() as usize;

    if n == 0 {
        Err("expected_n_for_multisig")?;
    }

    if !context.stack_has_enough_items(n) {
        Err("not_enough_items_in_stack")?;
    }

    let mut sec_pub_keys = Vec::new();
    for _ in 0..n {
        let elem = context.stack_pop_as_element()?;
        let pub_key = elem.as_bytes();
        sec_pub_keys.push(pub_key);
    }

    log::debug!("MS: n: {}", n);

    /*
       m
    */
    let elem_m = context.stack_pop_as_element()?;
    let m = elem_m.as_number() as usize;

    if m == 0 {
        Err("expected_m_for_multisig")?;
    }

    if m > n {
        Err("m_greater_than_n_for_multisig")?;
    }

    if !context.stack_has_enough_items(m) {
        Err("not_enough_items_in_stack")?;
    }

    let mut der_signatures = Vec::new();
    for _ in 0..m {
        let elem = context.stack_pop_as_element()?;
        let mut der_signature = elem.as_bytes();
        // Removing last byte, that is the signature hash (SIGHASH): https://learn.saylor.org/mod/book/view.php?id=36341&chapterid=18919
        der_signature.pop();

        der_signatures.push(der_signature);
    }

    log::debug!("MS: m: {}", m);

    /*
       one-of-error
    */
    let token = context.stack_pop();
    if !token.is_zero_or_empy() {
        Err("expected_op0_for_multisig")?;
    }

    /*
       checking
    */
    let mut pk_index: usize = 0;
    let mut valid_signatures: usize = 0;

    for der in der_signatures {
        while pk_index < sec_pub_keys.len() {
            let public_key = &sec_pub_keys[pk_index];
            let res = signature_is_valid(&der, public_key, &context.z);

            if res {
                valid_signatures += 1;
                break;
            }

            pk_index += 1;
        }
    }

    log::debug!("MS: valid signature: {} (should match with m)", valid_signatures);

    // In other words, all provided signatures must match at least one of the public keys in the redeem script.
    let element_value = element_value_by_result(valid_signatures == m);
    context.stack_push(Token::Element(element_value));

    log::debug!("MS: Multisignature check end");

    Ok(true)
}

fn signature_is_valid(der: &[u8], public_key: &[u8], z: &Integer) -> bool {
    let signature = match Signature::new_from_der(der.to_vec()) {
        Ok(signature) => signature,
        Err(_) => {
            log::debug!("Invalid DER signature during OP_ checking");
            return false;
        }
    };

    let point = Point::deserialize(public_key.to_vec());
    Key::verify_signature(&point, z, &signature)
}

pub fn op_dup(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let op = context.top_stack();
    context.stack_push(op.clone());

    Ok(true)
}

pub fn op_2dup(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();

    context.stack_push(op2.clone());
    context.stack_push(op1.clone());
    context.stack_push(op2);
    context.stack_push(op1);

    Ok(true)
}

pub fn op_3dup(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(3) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();
    let op3 = context.stack_pop();

    context.stack_push(op3.clone());
    context.stack_push(op2.clone());
    context.stack_push(op1.clone());
    context.stack_push(op3);
    context.stack_push(op2);
    context.stack_push(op1);

    Ok(true)
}

pub fn op_2over(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(4) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();
    let op3 = context.stack_pop();
    let op4 = context.stack_pop();

    context.stack_push(op4.clone());
    context.stack_push(op3.clone());
    context.stack_push(op2);
    context.stack_push(op1);
    context.stack_push(op4);
    context.stack_push(op3);

    Ok(true)
}

pub fn op_swap(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();

    context.stack_push(op1);
    context.stack_push(op2);

    Ok(true)
}

pub fn op_2swap(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(4) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();
    let op3 = context.stack_pop();
    let op4 = context.stack_pop();

    context.stack_push(op2);
    context.stack_push(op1);
    context.stack_push(op4);
    context.stack_push(op3);

    Ok(true)
}

pub fn op_rot(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(3) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();
    let op3 = context.stack_pop();

    context.stack_push(op2);
    context.stack_push(op1);
    context.stack_push(op3);

    Ok(true)
}

pub fn op_2rot(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(6) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();
    let op3 = context.stack_pop();
    let op4 = context.stack_pop();
    let op5 = context.stack_pop();
    let op6 = context.stack_pop();

    context.stack_push(op4);
    context.stack_push(op3);
    context.stack_push(op2);
    context.stack_push(op1);
    context.stack_push(op6);
    context.stack_push(op5);

    Ok(true)
}

pub fn op_verify(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let op = context.stack_pop_as_element()?;

    if op.as_bool() {
        return Ok(true);
    }

    Err("exit_by_failed_verify")?
}

pub fn op_equalverify(context: &mut Context) -> StdResult<bool> {
    op_equal(context)?;
    op_verify(context)
}

pub fn op_hash256(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = hash256(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_hash160(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = hash160(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_sha256(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = sha256(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_ripemd160(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = ripemd160(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_sha1(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = sha1(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_not(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let e = context.stack_pop();

    if let Token::Element(bytes) = e {
        if bytes.len() > 4 {
            Err("input_length_too_long")?;
        }

        if bytes == ELEMENT_ZERO {
            context.stack_push(Token::Element(ELEMENT_ONE.to_vec()));
            return Ok(true);
        }
    }

    context.stack_push(Token::Element(ELEMENT_ZERO.to_vec()));

    Ok(true)
}

pub fn op_toaltstack(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let op = context.stack_pop();
    context.alt_stack_push(op);

    Ok(true)
}

pub fn op_fromaltstack(context: &mut Context) -> StdResult<bool> {
    if !context.alt_stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let op = context.alt_stack_pop();
    context.stack_push(op);

    Ok(true)
}

pub fn op_drop(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    context.stack_pop();

    Ok(true)
}

pub fn op_2drop(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    context.stack_pop();
    context.stack_pop();

    Ok(true)
}

pub fn op_nip(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    context.stack_pop();

    context.stack_push(op1);

    Ok(true)
}

pub fn op_ifdup(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let op = context.top_stack();

    if (op.is_element() && op.as_bool()) || (op.is_command() && !op.is_op_0()) {
        context.stack_push(op.clone());
    }

    Ok(true)
}

pub fn op_depth(context: &mut Context) -> StdResult<bool> {
    let len = context.stack_len();
    context.stack_push(Token::Element(element_encode(len as i64)));

    Ok(true)
}

pub fn op_over(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();

    context.stack_push(op2.clone());
    context.stack_push(op1);
    context.stack_push(op2);

    Ok(true)
}

pub fn op_tuck(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();

    context.stack_push(op1.clone());
    context.stack_push(op2);
    context.stack_push(op1);

    Ok(true)
}

pub fn op_size(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let elem = context.stack_pop_as_element()?;
    context.stack_push(elem.clone());

    if let Token::Element(v) = elem {
        context.stack_push(Token::Element(element_encode(v.len() as i64)));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_pick(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(v) = e {
        let n = element_decode(v);
        let un = n as usize;

        if !context.stack_has_enough_items(un + 1) {
            Err("not_enough_items_in_stack")?;
        }

        let elem = context.stack_get_at(un);
        context.stack_push(elem.clone());

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_roll(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(bytes) = e {
        let n = element_decode(bytes);
        let un = n as usize;

        if !context.stack_has_enough_items(un + 1) {
            Err("not_enough_items_in_stack")?;
        }

        let elem = context.stack_remove_at(un);
        context.stack_push(elem);

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_1add(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let elem = context.stack_pop_as_element()?;
    if let Token::Element(bytes) = elem {
        if bytes.len() > 4 {
            Err("input_length_too_long")?;
        }

        let n = element_decode(bytes);
        context.stack_push(Token::Element(element_encode(n + 1)));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_1sub(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let elem = context.stack_pop_as_element()?;
    if let Token::Element(bytes) = elem {
        if bytes.len() > 4 {
            Err("input_length_too_long")?;
        }

        let n = element_decode(bytes);
        context.stack_push(Token::Element(element_encode(n - 1)));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_negate(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let elem = context.stack_pop_as_element()?;
    if let Token::Element(bytes) = elem {
        if bytes.len() > 4 {
            Err("input_length_too_long")?;
        }

        let n = element_decode(bytes);
        context.stack_push(Token::Element(element_encode(-n)));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_abs(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let elem = context.stack_pop_as_element()?;
    if let Token::Element(bytes) = elem {
        if bytes.len() > 4 {
            Err("input_length_too_long")?;
        }

        let n = element_decode(bytes);
        context.stack_push(Token::Element(element_encode(n.abs())));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_booland(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    let elem1_bool = elem1.as_bool();
    let elem2_bool = elem2.as_bool();

    if let (Token::Element(left), Token::Element(right)) = (elem1, elem2) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        let booland = if elem1_bool && elem2_bool {
            ELEMENT_TRUE
        } else {
            ELEMENT_FALSE
        };
        context.stack_push(Token::Element(booland.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_boolor(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    let elem1_bool = elem1.as_bool();
    let elem2_bool = elem2.as_bool();

    if let (Token::Element(left), Token::Element(right)) = (elem1, elem2) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        let boolean = if elem1_bool || elem2_bool {
            ELEMENT_TRUE
        } else {
            ELEMENT_FALSE
        };
        context.stack_push(Token::Element(boolean.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_0notequal(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(1) {
        Err("not_enough_items_in_stack")?;
    }

    let elem = context.stack_pop_as_element()?;
    if let Token::Element(bytes) = elem {
        if bytes.len() > 4 {
            Err("input_length_too_long")?;
        }

        let n = element_decode(bytes);
        let val = if n != 0 { ELEMENT_ONE } else { ELEMENT_ZERO };

        context.stack_push(Token::Element(val.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_numequal(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (elem1, elem2) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        let boolean = if left == right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.stack_push(Token::Element(boolean.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_numequalverify(context: &mut Context) -> StdResult<bool> {
    op_numequal(context)?;
    op_verify(context)
}

pub fn op_numnotequal(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (elem1, elem2) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        let boolean = if left != right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.stack_push(Token::Element(boolean.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_lessthan(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (elem2, elem1) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        let boolean = if left < right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.stack_push(Token::Element(boolean.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_lessthanorequal(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (elem2, elem1) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        let boolean = if left <= right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.stack_push(Token::Element(boolean.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_greaterthan(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (elem2, elem1) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        let boolean = if left > right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.stack_push(Token::Element(boolean.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_greaterthanorequal(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (elem2, elem1) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        let boolean = if left >= right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.stack_push(Token::Element(boolean.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_min(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (elem1, elem2) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        if left < right {
            context.stack_push(Token::Element(left))
        } else {
            context.stack_push(Token::Element(right))
        };

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_max(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(2) {
        Err("not_enough_items_in_stack")?;
    }

    let elem1 = context.stack_pop_as_element()?;
    let elem2 = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (elem1, elem2) {
        if left.len() > 4 || right.len() > 4 {
            Err("input_length_too_long")?;
        }

        if left > right {
            context.stack_push(Token::Element(left))
        } else {
            context.stack_push(Token::Element(right))
        };

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn op_within(context: &mut Context) -> StdResult<bool> {
    if !context.stack_has_enough_items(3) {
        Err("not_enough_items_in_stack")?;
    }

    let elem_min = context.stack_pop_as_element()?;
    let elem_max = context.stack_pop_as_element()?;
    let elem = context.stack_pop_as_element()?;

    if let (Token::Element(min), Token::Element(max), Token::Element(e)) = (elem_min, elem_max, elem) {
        if min.len() > 4 || max.len() > 4 || e.len() > 4 {
            Err("input_length_too_long")?;
        }

        let boolean = if e >= min && e < max {
            ELEMENT_TRUE
        } else {
            ELEMENT_FALSE
        };
        context.stack_push(Token::Element(boolean.to_vec()));

        return Ok(true);
    }

    Err("not_an_element")?
}

pub fn not_implemented(_context: &mut Context) -> StdResult<bool> {
    unimplemented!("command not implemented")
}

pub fn deprecated(_context: &mut Context) -> StdResult<bool> {
    Err("deprecated_opcode")?
}

pub fn reserved(_context: &mut Context) -> StdResult<bool> {
    Err("exit_by_reserved")?
}

pub fn ignored(_context: &mut Context) -> StdResult<bool> {
    Ok(true)
}

// invalid if used in script
pub fn invalid(_context: &mut Context) -> StdResult<bool> {
    Err("invalid_opcode")?
}

#[cfg(test)]
mod opcode_fn_test {
    use rug::Integer;

    use crate::{
        scripting::{opcode::*, token::Token},
        std_lib::vector::string_to_bytes,
    };

    use super::*;

    #[test]
    #[should_panic(expected = "not implemented")]
    fn not_implemented_test() {
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
        let ops = vec![Token::Element(pubkey), Token::Command(OP_CHECKSIG)];

        let mut context = Context::new(ops, Integer::from(0));

        let _ = not_implemented(&mut context);
    }
}
