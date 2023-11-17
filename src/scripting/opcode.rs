use once_cell::sync::Lazy;
use std::ops::Range;

use super::{
    context::{Context, ContextError},
    opcode_fn::*,
};

// TODO: why exit with bool?
type CommandExec = fn(&mut Context) -> Result<bool, ContextError>;

#[derive(Debug, Copy, Clone)]
pub struct OpCodeInfo {
    pub name: &'static str,
    pub exec: CommandExec,
}

impl OpCodeInfo {
    pub fn new(name: &'static str, exec: CommandExec) -> Self {
        Self { name, exec }
    }
}

pub type OpCode = usize;

pub const OP_ELEMENTS_RANGE: Range<OpCode> = OP_0..OP_PUSHDATA1;

pub const OP_0: OpCode = 0x00;
// From 0x01 to 0x4B: OP_PUSHBYTES_XX
pub const OP_PUSHDATA1: OpCode = 0x4C;
pub const OP_PUSHDATA2: OpCode = 0x4D;
pub const OP_PUSHDATA4: OpCode = 0x4E;
pub const OP_1NEGATE: OpCode = 0x4F;
pub const OP_RESERVED: OpCode = 0x50;
pub const OP_1: OpCode = 0x51;
pub const OP_2: OpCode = 0x52;
pub const OP_3: OpCode = 0x53;
pub const OP_4: OpCode = 0x54;
pub const OP_5: OpCode = 0x55;
pub const OP_6: OpCode = 0x56;
pub const OP_7: OpCode = 0x57;
pub const OP_8: OpCode = 0x58;
pub const OP_9: OpCode = 0x59;
pub const OP_10: OpCode = 0x5A;
pub const OP_11: OpCode = 0x5B;
pub const OP_12: OpCode = 0x5C;
pub const OP_13: OpCode = 0x5D;
pub const OP_14: OpCode = 0x5E;
pub const OP_15: OpCode = 0x5F;
pub const OP_16: OpCode = 0x60;
pub const OP_NOP: OpCode = 0x61;
pub const OP_VER: OpCode = 0x62;
pub const OP_IF: OpCode = 0x63;
pub const OP_NOTIF: OpCode = 0x64;
pub const OP_VERIF: OpCode = 0x65;
pub const OP_VERNOTIF: OpCode = 0x66;
pub const OP_ELSE: OpCode = 0x67;
pub const OP_ENDIF: OpCode = 0x68;
pub const OP_VERIFY: OpCode = 0x69;
pub const OP_RETURN: OpCode = 0x6A;
pub const OP_TOALTSTACK: OpCode = 0x6B;
pub const OP_FROMALTSTACK: OpCode = 0x6C;
pub const OP_2DROP: OpCode = 0x6D;
pub const OP_2DUP: OpCode = 0x6E;
pub const OP_3DUP: OpCode = 0x6F;
pub const OP_2OVER: OpCode = 0x70;
pub const OP_2ROT: OpCode = 0x71;
pub const OP_2SWAP: OpCode = 0x72;
pub const OP_IFDUP: OpCode = 0x73;
pub const OP_DEPTH: OpCode = 0x74;
pub const OP_DROP: OpCode = 0x75;
pub const OP_DUP: OpCode = 0x76;
pub const OP_NIP: OpCode = 0x77;
pub const OP_OVER: OpCode = 0x78;
pub const OP_PICK: OpCode = 0x79;
pub const OP_ROLL: OpCode = 0x7A;
pub const OP_ROT: OpCode = 0x7B;
pub const OP_SWAP: OpCode = 0x7C;
pub const OP_TUCK: OpCode = 0x7D;
pub const OP_CAT: OpCode = 0x7E;
pub const OP_SUBSTR: OpCode = 0x7F;
pub const OP_LEFT: OpCode = 0x80;
pub const OP_RIGHT: OpCode = 0x81;
pub const OP_SIZE: OpCode = 0x82;
pub const OP_INVERT: OpCode = 0x83;
pub const OP_AND: OpCode = 0x84;
pub const OP_OR: OpCode = 0x85;
pub const OP_XOR: OpCode = 0x86;
pub const OP_EQUAL: OpCode = 0x87;
pub const OP_EQUALVERIFY: OpCode = 0x88;
pub const OP_RESERVED1: OpCode = 0x89;
pub const OP_RESERVED2: OpCode = 0x8A;
pub const OP_1ADD: OpCode = 0x8B;
pub const OP_1SUB: OpCode = 0x8C;
pub const OP_2MUL: OpCode = 0x8D;
pub const OP_2DIV: OpCode = 0x8E;
pub const OP_NEGATE: OpCode = 0x8F;
pub const OP_ABS: OpCode = 0x90;
pub const OP_NOT: OpCode = 0x91;
pub const OP_0NOTEQUAL: OpCode = 0x92;
pub const OP_ADD: OpCode = 0x93;
pub const OP_SUB: OpCode = 0x94;
pub const OP_MUL: OpCode = 0x95;
pub const OP_DIV: OpCode = 0x96;
pub const OP_MOD: OpCode = 0x97;
pub const OP_LSHIFT: OpCode = 0x98;
pub const OP_RSHIFT: OpCode = 0x99;
pub const OP_BOOLAND: OpCode = 0x9A;
pub const OP_BOOLOR: OpCode = 0x9B;
pub const OP_NUMEQUAL: OpCode = 0x9C;
pub const OP_NUMEQUALVERIFY: OpCode = 0x9D;
pub const OP_NUMNOTEQUAL: OpCode = 0x9E;
pub const OP_LESSTHAN: OpCode = 0x9F;
pub const OP_GREATERTHAN: OpCode = 0xA0;
pub const OP_LESSTHANOREQUAL: OpCode = 0xA1;
pub const OP_GREATERTHANOREQUAL: OpCode = 0xA2;
pub const OP_MIN: OpCode = 0xA3;
pub const OP_MAX: OpCode = 0xA4;
pub const OP_WITHIN: OpCode = 0xA5;
pub const OP_RIPEMD160: OpCode = 0xA6;
pub const OP_SHA1: OpCode = 0xA7;
pub const OP_SHA256: OpCode = 0xA8;
pub const OP_HASH160: OpCode = 0xA9;
pub const OP_HASH256: OpCode = 0xAA;
pub const OP_CODESEPARATOR: OpCode = 0xAB;
pub const OP_CHECKSIG: OpCode = 0xAC;
pub const OP_CHECKSIGVERIFY: OpCode = 0xAD;
pub const OP_CHECKMULTISIG: OpCode = 0xAE;
pub const OP_CHECKMULTISIGVERIFY: OpCode = 0xAF;
pub const OP_NOP1: OpCode = 0xB0;
pub const OP_CHECKLOCKTIMEVERIFY: OpCode = 0xB1;
pub const OP_CHECKSEQUENCEVERIFY: OpCode = 0xB2;
pub const OP_NOP4: OpCode = 0xB3;
pub const OP_NOP5: OpCode = 0xB4;
pub const OP_NOP6: OpCode = 0xB5;
pub const OP_NOP7: OpCode = 0xB6;
pub const OP_NOP8: OpCode = 0xB7;
pub const OP_NOP9: OpCode = 0xB8;
pub const OP_NOP10: OpCode = 0xB9;
pub const OP_CHECKSIGADD: OpCode = 0xBA;
pub const OP_INVALIDOPCODE: OpCode = 0xFF;

pub const OPS_LENGTH: usize = 256;

pub static OP_TO_FN: Lazy<[OpCodeInfo; OPS_LENGTH]> = Lazy::new(|| {
    let mut op_to_fn: [OpCodeInfo; OPS_LENGTH] = [OpCodeInfo::new("", not_implemented); OPS_LENGTH];

    macro_rules! op2fn {
        ($OP:ident, $FN: ident) => {
            op_to_fn[$OP] = OpCodeInfo::new(stringify!($OP), $FN);
        };
    }

    op2fn!(OP_0, op_0);
    op2fn!(OP_PUSHDATA1, not_implemented);
    op2fn!(OP_PUSHDATA2, not_implemented);
    op2fn!(OP_PUSHDATA4, not_implemented);
    op2fn!(OP_1NEGATE, op_1negate);
    op2fn!(OP_RESERVED, reserved);
    op2fn!(OP_1, op_1);
    op2fn!(OP_2, op_2);
    op2fn!(OP_3, op_3);
    op2fn!(OP_4, op_4);
    op2fn!(OP_5, op_5);
    op2fn!(OP_6, op_6);
    op2fn!(OP_7, op_7);
    op2fn!(OP_8, op_8);
    op2fn!(OP_9, op_9);
    op2fn!(OP_10, op_10);
    op2fn!(OP_11, op_11);
    op2fn!(OP_12, op_12);
    op2fn!(OP_13, op_13);
    op2fn!(OP_14, op_14);
    op2fn!(OP_15, op_15);
    op2fn!(OP_16, op_16);
    op2fn!(OP_NOP, op_nop);
    op2fn!(OP_VER, reserved);
    op2fn!(OP_IF, op_if);
    op2fn!(OP_NOTIF, op_notif);
    op2fn!(OP_VERIF, reserved);
    op2fn!(OP_VERNOTIF, reserved);
    op2fn!(OP_ELSE, op_else);
    op2fn!(OP_ENDIF, op_endif);
    op2fn!(OP_VERIFY, op_verify);
    op2fn!(OP_RETURN, op_return);
    op2fn!(OP_TOALTSTACK, op_toaltstack);
    op2fn!(OP_FROMALTSTACK, op_fromaltstack);
    op2fn!(OP_2DROP, op_2drop);
    op2fn!(OP_2DUP, op_2dup);
    op2fn!(OP_3DUP, op_3dup);
    op2fn!(OP_2OVER, op_2over);
    op2fn!(OP_2ROT, op_2rot);
    op2fn!(OP_2SWAP, op_2swap);
    op2fn!(OP_IFDUP, op_ifdup);
    op2fn!(OP_DEPTH, op_depth);
    op2fn!(OP_DROP, op_drop);
    op2fn!(OP_DUP, op_dup);
    op2fn!(OP_NIP, op_nip);
    op2fn!(OP_OVER, op_over);
    op2fn!(OP_PICK, op_pick);
    op2fn!(OP_ROLL, op_roll);
    op2fn!(OP_ROT, op_rot);
    op2fn!(OP_SWAP, op_swap);
    op2fn!(OP_TUCK, op_tuck);
    op2fn!(OP_CAT, deprecated);
    op2fn!(OP_SUBSTR, deprecated);
    op2fn!(OP_LEFT, deprecated);
    op2fn!(OP_RIGHT, deprecated);
    op2fn!(OP_SIZE, op_size);
    op2fn!(OP_INVERT, deprecated);
    op2fn!(OP_AND, deprecated);
    op2fn!(OP_OR, deprecated);
    op2fn!(OP_XOR, deprecated);
    op2fn!(OP_EQUAL, op_equal);
    op2fn!(OP_EQUALVERIFY, op_equalverify);
    op2fn!(OP_RESERVED1, reserved);
    op2fn!(OP_RESERVED2, reserved);
    op2fn!(OP_1ADD, op_1add);
    op2fn!(OP_1SUB, op_1sub);
    op2fn!(OP_2MUL, deprecated);
    op2fn!(OP_2DIV, deprecated);
    op2fn!(OP_NEGATE, op_negate);
    op2fn!(OP_ABS, op_abs);
    op2fn!(OP_NOT, op_not);
    op2fn!(OP_0NOTEQUAL, op_0notequal);
    op2fn!(OP_ADD, op_add);
    op2fn!(OP_SUB, op_sub);
    op2fn!(OP_MUL, deprecated);
    op2fn!(OP_DIV, deprecated);
    op2fn!(OP_MOD, deprecated);
    op2fn!(OP_LSHIFT, deprecated);
    op2fn!(OP_RSHIFT, deprecated);
    op2fn!(OP_BOOLAND, op_booland);
    op2fn!(OP_BOOLOR, op_boolor);
    op2fn!(OP_NUMEQUAL, op_numequal);
    op2fn!(OP_NUMEQUALVERIFY, op_numequalverify);
    op2fn!(OP_NUMNOTEQUAL, op_numnotequal);
    op2fn!(OP_LESSTHAN, not_implemented);
    op2fn!(OP_GREATERTHAN, not_implemented);
    op2fn!(OP_LESSTHANOREQUAL, not_implemented);
    op2fn!(OP_GREATERTHANOREQUAL, not_implemented);
    op2fn!(OP_MIN, not_implemented);
    op2fn!(OP_MAX, not_implemented);
    op2fn!(OP_WITHIN, not_implemented);
    op2fn!(OP_RIPEMD160, not_implemented);
    op2fn!(OP_SHA1, op_sha1);
    op2fn!(OP_SHA256, op_sha256);
    op2fn!(OP_HASH160, op_hash160);
    op2fn!(OP_HASH256, op_hash256);
    op2fn!(OP_CODESEPARATOR, not_implemented);
    op2fn!(OP_CHECKSIG, op_checksig);
    op2fn!(OP_CHECKSIGVERIFY, not_implemented);
    op2fn!(OP_CHECKMULTISIG, op_checkmultisig);
    op2fn!(OP_CHECKMULTISIGVERIFY, not_implemented);
    op2fn!(OP_NOP1, ignored);
    op2fn!(OP_CHECKLOCKTIMEVERIFY, not_implemented);
    op2fn!(OP_CHECKSEQUENCEVERIFY, not_implemented);
    op2fn!(OP_NOP4, ignored);
    op2fn!(OP_NOP5, ignored);
    op2fn!(OP_NOP6, ignored);
    op2fn!(OP_NOP7, ignored);
    op2fn!(OP_NOP8, ignored);
    op2fn!(OP_NOP9, ignored);
    op2fn!(OP_NOP10, ignored);
    op2fn!(OP_CHECKSIGADD, not_implemented);
    op2fn!(OP_INVALIDOPCODE, not_implemented);

    // implement deprecated op codes for testing only
    // https://github.com/bitcoin/bitcoin/blob/d096743150fd35578b7ed71ef6bced2341927d43/src/script/interpreter.cpp#L456
    #[cfg(test)]
    fn implement_deprecated(op_to_fn: &mut [OpCodeInfo; OPS_LENGTH]) {
        op_to_fn[OP_MUL] = OpCodeInfo::new("OP_MUL", op_mul);
    }

    #[cfg(not(test))]
    fn implement_deprecated(_op_to_fn: &mut [OpCodeInfo; OPS_LENGTH]) {}

    implement_deprecated(&mut op_to_fn);

    op_to_fn
});
