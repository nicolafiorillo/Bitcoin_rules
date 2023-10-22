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

    op_to_fn[OP_0] = OpCodeInfo::new("OP_0", op_0);
    op_to_fn[OP_PUSHDATA1] = OpCodeInfo::new("OP_PUSHDATA1", not_implemented);
    op_to_fn[OP_PUSHDATA2] = OpCodeInfo::new("OP_PUSHDATA2", not_implemented);
    op_to_fn[OP_PUSHDATA4] = OpCodeInfo::new("OP_PUSHDATA4", not_implemented);
    op_to_fn[OP_1NEGATE] = OpCodeInfo::new("OP_1NEGATE", op_1negate);
    op_to_fn[OP_RESERVED] = OpCodeInfo::new("OP_RESERVED", reserved);
    op_to_fn[OP_1] = OpCodeInfo::new("OP_1", op_1);
    op_to_fn[OP_2] = OpCodeInfo::new("OP_2", op_2);
    op_to_fn[OP_3] = OpCodeInfo::new("OP_3", op_3);
    op_to_fn[OP_4] = OpCodeInfo::new("OP_4", op_4);
    op_to_fn[OP_5] = OpCodeInfo::new("OP_5", op_5);
    op_to_fn[OP_6] = OpCodeInfo::new("OP_6", op_6);
    op_to_fn[OP_7] = OpCodeInfo::new("OP_7", op_7);
    op_to_fn[OP_8] = OpCodeInfo::new("OP_8", op_8);
    op_to_fn[OP_9] = OpCodeInfo::new("OP_9", op_9);
    op_to_fn[OP_10] = OpCodeInfo::new("OP_10", op_10);
    op_to_fn[OP_11] = OpCodeInfo::new("OP_11", op_11);
    op_to_fn[OP_12] = OpCodeInfo::new("OP_12", op_12);
    op_to_fn[OP_13] = OpCodeInfo::new("OP_13", op_13);
    op_to_fn[OP_14] = OpCodeInfo::new("OP_14", op_14);
    op_to_fn[OP_15] = OpCodeInfo::new("OP_15", op_15);
    op_to_fn[OP_16] = OpCodeInfo::new("OP_16", op_16);
    op_to_fn[OP_NOP] = OpCodeInfo::new("OP_NOP", op_nop);
    op_to_fn[OP_VER] = OpCodeInfo::new("OP_VER", reserved);
    op_to_fn[OP_IF] = OpCodeInfo::new("OP_IF", op_if);
    op_to_fn[OP_NOTIF] = OpCodeInfo::new("OP_NOTIF", op_notif);
    op_to_fn[OP_VERIF] = OpCodeInfo::new("OP_VERIF", reserved);
    op_to_fn[OP_VERNOTIF] = OpCodeInfo::new("OP_VERNOTIF", reserved);
    op_to_fn[OP_ELSE] = OpCodeInfo::new("OP_ELSE", op_else);
    op_to_fn[OP_ENDIF] = OpCodeInfo::new("OP_ENDIF", op_endif);
    op_to_fn[OP_VERIFY] = OpCodeInfo::new("OP_VERIFY", op_verify);
    op_to_fn[OP_RETURN] = OpCodeInfo::new("OP_RETURN", op_return);
    op_to_fn[OP_TOALTSTACK] = OpCodeInfo::new("OP_TOALTSTACK", op_toaltstack);
    op_to_fn[OP_FROMALTSTACK] = OpCodeInfo::new("OP_FROMALTSTACK", op_fromaltstack);
    op_to_fn[OP_2DROP] = OpCodeInfo::new("OP_2DROP", op_2drop);
    op_to_fn[OP_2DUP] = OpCodeInfo::new("OP_2DUP", op_2dup);
    op_to_fn[OP_3DUP] = OpCodeInfo::new("OP_3DUP", op_3dup);
    op_to_fn[OP_2OVER] = OpCodeInfo::new("OP_2OVER", op_2over);
    op_to_fn[OP_2ROT] = OpCodeInfo::new("OP_2ROT", op_2rot);
    op_to_fn[OP_2SWAP] = OpCodeInfo::new("OP_2SWAP", op_2swap);
    op_to_fn[OP_IFDUP] = OpCodeInfo::new("OP_IFDUP", not_implemented);
    op_to_fn[OP_DEPTH] = OpCodeInfo::new("OP_DEPTH", not_implemented);
    op_to_fn[OP_DROP] = OpCodeInfo::new("OP_DROP", op_drop);
    op_to_fn[OP_DUP] = OpCodeInfo::new("OP_DUP", op_dup);
    op_to_fn[OP_NIP] = OpCodeInfo::new("OP_NIP", op_nip);
    op_to_fn[OP_OVER] = OpCodeInfo::new("OP_OVER", not_implemented);
    op_to_fn[OP_PICK] = OpCodeInfo::new("OP_PICK", not_implemented);
    op_to_fn[OP_ROLL] = OpCodeInfo::new("OP_ROLL", not_implemented);
    op_to_fn[OP_ROT] = OpCodeInfo::new("OP_ROT", op_rot);
    op_to_fn[OP_SWAP] = OpCodeInfo::new("OP_SWAP", op_swap);
    op_to_fn[OP_TUCK] = OpCodeInfo::new("OP_TUCK", not_implemented);
    op_to_fn[OP_CAT] = OpCodeInfo::new("OP_CAT", deprecated);
    op_to_fn[OP_SUBSTR] = OpCodeInfo::new("OP_SUBSTR", deprecated);
    op_to_fn[OP_LEFT] = OpCodeInfo::new("OP_LEFT", deprecated);
    op_to_fn[OP_RIGHT] = OpCodeInfo::new("OP_RIGHT", deprecated);
    op_to_fn[OP_SIZE] = OpCodeInfo::new("OP_SIZE", not_implemented);
    op_to_fn[OP_INVERT] = OpCodeInfo::new("OP_INVERT", deprecated);
    op_to_fn[OP_AND] = OpCodeInfo::new("OP_AND", deprecated);
    op_to_fn[OP_OR] = OpCodeInfo::new("OP_OR", deprecated);
    op_to_fn[OP_XOR] = OpCodeInfo::new("OP_XOR", deprecated);
    op_to_fn[OP_EQUAL] = OpCodeInfo::new("OP_EQUAL", op_equal);
    op_to_fn[OP_EQUALVERIFY] = OpCodeInfo::new("OP_EQUALVERIFY", op_equalverify);
    op_to_fn[OP_RESERVED1] = OpCodeInfo::new("OP_RESERVED1", reserved);
    op_to_fn[OP_RESERVED2] = OpCodeInfo::new("OP_RESERVED2", reserved);
    op_to_fn[OP_1ADD] = OpCodeInfo::new("OP_1ADD", not_implemented);
    op_to_fn[OP_1SUB] = OpCodeInfo::new("OP_1SUB", not_implemented);
    op_to_fn[OP_2MUL] = OpCodeInfo::new("OP_2MUL", deprecated);
    op_to_fn[OP_2DIV] = OpCodeInfo::new("OP_2DIV", deprecated);
    op_to_fn[OP_NEGATE] = OpCodeInfo::new("OP_NEGATE", not_implemented);
    op_to_fn[OP_ABS] = OpCodeInfo::new("OP_ABS", not_implemented);
    op_to_fn[OP_NOT] = OpCodeInfo::new("OP_NOT", op_not);
    op_to_fn[OP_0NOTEQUAL] = OpCodeInfo::new("OP_0NOTEQUAL", not_implemented);
    op_to_fn[OP_ADD] = OpCodeInfo::new("OP_ADD", op_add);
    op_to_fn[OP_SUB] = OpCodeInfo::new("OP_SUB", not_implemented);
    op_to_fn[OP_MUL] = OpCodeInfo::new("OP_MUL", deprecated);
    op_to_fn[OP_DIV] = OpCodeInfo::new("OP_DIV", deprecated);
    op_to_fn[OP_MOD] = OpCodeInfo::new("OP_MOD", deprecated);
    op_to_fn[OP_LSHIFT] = OpCodeInfo::new("OP_LSHIFT", deprecated);
    op_to_fn[OP_RSHIFT] = OpCodeInfo::new("OP_RSHIFT", deprecated);
    op_to_fn[OP_BOOLAND] = OpCodeInfo::new("OP_BOOLAND", not_implemented);
    op_to_fn[OP_BOOLOR] = OpCodeInfo::new("OP_BOOLOR", not_implemented);
    op_to_fn[OP_NUMEQUAL] = OpCodeInfo::new("OP_NUMEQUAL", not_implemented);
    op_to_fn[OP_NUMEQUALVERIFY] = OpCodeInfo::new("OP_NUMEQUALVERIFY", not_implemented);
    op_to_fn[OP_NUMNOTEQUAL] = OpCodeInfo::new("OP_NUMNOTEQUAL", not_implemented);
    op_to_fn[OP_LESSTHAN] = OpCodeInfo::new("OP_LESSTHAN", not_implemented);
    op_to_fn[OP_GREATERTHAN] = OpCodeInfo::new("OP_GREATERTHAN", not_implemented);
    op_to_fn[OP_LESSTHANOREQUAL] = OpCodeInfo::new("OP_LESSTHANOREQUAL", not_implemented);
    op_to_fn[OP_GREATERTHANOREQUAL] = OpCodeInfo::new("OP_GREATERTHANOREQUAL", not_implemented);
    op_to_fn[OP_MIN] = OpCodeInfo::new("OP_MIN", not_implemented);
    op_to_fn[OP_MAX] = OpCodeInfo::new("OP_MAX", not_implemented);
    op_to_fn[OP_WITHIN] = OpCodeInfo::new("OP_WITHIN", not_implemented);
    op_to_fn[OP_RIPEMD160] = OpCodeInfo::new("OP_RIPEMD160", not_implemented);
    op_to_fn[OP_SHA1] = OpCodeInfo::new("OP_SHA1", op_sha1);
    op_to_fn[OP_SHA256] = OpCodeInfo::new("OP_SHA256", op_sha256);
    op_to_fn[OP_HASH160] = OpCodeInfo::new("OP_HASH160", op_hash160);
    op_to_fn[OP_HASH256] = OpCodeInfo::new("OP_HASH256", op_hash256);
    op_to_fn[OP_CODESEPARATOR] = OpCodeInfo::new("OP_CODESEPARATOR", not_implemented);
    op_to_fn[OP_CHECKSIG] = OpCodeInfo::new("OP_CHECKSIG", op_checksig);
    op_to_fn[OP_CHECKSIGVERIFY] = OpCodeInfo::new("OP_CHECKSIGVERIFY", not_implemented);
    op_to_fn[OP_CHECKMULTISIG] = OpCodeInfo::new("OP_CHECKMULTISIG", not_implemented);
    op_to_fn[OP_CHECKMULTISIGVERIFY] = OpCodeInfo::new("OP_CHECKMULTISIGVERIFY", not_implemented);
    op_to_fn[OP_NOP1] = OpCodeInfo::new("OP_NOP1", ignored);
    op_to_fn[OP_CHECKLOCKTIMEVERIFY] = OpCodeInfo::new("OP_CHECKLOCKTIMEVERIFY", not_implemented);
    op_to_fn[OP_CHECKSEQUENCEVERIFY] = OpCodeInfo::new("OP_CHECKSEQUENCEVERIFY", not_implemented);
    op_to_fn[OP_NOP4] = OpCodeInfo::new("OP_NOP4", ignored);
    op_to_fn[OP_NOP5] = OpCodeInfo::new("OP_NOP5", ignored);
    op_to_fn[OP_NOP6] = OpCodeInfo::new("OP_NOP6", ignored);
    op_to_fn[OP_NOP7] = OpCodeInfo::new("OP_NOP7", ignored);
    op_to_fn[OP_NOP8] = OpCodeInfo::new("OP_NOP8", ignored);
    op_to_fn[OP_NOP9] = OpCodeInfo::new("OP_NOP9", ignored);
    op_to_fn[OP_NOP10] = OpCodeInfo::new("OP_NOP10", ignored);
    op_to_fn[OP_CHECKSIGADD] = OpCodeInfo::new("OP_CHECKSIGADD", not_implemented);
    op_to_fn[OP_INVALIDOPCODE] = OpCodeInfo::new("OP_INVALIDOPCODE", not_implemented);

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
