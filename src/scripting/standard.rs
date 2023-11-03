use crate::std_lib::vector::vect_to_hex_string;

use super::script_lang::ScriptLang;

#[derive(Clone, Debug, PartialEq)]
pub enum StandardType {
    UNKNOWN,
    P2PKH,
    DATA,
}

pub fn p2pkh_script(h160: Vec<u8>) -> ScriptLang {
    let hash_str = vect_to_hex_string(&h160);
    let script_repr = format!("OP_DUP OP_HASH160 {} OP_EQUALVERIFY OP_CHECKSIG", hash_str);

    ScriptLang::from_representation(&script_repr).unwrap()
}

pub fn data_script(data: &[u8]) -> ScriptLang {
    let hash_str = vect_to_hex_string(data);
    let script_repr = format!("OP_RETURN {} ", hash_str);

    ScriptLang::from_representation(&script_repr).unwrap()
}
