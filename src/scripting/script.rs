use rug::Integer;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

use crate::std_lib::vector::vect_to_hex_string;

use super::{
    context::{Context, ContextError},
    opcode::{OPS_LENGTH, OP_TO_FN},
    operation::Operation,
};

#[derive(Debug)]
pub struct Script(Vec<Operation>);

pub enum ScriptError {
    InvalidScript,
}

impl Script {
    pub fn deserialize(data: &[u8]) -> Result<Self, ScriptError> {
        Err(ScriptError::InvalidScript)
    }

    pub fn from_script_items(items: Vec<Operation>) -> Self {
        Script(items)
    }

    pub fn evaluate(&self, z: Integer) -> Result<bool, ContextError> {
        let Self(operations) = self;

        let mut context = Context::new(operations.clone(), z);

        while !context.is_over() {
            let operation = context.next();
            match operation {
                Operation::Element(bytes) => {
                    let e = Operation::Element(bytes.to_vec());
                    context.push_element(e);
                }
                Operation::Command(op_code) => {
                    if op_code > &OPS_LENGTH {
                        return Err(ContextError::InvalidOpCode);
                    }

                    (*OP_TO_FN)[*op_code](&mut context)?;
                }
            }
        }

        Ok(context.is_valid())
    }

    pub fn combine(left: Self, right: Self) -> Self {
        let Self(left_items) = left;
        let Self(right_items) = right;

        Script([left_items, right_items].concat())
    }
}

impl Display for Script {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self(items) = self;
        for item in items {
            match item {
                Operation::Element(bytes) => {
                    let e = vect_to_hex_string(bytes);
                    write!(f, "{:} ", e)?;
                }
                Operation::Command(op_code) => {
                    write!(f, "{:?} ", op_code)?;
                }
            }
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod script_test {
    use crate::{
        scripting::opcode::*,
        std_lib::{integer_ex::IntegerEx, vector::string_to_bytes},
    };
    use rug::Integer;

    use super::*;

    #[test]
    fn evaluate_checksig() {
        let z: Integer = IntegerEx::from_hex_str("7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D");
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34");
        let signature = string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601");

        let pubkey_script =
            Script::from_script_items(vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)]);

        let signature_script = Script::from_script_items(vec![Operation::Element(signature)]);
        let script = Script::combine(signature_script, pubkey_script);

        assert!(script.evaluate(z).unwrap());
    }
}
