use crate::{
    ecdsa::point::Point,
    keys::{signature::Signature, verification::verify},
};

use super::{
    context::{Context, ContextError},
    operation::{Operation, ELEMENT_ONE, ELEMENT_ZERO},
};

fn element_value_by_result(res: bool) -> Vec<u8> {
    if res {
        ELEMENT_ONE.to_vec()
    } else {
        ELEMENT_ZERO.to_vec()
    }
}

pub fn op_checksig(context: &mut Context) -> Result<bool, ContextError> {
    if !context.stack_has_at_least_two_elements() {
        return Err(ContextError::NotEnoughElementsInStack);
    }

    let pub_key = context.pop_element_from_stack()?;
    let sig = context.pop_element_from_stack()?;

    if let Operation::Element(public_key) = pub_key {
        if let Operation::Element(mut der) = sig {
            // Removing last byte, that is the signature hash (SIGHASH): https://learn.saylor.org/mod/book/view.php?id=36341&chapterid=18919
            der.pop();

            let point = Point::deserialize(public_key);
            let signature = Signature::new_from_der(der).unwrap();

            let res = verify(&point, &context.z(), &signature);

            let element_value = element_value_by_result(res);
            context.push_on_stack(Operation::Element(element_value));
        }
    }

    Ok(false)
}

pub fn not_implemented(_context: &mut Context) -> Result<bool, ContextError> {
    panic!("not implemented");
}

#[cfg(test)]
mod opcode_fn_test {
    use rug::Integer;

    use crate::{
        scripting::{opcode::*, operation::Operation},
        std_lib::{integer_ex::IntegerEx, vector::string_to_bytes},
    };

    use super::*;

    #[test]
    #[should_panic(expected = "not implemented")]
    fn not_implemented_test() {
        let z: Integer = IntegerEx::from_hex_str("7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D");
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34");
        let ops = vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)];

        let mut context = Context::new(ops, z);

        let _ = not_implemented(&mut context);
    }
}
