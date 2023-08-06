use rug::Integer;
use std::fmt::{Display, Formatter};

use crate::{
    encoding::varint::{varint_decode, varint_encode, VarInt},
    std_lib::vector::{string_to_bytes, vect_to_hex_string},
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
    InvalidScriptRepresentation,
}

impl Script {
    pub fn deserialize(data: &[u8]) -> Result<Self, ScriptError> {
        match varint_decode(data, 0) {
            Err(_) => Err(ScriptError::InvalidScript),
            Ok(var_int) => Self::raw_deserialize(data, &var_int),
        }
    }

    pub fn from_operations(items: Vec<Operation>) -> Self {
        Script(items)
    }

    pub fn from_representation(repr: &str) -> Result<Self, ScriptError> {
        let trimmed_repr = repr.trim();
        let mut items: Vec<Operation> = vec![];

        let tokens = trimmed_repr.split(' ').collect::<Vec<&str>>();

        for item in tokens {
            if let Some(op_code) = OP_TO_FN.iter().position(|op| op.name == item) {
                items.push(Operation::Command(op_code));
            } else {
                match string_to_bytes(item) {
                    Ok(bytes) => items.push(Operation::Element(bytes)),
                    Err(_) => return Err(ScriptError::InvalidScriptRepresentation),
                };
            }
        }

        Ok(Script(items))
    }

    pub fn representation(&self) -> String {
        let Self(items) = self;

        let mut repr = String::new();
        for item in items {
            match item {
                Operation::Element(bytes) => {
                    let e = vect_to_hex_string(bytes);
                    repr.push_str(&e);
                    repr.push(' ');
                }
                Operation::Command(op_code) => {
                    repr.push_str((*OP_TO_FN)[*op_code].name);
                    repr.push(' ');
                }
            }
        }

        repr.trim_end().to_string()
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
            let executing = context.executing();

            let operation = context.next_token();
            log::debug!("Operation (exec: {}): {:?}", executing, operation);

            if !executing && !operation.is_op_condition() {
                continue;
            }

            match operation {
                Operation::Element(bytes) => {
                    let e = Operation::Element(bytes.to_vec());
                    context.push(e);
                }
                Operation::Command(op_code) => {
                    if *op_code > OPS_LENGTH {
                        return Err(ContextError::InvalidOpCode);
                    }

                    ((*OP_TO_FN)[*op_code].exec)(&mut context)?;
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
        write!(f, "{:}", self.representation())
    }
}

#[cfg(test)]
mod script_test {
    use crate::{
        scripting::{opcode::*, operation::*},
        std_lib::{integer_ex::IntegerEx, vector::string_to_bytes},
    };
    use rug::Integer;

    use super::*;

    #[test]
    fn represent() {
        let script = Script::from_operations(vec![
            Operation::Element(vec![0x00]),
            Operation::Element(vec![0x01]),
            Operation::Command(OP_CHECKSIG),
        ]);

        assert_eq!(format!("{}", script), "00 01 OP_CHECKSIG");
    }

    #[test]
    fn from_representation() {
        let expected = vec![
            Operation::Element(vec![0x00]),
            Operation::Element(vec![0x01]),
            Operation::Command(OP_CHECKSIG),
        ];

        let script = Script::from_representation("00 01 OP_CHECKSIG").unwrap();
        let Script(operations) = script;

        assert_eq!(expected, operations);
    }

    #[test]
    fn serialize() {
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
        let signature = string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();

        let pubkey_script = Script::from_operations(vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)]);

        let signature_script = Script::from_operations(vec![Operation::Element(signature)]);
        let script = Script::combine(signature_script, pubkey_script);

        let serialized = script.serialize().unwrap();
        let expected = string_to_bytes("8c483045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6014104887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34ac").unwrap();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize() {
        let data = string_to_bytes("8c483045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6014104887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34ac").unwrap();
        let script = Script::deserialize(&data).unwrap();

        let Script(operations) = script;

        assert_eq!(operations.len(), 3);
        assert_eq!(operations[0], Operation::Element(string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap()));
        assert_eq!(operations[1], Operation::Element(string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap()));
        assert_eq!(operations[2], Operation::Command(OP_CHECKSIG));
    }

    #[test]
    fn evaluate_checksig() {
        let z: Integer = IntegerEx::from_hex_str("7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D");
        let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
        let signature = string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();

        let pubkey_script = Script::from_operations(vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)]);

        let signature_script = Script::from_operations(vec![Operation::Element(signature)]);
        let script = Script::combine(signature_script, pubkey_script);

        assert!(script.evaluate(z).unwrap().is_valid());
    }

    #[test]
    fn evaluate_0() {
        let script = Script::from_operations(vec![Operation::Command(OP_0)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(ELEMENT_ZERO.to_vec()));
    }

    #[test]
    fn evaluate_1() {
        let script = Script::from_operations(vec![Operation::Command(OP_1)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(1)));
    }

    #[test]
    fn evaluate_2() {
        let script = Script::from_operations(vec![Operation::Command(OP_2)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(2)));
    }
    #[test]
    fn evaluate_3() {
        let script = Script::from_operations(vec![Operation::Command(OP_3)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(3)));
    }
    #[test]
    fn evaluate_4() {
        let script = Script::from_operations(vec![Operation::Command(OP_4)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(4)));
    }
    #[test]
    fn evaluate_5() {
        let script = Script::from_operations(vec![Operation::Command(OP_5)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(5)));
    }
    #[test]
    fn evaluate_6() {
        let script = Script::from_operations(vec![Operation::Command(OP_6)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(6)));
    }
    #[test]
    fn evaluate_7() {
        let script = Script::from_operations(vec![Operation::Command(OP_7)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(7)));
    }
    #[test]
    fn evaluate_8() {
        let script = Script::from_operations(vec![Operation::Command(OP_8)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(8)));
    }
    #[test]
    fn evaluate_9() {
        let script = Script::from_operations(vec![Operation::Command(OP_9)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(9)));
    }
    #[test]
    fn evaluate_10() {
        let script = Script::from_operations(vec![Operation::Command(OP_10)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(10)));
    }
    #[test]
    fn evaluate_11() {
        let script = Script::from_operations(vec![Operation::Command(OP_11)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(11)));
    }
    #[test]
    fn evaluate_12() {
        let script = Script::from_operations(vec![Operation::Command(OP_12)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(12)));
    }
    #[test]
    fn evaluate_13() {
        let script = Script::from_operations(vec![Operation::Command(OP_13)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(13)));
    }
    #[test]
    fn evaluate_14() {
        let script = Script::from_operations(vec![Operation::Command(OP_14)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(14)));
    }
    #[test]
    fn evaluate_15() {
        let script = Script::from_operations(vec![Operation::Command(OP_15)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(15)));
    }
    #[test]
    fn evaluate_16() {
        let script = Script::from_operations(vec![Operation::Command(OP_16)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(element_encode(16)));
    }

    #[test]
    fn evaluate_negate() {
        let script = Script::from_operations(vec![Operation::Command(OP_1NEGATE)]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(ELEMENT_ONE_NEGATE.to_vec()));
    }

    #[test]
    fn evaluate_nop() {
        let script = Script::from_operations(vec![Operation::Command(OP_NOP)]);
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(0));
    }

    #[test]
    fn evaluate_add() {
        let script = Script::from_operations(vec![
            Operation::Element(vec![0x01]),
            Operation::Element(vec![0x02]),
            Operation::Command(OP_ADD),
        ]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(vec![0x03]));
    }

    #[test]
    fn evaluate_mul() {
        let script = Script::from_operations(vec![
            Operation::Element(vec![0x02]),
            Operation::Element(vec![0x02]),
            Operation::Command(OP_MUL),
        ]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(vec![0x04]));
    }

    #[test]
    fn evaluate_equal_true() {
        let script = Script::from_operations(vec![
            Operation::Element(vec![0x01]),
            Operation::Element(vec![0x01]),
            Operation::Command(OP_EQUAL),
        ]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(ELEMENT_TRUE.to_vec()));
    }

    #[test]
    fn evaluate_equal_false() {
        let script = Script::from_operations(vec![
            Operation::Element(vec![0x01]),
            Operation::Element(vec![0x02]),
            Operation::Command(OP_EQUAL),
        ]);
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        let op = context.pop_as_element().unwrap();

        assert_eq!(op, Operation::Element(ELEMENT_FALSE.to_vec()));
    }

    #[test]
    fn evaluate_if() {
        let script = Script::from_operations(vec![Operation::Element(vec![0x01]), Operation::Command(OP_IF)]);
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(0));
        assert!(context.executing())
    }

    #[test]
    fn evaluate_return() {
        let script = Script::from_operations(vec![Operation::Element(vec![0x01]), Operation::Command(OP_RETURN)]);
        let context = script.evaluate(Integer::from(0));

        assert_eq!(ContextError::ExitByReturn, context.expect_err("Err"));
    }

    #[test]
    fn evaluate_if_endif() {
        let script = Script::from_operations(vec![
            Operation::Element(vec![0x01]),
            Operation::Command(OP_IF),
            Operation::Command(OP_ENDIF),
        ]);
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(0));
        assert!(context.executing())
    }

    #[test]
    fn evaluate_if_else_endif() {
        let script = Script::from_operations(vec![
            Operation::Element(vec![0x01]),
            Operation::Command(OP_IF),
            Operation::Command(OP_ELSE),
            Operation::Command(OP_ENDIF),
        ]);
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(0));
        assert!(context.executing())
    }

    #[test]
    fn evaluate_conditional_script_1() {
        let script = Script::from_representation("01 00 OP_IF 02 OP_ENDIF").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(1));

        assert!(context.executing());
        assert!(context.is_valid());
    }

    #[test]
    fn evaluate_conditional_script_2() {
        let script = Script::from_representation("01 01 OP_IF 02 OP_ENDIF").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(2));

        assert!(context.executing());
        assert!(!context.is_valid());
    }

    #[test]
    fn evaluate_conditional_script_3() {
        let script = Script::from_representation("00 OP_IF 01 OP_ELSE 00 OP_ENDIF").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(1));

        assert!(context.executing());
        assert!(!context.is_valid());
    }

    #[test]
    fn evaluate_conditional_script_4() {
        let script = Script::from_representation("01 OP_IF 01 OP_ELSE 00 OP_ENDIF").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(1));

        assert!(context.executing());
        assert!(context.is_valid());
    }

    #[test]
    fn evaluate_script_nested_if_1() {
        let script = Script::from_representation("00 OP_IF 01 OP_IF OP_RETURN OP_ELSE OP_RETURN OP_ELSE OP_RETURN OP_ENDIF OP_ELSE 01 OP_IF 01 OP_ELSE OP_RETURN OP_ELSE 01 OP_ENDIF OP_ELSE OP_RETURN OP_ENDIF OP_ADD 02 OP_EQUAL").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.is_valid());
    }

    #[test]
    fn evaluate_script_nested_if_2() {
        let script = Script::from_representation("20 OP_IF 00 OP_IF OP_RETURN OP_ELSE 10 OP_ENDIF OP_ELSE 01 OP_IF 01 OP_ELSE OP_RETURN OP_ELSE 01 OP_ENDIF OP_ELSE 30 OP_ENDIF OP_ADD 40 OP_EQUAL").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.is_valid());
    }

    #[test]
    fn evaluate_dup() {
        let script = Script::from_representation("09 OP_DUP").unwrap();
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(2));

        let op = context.pop_as_element().unwrap();
        assert_eq!(op, Operation::Element(vec![0x09]));

        let op = context.pop_as_element().unwrap();
        assert_eq!(op, Operation::Element(vec![0x09]));
    }

    #[test]
    fn evaluate_2dup() {
        let script = Script::from_representation("01 02 OP_2DUP").unwrap();
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(4));

        let op = context.pop_as_element().unwrap();
        assert_eq!(op, Operation::Element(vec![0x01]));

        let op = context.pop_as_element().unwrap();
        assert_eq!(op, Operation::Element(vec![0x02]));

        let op = context.pop_as_element().unwrap();
        assert_eq!(op, Operation::Element(vec![0x01]));

        let op = context.pop_as_element().unwrap();
        assert_eq!(op, Operation::Element(vec![0x02]));
    }

    #[test]
    fn evaluate_hash160() {
        let script = Script::from_representation("09 OP_HASH160").unwrap();
        let mut context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.has_elements(1));

        let op = context.pop_as_element().unwrap();

        let expected = string_to_bytes("d6a8a804d5be366ae5d3a318cdced1dc1cfe28ea").unwrap();
        assert_eq!(op, Operation::Element(expected));
    }

    #[test]
    fn evaluate_verify_true() {
        let script = Script::from_representation("09 OP_VERIFY 01").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.is_valid());
        assert!(context.has_elements(1));
    }

    #[test]
    fn evaluate_verify_false() {
        let script = Script::from_representation("00 OP_VERIFY").unwrap();
        let context = script.evaluate(Integer::from(0));

        assert_eq!(ContextError::ExitByFailedVerify, context.expect_err("Err"));
    }

    #[test]
    fn evaluate_equalverify_true() {
        let script = Script::from_representation("09 09 OP_EQUALVERIFY 01").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.is_valid());
        assert!(context.has_elements(1));
    }

    #[test]
    fn evaluate_equalverify_false() {
        let script = Script::from_representation("09 08 OP_EQUALVERIFY 01").unwrap();
        let context = script.evaluate(Integer::from(0));

        assert_eq!(ContextError::ExitByFailedVerify, context.expect_err("Err"));
    }

    #[test]
    fn evaluate_generic_script_1() {
        let script = Script::from_representation("02 OP_DUP OP_DUP OP_MUL OP_ADD OP_6 OP_EQUAL").unwrap();
        let context = script.evaluate(Integer::from(0)).unwrap();

        assert!(context.is_valid());
        assert!(context.has_elements(1));
    }

    // #[test]
    // fn evaluate_generic_script_2() {
    //     let script =
    //         Script::from_representation("OP_2DUP OP_EQUAL OP_NOT OP_VERIFY OP_SHA1 OP_SWAP OP_SHA1 OP_EQUAL").unwrap();
    //     let context = script.evaluate(Integer::from(0)).unwrap();

    //     assert!(context.is_valid());
    //     assert!(context.has_elements(1));
    // }

    // OP_2DUP
    // OP_NOT
    // OP_SHA1
    // OP_SWAP
    // OP_SHA256
    // OP_HASH160
    // OP_HASH256

    //
    // Deprecated
    //
    macro_rules! evaluate_op_deprecated {
        ($n:literal, $f:ident) => {
            #[test]
            fn $f() {
                let script = Script::from_representation($n).unwrap();
                let context = script.evaluate(Integer::from(0));

                assert_eq!(ContextError::DeprecatedOpCode, context.expect_err("Err"));
            }
        };
    }

    // evaluate_op_deprecated!("OP_MUL", evaluate_op_mul);
    evaluate_op_deprecated!("OP_CAT", evaluate_op_cat);
    evaluate_op_deprecated!("OP_SUBSTR", evaluate_op_substr);
    evaluate_op_deprecated!("OP_LEFT", evaluate_op_left);
    evaluate_op_deprecated!("OP_RIGHT", evaluate_op_right);
    evaluate_op_deprecated!("OP_INVERT", evaluate_op_invert);
    evaluate_op_deprecated!("OP_AND", evaluate_op_and);
    evaluate_op_deprecated!("OP_OR", evaluate_op_or);
    evaluate_op_deprecated!("OP_XOR", evaluate_op_xor);
    evaluate_op_deprecated!("OP_2MUL", evaluate_op_2mul);
    evaluate_op_deprecated!("OP_2DIV", evaluate_op_2div);
    evaluate_op_deprecated!("OP_DIV", evaluate_op_div);
    evaluate_op_deprecated!("OP_MOD", evaluate_op_mod);
    evaluate_op_deprecated!("OP_LSHIFT", evaluate_op_lshift);
    evaluate_op_deprecated!("OP_RSHIFT", evaluate_op_rshift);

    //
    // P2PK
    //
    #[test]
    fn evaluate_p2pk() {
        let signature = "3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601";
        let pubkey = "04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";

        let script = Script::from_representation(&format!("{} {} OP_CHECKSIG", signature, pubkey)).unwrap();
        let z: Integer = IntegerEx::from_hex_str("7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D");

        assert!(script.evaluate(z).unwrap().is_valid());
    }

    //
    // P2PKH
    //
    #[test]
    fn evaluate_p2pkh() {
        let signature = "3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601";
        let pubkey = "04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
        let hash = "fb6c931433c83e8bb5a4c6588c7fc24c08dac6e3";

        let script = Script::from_representation(&format!(
            "{} {} OP_DUP OP_HASH160 {} OP_EQUALVERIFY OP_CHECKSIG",
            signature, pubkey, hash
        ))
        .unwrap();
        let z: Integer = IntegerEx::from_hex_str("7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D");

        assert!(script.evaluate(z).unwrap().is_valid());
    }
}
