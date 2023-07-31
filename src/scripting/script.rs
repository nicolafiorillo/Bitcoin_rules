use rug::Integer;
use std::fmt::{Display, Formatter};

use crate::{
    encoding::varint::{varint_decode, varint_encode, VarInt},
    std_lib::vector::vect_to_hex_string,
};

use super::{
    context::{Context, ContextError},
    opcode::*,
    operation::Operation,
};

#[derive(Debug)]
pub struct Script(Vec<Operation>);

#[derive(Debug)]
pub enum ScriptError {
    InvalidScript,
    InvalidScriptLength,
    ElementTooLong,
    PushData4IsDeprecated,
}

impl Script {
    pub fn deserialize(data: &[u8]) -> Result<Self, ScriptError> {
        match varint_decode(data, 0) {
            Err(_) => Err(ScriptError::InvalidScript),
            Ok(var_int) => Self::raw_deserialize(data, &var_int),
        }
    }

    pub fn from_script_items(items: Vec<Operation>) -> Self {
        Script(items)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, ScriptError> {
        let Self(operations) = self;

        let raw = Script::raw_serialize(operations)?;

        let length = varint_encode(raw.len() as u64);
        Ok([length.as_slice(), raw.as_slice()].concat())
    }

    pub fn evaluate(&self, z: Integer) -> Result<Context, ContextError> {
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

        Ok(context)
    }

    pub fn combine(left: Self, right: Self) -> Self {
        let Self(left_items) = left;
        let Self(right_items) = right;

        Script([left_items, right_items].concat())
    }

    fn raw_serialize(operations: &Vec<Operation>) -> Result<Vec<u8>, ScriptError> {
        let mut raw: Vec<u8> = vec![];

        for operation in operations {
            match operation {
                Operation::Element(bytes) => {
                    let len = bytes.len();
                    if len <= 75 {
                        raw.push(len as u8);
                    } else if len <= 0xFF {
                        raw.push(OP_PUSHDATA1 as u8);
                        raw.push(len as u8);
                    } else if len <= 0x208 {
                        raw.push(OP_PUSHDATA2 as u8);
                        raw.extend(len.to_le_bytes().iter());
                    } else if len < 0x100000000 {
                        return Err(ScriptError::PushData4IsDeprecated);
                    } else {
                        return Err(ScriptError::ElementTooLong);
                    }

                    raw.extend(bytes);
                }
                Operation::Command(op_code) => {
                    raw.push(*op_code as u8);
                }
            }
        }

        Ok(raw)
    }

    fn raw_deserialize(data: &[u8], var_int: &VarInt) -> Result<Self, ScriptError> {
        let mut operations: Vec<Operation> = vec![];
        let length = var_int.value;

        let mut i = var_int.length as u64;
        while i <= length {
            let first = data[i as usize];
            if OP_ELEMENTS_RANGE.contains(&(first as OpCode)) {
                i += 1;

                let start = i as usize;
                let end = start + first as usize;

                let bytes = &data[start..end];
                operations.push(Operation::Element(bytes.to_vec()));

                i += first as u64;
            } else if first == OP_PUSHDATA1 as u8 {
                // TODO: NOT TESTED
                i += 1;
                let len = data[i as usize];

                i += 1;
                let start = i as usize;
                let end = start + len as usize;

                let bytes = &data[start..end];
                operations.push(Operation::Element(bytes.to_vec()));

                i += len as u64;
            } else if first == OP_PUSHDATA2 as u8 {
                // TODO: NOT TESTED
                let len_bytes = &data[(i + 1) as usize..(i + 3) as usize];
                let len = u16::from_le_bytes([len_bytes[0], len_bytes[1]]);

                i += 2;

                let start = (i + 1) as usize;
                let end = start + len as usize;

                let bytes = &data[start..end];
                operations.push(Operation::Element(bytes.to_vec()));

                i += 1 + len as u64;
            } else if first == OP_PUSHDATA4 as u8 {
                // TODO: NOT TESTED
                let len_bytes = &data[(i + 1) as usize..(i + 5) as usize];
                let len = u32::from_le_bytes([len_bytes[0], len_bytes[1], len_bytes[2], len_bytes[3]]);

                i += 4;

                let start = (i + 1) as usize;
                let end = start + len as usize;

                let bytes = &data[start..end];
                operations.push(Operation::Element(bytes.to_vec()));

                i += 1 + len as u64;
            } else {
                operations.push(Operation::Command(first as OpCode));
                i += 1;
            }
        }
        Ok(Script(operations))
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
                    write!(f, "0x{:X} ", op_code)?;
                }
            }
        }

        write!(f, "")
    }
}

#[cfg(test)]
mod script_test {
    use crate::{
        scripting::{opcode::*, operation::ELEMENT_ZERO},
        std_lib::{integer_ex::IntegerEx, vector::string_to_bytes},
    };
    use rug::Integer;

    use super::*;

    #[test]
    fn serialize() {
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34");
        let signature = string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601");

        let pubkey_script =
            Script::from_script_items(vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)]);

        let signature_script = Script::from_script_items(vec![Operation::Element(signature)]);
        let script = Script::combine(signature_script, pubkey_script);

        let serialized = script.serialize().unwrap();
        let expected = string_to_bytes("8c483045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6014104887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34ac");

        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize() {
        let data = string_to_bytes("8c483045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6014104887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34ac");
        let script = Script::deserialize(&data).unwrap();

        let Script(operations) = script;

        assert_eq!(operations.len(), 3);
        assert_eq!(operations[0], Operation::Element(string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601")));
        assert_eq!(operations[1], Operation::Element(string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34")));
        assert_eq!(operations[2], Operation::Command(OP_CHECKSIG));
    }

    #[test]
    fn evaluate_checksig() {
        let z: Integer = IntegerEx::from_hex_str("7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D");
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34");
        let signature = string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601");

        let pubkey_script =
            Script::from_script_items(vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)]);

        let signature_script = Script::from_script_items(vec![Operation::Element(signature)]);
        let script = Script::combine(signature_script, pubkey_script);

        assert!(script.evaluate(z).unwrap().is_valid());
    }

    #[test]
    fn evaluate_0() {
        let z: Integer = IntegerEx::from_hex_str("7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D");

        let script = Script::from_script_items(vec![Operation::Command(OP_0)]);
        let mut context = script.evaluate(z).unwrap();

        let op = context.pop_element().unwrap();

        assert_eq!(op, Operation::Element(ELEMENT_ZERO.to_vec()));
    }
}
