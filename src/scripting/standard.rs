use once_cell::sync::Lazy;
use regex::Regex;

use crate::std_lib::vector::vect_to_hex_string;

use super::script_lang::ScriptLang;

#[derive(Clone, Debug, PartialEq)]
pub enum StandardType {
    Unknown,
    P2pk,
    P2pkh,
    Data,
}

static P2PK_SCRIPT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-fA-F]+ OP_CHECKSIG$").unwrap());
static P2PKH_SCRIPT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^OP_DUP OP_HASH160 [0-9a-fA-F]{40} OP_EQUALVERIFY OP_CHECKSIG$").unwrap());
static DATA_SCRIPT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^OP_RETURN [0-9a-fA-F]+$").unwrap());

/*
   TODO: ugly and not very efficient implementation of the standard type of a script.
*/
pub fn standard_type(script: &ScriptLang) -> StandardType {
    let script_repr = script.representation();

    if DATA_SCRIPT_REGEX.is_match(&script_repr) {
        return StandardType::Data;
    }

    if P2PKH_SCRIPT_REGEX.is_match(&script_repr) {
        return StandardType::P2pkh;
    }

    if P2PK_SCRIPT_REGEX.is_match(&script_repr) {
        return StandardType::P2pk;
    }

    StandardType::Unknown
}

// ANCHOR: p2pk_script
/*
   Despite being deprecated, the P2PK script is also implemented here because it was used in the first Bitcoin transactions.
   The P2PK script contemplates the presence of the recipient's public key in the `ScriptPubKey` and, if spent, the sender's signature in the `ScriptSig`.
   This scenario could allow, thanks to a specific [implementation of a Shor algorithm](https://eprint.iacr.org/2016/1128.pdf) capable of solving the [Discrete Logarithm problem](https://sefiks.com/2018/02/28/attacking-elliptic-curve-discrete-logarithm-problem/) in polynomial time, to determine the private key corresponding to the public key. However, we are talking about the need for quantum computers capable of performing an enormous number of operations in short time.
   As already indicated, this type of script has been deprecated in favour of the P2PKH script that uses its `hash160` instead of the public key in plain text.
   The application of the hash means that the public key is not visible, adding an additional layer of security and considerably mitigating the risk of it being compromised and the corresponding private key being calculated.

   Moreover the P2PKH script is quite long (33 bytes as compressed SEC format and 65 bytes as uncompressed SEC format), doubling the size when encoded in hexadecimal form.
   This is a problem for the chain because it increases the size of the transactions and therefore the cost of the fees.
*/
// ANCHOR_END: p2pk_script
pub fn p2pk_script(address: Vec<u8>) -> ScriptLang {
    let addr_str = vect_to_hex_string(&address);
    let script_repr = format!("{addr_str} OP_CHECKSIG");

    ScriptLang::from_representation(&script_repr).unwrap()
}

pub fn p2pkh_script(h160: Vec<u8>) -> ScriptLang {
    let hash_str = vect_to_hex_string(&h160);
    let script_repr = format!("OP_DUP OP_HASH160 {hash_str} OP_EQUALVERIFY OP_CHECKSIG");

    ScriptLang::from_representation(&script_repr).unwrap()
}

pub fn data_script(data: &[u8]) -> ScriptLang {
    let data_str = vect_to_hex_string(data);
    let script_repr = format!("OP_RETURN {data_str}");

    ScriptLang::from_representation(&script_repr).unwrap()
}
