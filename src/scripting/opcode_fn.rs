use crate::{
    ecdsa::point::Point,
    hashing::hash160::hash160,
    keys::{signature::Signature, verification::verify},
};

use super::{
    context::{Context, ContextError},
    operation::*,
};

macro_rules! op_n {
    ($n:tt, $f:ident) => {
        pub fn $f(context: &mut Context) -> Result<bool, ContextError> {
            context.push(Operation::Element(element_encode($n)));

            Ok(true)
        }
    };
}

// TODO: put function in code order

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
    context.push(Operation::Element(ELEMENT_ZERO.to_vec()));

    Ok(true)
}

pub fn op_1(context: &mut Context) -> Result<bool, ContextError> {
    context.push(Operation::Element(element_encode(1)));

    Ok(true)
}

pub fn op_1negate(context: &mut Context) -> Result<bool, ContextError> {
    context.push(Operation::Element(ELEMENT_ONE_NEGATE.to_vec()));

    Ok(true)
}

pub fn op_nop(_context: &mut Context) -> Result<bool, ContextError> {
    Ok(true)
}

pub fn op_return(_context: &mut Context) -> Result<bool, ContextError> {
    Err(ContextError::ExitByReturn)
}

pub fn op_if(context: &mut Context) -> Result<bool, ContextError> {
    let mut exec = false;

    if context.executing() {
        if !context.has_enough_items(1) {
            return Err(ContextError::NotEnoughElementsInStack);
        }

        let operation = context.pop_as_element()?;
        exec = operation.as_bool(); // OP_NOTIF is the same but with inverted exec
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
    if !context.has_enough_items(2) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let a = context.pop_as_element()?;
    let b = context.pop_as_element()?;

    if let (Operation::Element(a), Operation::Element(b)) = (a, b) {
        let left = element_decode(a);
        let right = element_decode(b);

        let sum = left + right; // TODO: check overflow
        context.push(Operation::Element(element_encode(sum)));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_mul(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(2) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let a = context.pop_as_element()?;
    let b = context.pop_as_element()?;

    if let (Operation::Element(a), Operation::Element(b)) = (a, b) {
        let left = element_decode(a);
        let right = element_decode(b);

        let sum = left * right;
        context.push(Operation::Element(element_encode(sum)));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_equal(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(2) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let a = context.pop_as_element()?;
    let b = context.pop_as_element()?;

    if let (Operation::Element(left), Operation::Element(right)) = (a, b) {
        let equals = if left == right { ELEMENT_TRUE } else { ELEMENT_FALSE };
        context.push(Operation::Element(equals.to_vec()));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_checksig(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(2) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let pub_key = context.pop_as_element()?;
    let sig = context.pop_as_element()?;

    if let Operation::Element(public_key) = pub_key {
        if let Operation::Element(mut der) = sig {
            // Removing last byte, that is the signature hash (SIGHASH): https://learn.saylor.org/mod/book/view.php?id=36341&chapterid=18919
            der.pop();

            let point = Point::deserialize(public_key);
            let signature = match Signature::new_from_der(der) {
                Ok(signature) => signature,
                Err(_) => {
                    log::debug!("Invalid DER signature during OP_CHECKSIG");
                    return Err(ContextError::DerError);
                }
            };

            let res = verify(&point, &context.z, &signature);

            let element_value = element_value_by_result(res);
            context.push(Operation::Element(element_value));
        }
    }

    Ok(false)
}

pub fn op_dup(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(1) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let op = context.top_stack();
    context.push(op.clone());

    Ok(true)
}

pub fn op_2dup(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(2) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let op1 = context.pop();
    let op2 = context.pop();

    context.push(op2.clone());
    context.push(op1.clone());
    context.push(op2);
    context.push(op1);

    Ok(true)
}

pub fn op_swap(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(2) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let op1 = context.pop();
    let op2 = context.pop();

    context.push(op1);
    context.push(op2);

    Ok(true)
}

pub fn op_verify(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(1) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let op = context.pop_as_element()?;

    if op.as_bool() {
        return Ok(true);
    }

    Err(ContextError::ExitByFailedVerify)
}

pub fn op_equalverify(context: &mut Context) -> Result<bool, ContextError> {
    op_equal(context)?;
    op_verify(context)
}

pub fn op_hash160(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(1) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let e = context.pop_as_element()?;
    if let Operation::Element(value) = e {
        let hash = hash160(&value);
        context.push(Operation::Element(hash));

        return Ok(true);
    }

    Err(ContextError::NotAnElement)
}

pub fn op_not(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_enough_items(1) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let e = context.pop();

    if let Operation::Element(value) = e {
        if value == ELEMENT_ZERO {
            context.push(Operation::Element(ELEMENT_ONE.to_vec()));
            return Ok(true);
        }
    }

    context.push(Operation::Element(ELEMENT_ZERO.to_vec()));

    Ok(true)
}

pub fn not_implemented(_context: &mut Context) -> Result<bool, ContextError> {
    unimplemented!("operation not implemented")
}

pub fn deprecated(_context: &mut Context) -> Result<bool, ContextError> {
    Err(ContextError::DeprecatedOpCode)
}

#[cfg(test)]
mod opcode_fn_test {
    use rug::Integer;

    use crate::{
        scripting::{opcode::*, operation::Operation},
        std_lib::vector::string_to_bytes,
    };

    use super::*;

    #[test]
    #[should_panic(expected = "not implemented")]
    fn not_implemented_test() {
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
        let ops = vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)];

        let mut context = Context::new(ops, Integer::from(0));

        let _ = not_implemented(&mut context);
    }
}
