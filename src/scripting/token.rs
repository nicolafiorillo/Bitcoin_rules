use std::fmt::{Display, Formatter};

use crate::{
    scripting::opcode,
    scripting::opcode::OP_TO_FN,
    std_lib::vector::{padding_left, trim_right, vect_to_hex_string},
};

use super::opcode::OpCode;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Element(Vec<u8>),
    Command(OpCode),
}

impl Token {
    pub fn as_bool(&self) -> bool {
        match self {
            Token::Element(value) => {
                for i in value {
                    // TODO: can be negative zero
                    // if (i == vch.size()-1 && vch[i] == 0x80)
                    // return false;
                    // see: https://github.com/bitcoin/bitcoin/blob/a4ca4975880c4f870c6047065c70610af2529e74/src/script/interpreter.cpp#L42
                    if *i != 0 {
                        return true;
                    }
                }

                false
            }
            _ => false,
        }
    }

    pub fn is_op_branch_condition(&self) -> bool {
        matches!(
            self,
            Token::Command(opcode::OP_IF)
                | Token::Command(opcode::OP_NOTIF)
                | Token::Command(opcode::OP_ELSE)
                | Token::Command(opcode::OP_ENDIF)
        )
    }

    pub fn is_op_0(&self) -> bool {
        matches!(self, Token::Command(opcode::OP_0))
    }

    pub fn is_command(&self) -> bool {
        matches!(self, Token::Command(_))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Token::Element(_))
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Token::Element(bytes) => vect_to_hex_string(bytes),
            Token::Command(op_code) => (*OP_TO_FN)[*op_code].name.to_string(),
        };

        write!(f, "{:}", s)
    }
}

pub static ELEMENT_ZERO: [u8; 1] = [0x00];
pub static ELEMENT_ONE: [u8; 1] = [0x01];
pub static ELEMENT_ONE_NEGATE: [u8; 1] = [0x81];
pub static ELEMENT_TRUE: [u8; 1] = ELEMENT_ONE;
pub static ELEMENT_FALSE: [u8; 1] = ELEMENT_ZERO;

pub fn element_encode(num: i64) -> Vec<u8> {
    if num == 0 {
        return vec![];
    }

    let abs_num = num.abs();
    let negative = num < 0;

    let mut res = abs_num.to_le_bytes().to_vec();
    res = trim_right(&res, 0);

    let last_pos = res.len() - 1;
    let last = res[last_pos];

    if (last & 0x80) == 0x80 {
        if negative {
            res.push(0x80);
        } else {
            res.push(0x00);
        }
    } else if negative {
        res[last_pos] = last | 0x80;
    }

    res
}

pub fn element_decode(bytes: Vec<u8>) -> i64 {
    if bytes.is_empty() {
        return 0;
    }

    let mut big_endian = bytes;
    big_endian.reverse();

    let mut first = big_endian[0];

    let negative: i8 = if (first & 0x80) == 0x80 { -1 } else { 1 };
    first = if negative == -1 { first & 0x7F } else { first };

    big_endian[0] = first;

    let padded_bytes = padding_left(&big_endian, 8, 0);
    let value = i64::from_be_bytes(padded_bytes.try_into().unwrap());

    value * negative as i64
}

#[cfg(test)]
mod token_test {
    use super::*;

    #[test]
    fn test_0() {
        let encoded: Vec<u8> = element_encode(0);
        assert!(encoded.is_empty());

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 0);
    }

    #[test]
    fn test_1() {
        let encoded: Vec<u8> = element_encode(1);
        assert_eq!(encoded, [0x1]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 1);
    }

    #[test]
    fn test_1_neg() {
        let encoded: Vec<u8> = element_encode(-1);
        assert_eq!(encoded, [0x81]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -1);
    }

    #[test]
    fn test_2() {
        let encoded: Vec<u8> = element_encode(2);
        assert_eq!(encoded, [0x2]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 2);
    }

    #[test]
    fn test_2_neg() {
        let encoded: Vec<u8> = element_encode(-2);
        assert_eq!(encoded, [0x82]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -2);
    }

    #[test]
    fn test_4() {
        let encoded: Vec<u8> = element_encode(4);
        assert_eq!(encoded, [0x4]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 4);
    }

    #[test]
    fn test_4_neg() {
        let encoded: Vec<u8> = element_encode(-4);
        assert_eq!(encoded, [0x84]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -4);
    }

    #[test]
    fn test_8() {
        let encoded: Vec<u8> = element_encode(8);
        assert_eq!(encoded, [0x8]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 8);
    }

    #[test]
    fn test_8_neg() {
        let encoded: Vec<u8> = element_encode(-8);
        assert_eq!(encoded, [0x88]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -8);
    }

    #[test]
    fn test_16() {
        let encoded: Vec<u8> = element_encode(16);
        assert_eq!(encoded, [0x10]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 16);
    }

    #[test]
    fn test_16_neg() {
        let encoded: Vec<u8> = element_encode(-16);
        assert_eq!(encoded, [0x90]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -16);
    }

    #[test]
    fn test_32() {
        let encoded: Vec<u8> = element_encode(32);
        assert_eq!(encoded, [0x20]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 32);
    }

    #[test]
    fn test_32_neg() {
        let encoded: Vec<u8> = element_encode(-32);
        assert_eq!(encoded, [0xA0]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -32);
    }

    #[test]
    fn test_64() {
        let encoded: Vec<u8> = element_encode(64);
        assert_eq!(encoded, [0x40]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 64);
    }

    #[test]
    fn test_64_neg() {
        let encoded: Vec<u8> = element_encode(-64);
        assert_eq!(encoded, [0xC0]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -64);
    }

    #[test]
    fn test_128() {
        let encoded: Vec<u8> = element_encode(128);
        assert_eq!(encoded, [0x80, 0x00]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 128);
    }

    #[test]
    fn test_128_neg() {
        let encoded: Vec<u8> = element_encode(-128);
        assert_eq!(encoded, [0x80, 0x80]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -128);
    }

    #[test]
    fn test_129() {
        let encoded: Vec<u8> = element_encode(129);
        assert_eq!(encoded, [0x81, 0x00]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 129);
    }

    #[test]
    fn test_129_neg() {
        let encoded: Vec<u8> = element_encode(-129);
        assert_eq!(encoded, [0x81, 0x80]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -129);
    }

    #[test]
    fn test_255() {
        let encoded: Vec<u8> = element_encode(255);
        assert_eq!(encoded, [0xFF, 0x00]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 255);
    }

    #[test]
    fn test_255_neg() {
        let encoded: Vec<u8> = element_encode(-255);
        assert_eq!(encoded, [0xFF, 0x80]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -255);
    }

    #[test]
    fn test_256() {
        let encoded: Vec<u8> = element_encode(256);
        assert_eq!(encoded, [0x00, 0x01]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 256);
    }

    #[test]
    fn test_256_neg() {
        let encoded: Vec<u8> = element_encode(-256);
        assert_eq!(encoded, [0x00, 0x81]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -256);
    }

    #[test]
    fn test_512() {
        let encoded: Vec<u8> = element_encode(512);
        assert_eq!(encoded, [0x00, 0x02]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 512);
    }

    #[test]
    fn test_512_neg() {
        let encoded: Vec<u8> = element_encode(-512);
        assert_eq!(encoded, [0x00, 0x82]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -512);
    }

    #[test]
    fn test_1024() {
        let encoded: Vec<u8> = element_encode(1024);
        assert_eq!(encoded, [0x00, 0x04]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 1024);
    }

    #[test]
    fn test_1024_neg() {
        let encoded: Vec<u8> = element_encode(-1024);
        assert_eq!(encoded, [0x00, 0x84]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -1024);
    }

    #[test]
    fn test_2048() {
        let encoded: Vec<u8> = element_encode(2048);
        assert_eq!(encoded, [0x00, 0x08]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 2048);
    }

    #[test]
    fn test_2048_neg() {
        let encoded: Vec<u8> = element_encode(-2048);
        assert_eq!(encoded, [0x00, 0x88]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -2048);
    }

    #[test]
    fn test_4096() {
        let encoded: Vec<u8> = element_encode(4096);
        assert_eq!(encoded, [0x00, 0x10]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 4096);
    }

    #[test]
    fn test_4096_neg() {
        let encoded: Vec<u8> = element_encode(-4096);
        assert_eq!(encoded, [0x00, 0x90]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -4096);
    }

    #[test]
    fn test_8192() {
        let encoded: Vec<u8> = element_encode(8192);
        assert_eq!(encoded, [0x00, 0x20]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 8192);
    }

    #[test]
    fn test_8192_neg() {
        let encoded: Vec<u8> = element_encode(-8192);
        assert_eq!(encoded, [0x00, 0xA0]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -8192);
    }

    #[test]
    fn test_32767() {
        let encoded: Vec<u8> = element_encode(32767);
        assert_eq!(encoded, [0xFF, 0x7F]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 32767);
    }

    #[test]
    fn test_32767_neg() {
        let encoded: Vec<u8> = element_encode(-32767);
        assert_eq!(encoded, [0xFF, 0xFF]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -32767);
    }

    #[test]
    fn test_32768() {
        let encoded: Vec<u8> = element_encode(32768);
        assert_eq!(encoded, [0x00, 0x80, 0x00]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 32768);
    }

    #[test]
    fn test_32768_neg() {
        let encoded: Vec<u8> = element_encode(-32768);
        assert_eq!(encoded, [0x00, 0x80, 0x80]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -32768);
    }

    #[test]
    fn test_32769() {
        let encoded: Vec<u8> = element_encode(32769);
        assert_eq!(encoded, [0x01, 0x80, 0x00]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 32769);
    }

    #[test]
    fn test_32769_neg() {
        let encoded: Vec<u8> = element_encode(-32769);
        assert_eq!(encoded, [0x01, 0x80, 0x80]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -32769);
    }

    #[test]
    fn test_65535() {
        let encoded: Vec<u8> = element_encode(65535);
        assert_eq!(encoded, [0xFF, 0xFF, 0x00]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 65535);
    }

    #[test]
    fn test_65535_neg() {
        let encoded: Vec<u8> = element_encode(-65535);
        assert_eq!(encoded, [0xFF, 0xFF, 0x80]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -65535);
    }

    #[test]
    fn test_65536() {
        let encoded: Vec<u8> = element_encode(65536);
        assert_eq!(encoded, [0x00, 0x00, 0x01]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 65536);
    }

    #[test]
    fn test_65536_neg() {
        let encoded: Vec<u8> = element_encode(-65536);
        assert_eq!(encoded, [0x00, 0x00, 0x81]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -65536);
    }

    #[test]
    fn test_4294967295() {
        let encoded: Vec<u8> = element_encode(4294967295);
        assert_eq!(encoded, [0xFF, 0xFF, 0xFF, 0xFF, 0x00]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 4294967295);
    }

    #[test]
    fn test_4294967295_neg() {
        let encoded: Vec<u8> = element_encode(-4294967295);
        assert_eq!(encoded, [0xFF, 0xFF, 0xFF, 0xFF, 0x80]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -4294967295);
    }

    #[test]
    fn test_4294967296() {
        let encoded: Vec<u8> = element_encode(4294967296);
        assert_eq!(encoded, [0x00, 0x00, 0x00, 0x00, 0x01]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 4294967296);
    }

    #[test]
    fn test_4294967296_neg() {
        let encoded: Vec<u8> = element_encode(-4294967296);
        assert_eq!(encoded, [0x00, 0x00, 0x00, 0x00, 0x81]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -4294967296);
    }

    #[test]
    fn test_4294967297() {
        let encoded: Vec<u8> = element_encode(4294967297);
        assert_eq!(encoded, [0x01, 0x00, 0x00, 0x00, 0x01]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, 4294967297);
    }

    #[test]
    fn test_4294967297_neg() {
        let encoded: Vec<u8> = element_encode(-4294967297);
        assert_eq!(encoded, [0x01, 0x00, 0x00, 0x00, 0x81]);

        let decoded = element_decode(encoded);
        assert_eq!(decoded, -4294967297);
    }

    // Other cases to test:
    //
    // 18446744073709551615 - "FFFFFFFFFFFFFFFF00"
    // 18446744073709551616 - "000000000000000001"
    // 18446744073709551617 - "010000000000000001"
    // 340282366920938463463374607431768211455 - "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00"
    // 340282366920938463463374607431768211456 - "0000000000000000000000000000000001"
    // 340282366920938463463374607431768211457 - "0100000000000000000000000000000001"
    // 115792089237316195423570985008687907853269984665640564039457584007913129639935 - "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF00"
    // 115792089237316195423570985008687907853269984665640564039457584007913129639936 - "000000000000000000000000000000000000000000000000000000000000000001"
    // 115792089237316195423570985008687907853269984665640564039457584007913129639937 - "010000000000000000000000000000000000000000000000000000000000000001"
    // -18446744073709551615 - "FFFFFFFFFFFFFFFF80"
    // -18446744073709551616 - "000000000000000081"
    // -18446744073709551617 - "010000000000000081"
    // -340282366920938463463374607431768211455 - "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF80"
    // -340282366920938463463374607431768211456 - "0000000000000000000000000000000081"
    // -340282366920938463463374607431768211457 - "0100000000000000000000000000000081"
    // -115792089237316195423570985008687907853269984665640564039457584007913129639935 - "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF80"
    // -115792089237316195423570985008687907853269984665640564039457584007913129639936 - "000000000000000000000000000000000000000000000000000000000000000081"
    // -115792089237316195423570985008687907853269984665640564039457584007913129639937 - "010000000000000000000000000000000000000000000000000000000000000081"
}
