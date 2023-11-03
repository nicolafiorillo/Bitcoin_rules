use crate::{
    ecdsa::point::Point,
    hashing::{hash160::hash160, hash256::hash256, sha1::sha1, sha256::sha256},
    keys::{key::Key, signature::Signature},
};

use super::{
    constants::MAX_RETURN_DATA_LENGTH,
    context::{Context, ContextError},
    token::*,
};

/*
   Ref: https://en.bitcoin.it/wiki/Script
*/
macro_rules! op_n {
    ($n:tt, $f:ident) => {
        pub fn $f(context: &mut Context) -> Result<bool, ContextError> {
            context.stack_push(Token::Element(element_encode($n)));

            Ok(true)
        }
    };
}

// TODO: put function in code order
// TODO: split in more files, one for each category

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

pub fn op_0(context: &mut Context) -> Result<bool, ContextError> {
    context.stack_push(Token::Element(ELEMENT_ZERO.to_vec()));

    Ok(true)
}

pub fn op_1(context: &mut Context) -> Result<bool, ContextError> {
    context.stack_push(Token::Element(element_encode(1)));

    Ok(true)
}

pub fn op_1negate(context: &mut Context) -> Result<bool, ContextError> {
    context.stack_push(Token::Element(ELEMENT_ONE_NEGATE.to_vec()));

    Ok(true)
}

pub fn op_nop(_context: &mut Context) -> Result<bool, ContextError> {
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
pub fn op_return(context: &mut Context) -> Result<bool, ContextError> {
    context.set_data(Vec::new());

    if !context.tokens_are_over() {
        let token = context.next_token();

        if let Token::Element(data) = token {
            let d = data.clone();

            if d.len() > MAX_RETURN_DATA_LENGTH {
                return Err(ContextError::ReturnDataTooLong);
            }

            context.set_data(d);
        }
    }
    log::debug!("Token for OP_RETURN: {:?}", context.data());

    Err(ContextError::ExitByReturn)
}

pub fn op_if(context: &mut Context) -> Result<bool, ContextError> {
    let mut exec = false;

    if context.executing() {
        if !context.stack_has_enough_items(1) {
            return Err(ContextError::NotEnoughItemsInStack);
        }

        let token = context.stack_pop_as_element()?;
        exec = token.as_bool();
    }

    context.set_execute(exec);

    Ok(true)
}

pub fn op_notif(context: &mut Context) -> Result<bool, ContextError> {
    let mut exec = false;

    if context.executing() {
        if !context.stack_has_enough_items(1) {
            return Err(ContextError::NotEnoughItemsInStack);
        }

        let token = context.stack_pop_as_element()?;
        exec = !token.as_bool();
    }

    context.set_execute(exec);

    Ok(true)
}

pub fn op_endif(context: &mut Context) -> Result<bool, ContextError> {
    if !context.in_condition() {
        return Err(ContextError::UnexpectedEndIf);
    }

    context.unset_execute();
    Ok(true)
}

pub fn op_else(context: &mut Context) -> Result<bool, ContextError> {
    if !context.in_condition() {
        return Err(ContextError::UnexpectedElse);
    }

    context.toggle_execute();
    Ok(true)
}

pub fn op_add(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let a = context.stack_pop_as_element()?;
    let b = context.stack_pop_as_element()?;

    if let (Token::Element(a), Token::Element(b)) = (a, b) {
        let left = element_decode(a);
        let right = element_decode(b);

        let sum = left.checked_add(right).ok_or(ContextError::Overflow)?;
        context.stack_push(Token::Element(element_encode(sum)));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_mul(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
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

    Err(ContextError::NotAnElement)
}

pub fn op_equal(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let a = context.stack_pop_as_element()?;
    let b = context.stack_pop_as_element()?;

    if let (Token::Element(left), Token::Element(right)) = (a, b) {
        let equals = if left == right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.stack_push(Token::Element(equals.to_vec()));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_checksig(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let pub_key = context.stack_pop_as_element()?;
    let sig = context.stack_pop_as_element()?;

    if let Token::Element(public_key) = pub_key {
        if let Token::Element(mut der) = sig {
            // Removing last byte, that is the signature hash (SIGHASH): https://learn.saylor.org/mod/book/view.php?id=36341&chapterid=18919
            der.pop();

            let signature = match Signature::new_from_der(der) {
                Ok(signature) => signature,
                Err(_) => {
                    log::debug!("Invalid DER signature during OP_CHECKSIG");
                    return Err(ContextError::DerError);
                }
            };

            let point = Point::deserialize(public_key);
            let res = Key::verify_signature(&point, &context.z, &signature);

            let element_value = element_value_by_result(res);
            context.stack_push(Token::Element(element_value));
        }
    }

    Ok(false)
}

pub fn op_dup(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op = context.top_stack();
    context.stack_push(op.clone());

    Ok(true)
}

pub fn op_2dup(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();

    context.stack_push(op2.clone());
    context.stack_push(op1.clone());
    context.stack_push(op2);
    context.stack_push(op1);

    Ok(true)
}

pub fn op_3dup(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(3) {
        return Err(ContextError::NotEnoughItemsInStack);
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

pub fn op_2over(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(4) {
        return Err(ContextError::NotEnoughItemsInStack);
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

pub fn op_swap(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();

    context.stack_push(op1);
    context.stack_push(op2);

    Ok(true)
}

pub fn op_2swap(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(4) {
        return Err(ContextError::NotEnoughItemsInStack);
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

pub fn op_rot(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(3) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();
    let op3 = context.stack_pop();

    context.stack_push(op2);
    context.stack_push(op1);
    context.stack_push(op3);

    Ok(true)
}

pub fn op_2rot(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(6) {
        return Err(ContextError::NotEnoughItemsInStack);
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

pub fn op_verify(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op = context.stack_pop_as_element()?;

    if op.as_bool() {
        return Ok(true);
    }

    Err(ContextError::ExitByFailedVerify)
}

pub fn op_equalverify(context: &mut Context) -> Result<bool, ContextError> {
    op_equal(context)?;
    op_verify(context)
}

pub fn op_hash256(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = hash256(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_hash160(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = hash160(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_sha256(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = sha256(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_sha1(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(value) = e {
        let hash = sha1(&value);
        context.stack_push(Token::Element(hash));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_not(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let e = context.stack_pop();

    if let Token::Element(value) = e {
        if value == ELEMENT_ZERO {
            context.stack_push(Token::Element(ELEMENT_ONE.to_vec()));
            return Ok(true);
        }
    }

    context.stack_push(Token::Element(ELEMENT_ZERO.to_vec()));

    Ok(true)
}

pub fn op_toaltstack(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op = context.stack_pop();
    context.alt_stack_push(op);

    Ok(true)
}

pub fn op_fromaltstack(context: &mut Context) -> Result<bool, ContextError> {
    if !context.alt_stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op = context.alt_stack_pop();
    context.stack_push(op);

    Ok(true)
}

pub fn op_drop(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    context.stack_pop();

    Ok(true)
}

pub fn op_2drop(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    context.stack_pop();
    context.stack_pop();

    Ok(true)
}

pub fn op_nip(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op1 = context.stack_pop();
    context.stack_pop();

    context.stack_push(op1);

    Ok(true)
}

pub fn op_ifdup(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(1) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op = context.top_stack();

    if (op.is_element() && op.as_bool()) || (op.is_command() && !op.is_op_0()) {
        context.stack_push(op.clone());
    }

    Ok(true)
}

pub fn op_depth(context: &mut Context) -> Result<bool, ContextError> {
    let len = context.stack_len();
    context.stack_push(Token::Element(element_encode(len as i64)));

    Ok(true)
}

pub fn op_over(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let op1 = context.stack_pop();
    let op2 = context.stack_pop();

    context.stack_push(op2.clone());
    context.stack_push(op1);
    context.stack_push(op2);

    Ok(true)
}

pub fn op_pick(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(v) = e {
        let n = element_decode(v);
        let un = n as usize;

        if !context.stack_has_enough_items(un + 1) {
            return Err(ContextError::NotEnoughItemsInStack);
        }

        let elem = context.stack_get_at(un);
        context.stack_push(elem.clone());
    }

    Ok(true)
}

pub fn op_roll(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_enough_items(2) {
        return Err(ContextError::NotEnoughItemsInStack);
    }

    let e = context.stack_pop_as_element()?;
    if let Token::Element(v) = e {
        let n = element_decode(v);
        let un = n as usize;

        if !context.stack_has_enough_items(un + 1) {
            return Err(ContextError::NotEnoughItemsInStack);
        }

        let elem = context.stack_remove_at(un);
        context.stack_push(elem);
    }

    Ok(true)
}

pub fn not_implemented(_context: &mut Context) -> Result<bool, ContextError> {
    unimplemented!("command not implemented")
}

pub fn deprecated(_context: &mut Context) -> Result<bool, ContextError> {
    Err(ContextError::DeprecatedOpCode)
}

pub fn reserved(_context: &mut Context) -> Result<bool, ContextError> {
    Err(ContextError::ExitByReserved)
}

pub fn ignored(_context: &mut Context) -> Result<bool, ContextError> {
    Ok(true)
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
