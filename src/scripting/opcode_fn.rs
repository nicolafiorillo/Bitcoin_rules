use crate::{
    ecdsa::point::Point,
    keys::{signature::Signature, verification::verify},
};

use super::{
    context::{Context, ContextError},
    operation::{element_encode, Operation, ELEMENT_ONE, ELEMENT_ONE_NEGATE, ELEMENT_ZERO},
};

macro_rules! op_n {
    ($n:tt, $f:ident) => {
        pub fn $f(context: &mut Context) -> Result<bool, ContextError> {
            context.push_element(Operation::Element(element_encode($n)));

            Ok(true)
        }
    };
}

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
    context.push_element(Operation::Element(ELEMENT_ZERO.to_vec()));

    Ok(true)
}

pub fn op_1(context: &mut Context) -> Result<bool, ContextError> {
    context.push_element(Operation::Element(element_encode(1)));

    Ok(true)
}

pub fn op_1negate(context: &mut Context) -> Result<bool, ContextError> {
    context.push_element(Operation::Element(ELEMENT_ONE_NEGATE.to_vec()));

    Ok(true)
}

pub fn op_checksig(context: &mut Context) -> Result<bool, ContextError> {
    if !context.has_elements(2) {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let pub_key = context.pop_element()?;
    let sig = context.pop_element()?;

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
            context.push_element(Operation::Element(element_value));
        }
    }

    Ok(false)
}

pub fn not_implemented(_context: &mut Context) -> Result<bool, ContextError> {
    unimplemented!("not implemented")
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
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34");
        let ops = vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)];

        let mut context = Context::new(ops, Integer::from(0));

        let _ = not_implemented(&mut context);
    }
}
