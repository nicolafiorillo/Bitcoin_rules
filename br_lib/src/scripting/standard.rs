use once_cell::sync::Lazy;
use regex::Regex;

use crate::std_lib::vector::bytes_to_string;

use super::script_lang::ScriptLang;

#[derive(Clone, Debug, PartialEq)]
pub enum StandardType {
    Unknown,
    P2pk,
    P2pkh,
    Data,
    P2ms,
}

// TODO: modify regex to match addresses length
static P2PK_SCRIPT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-fA-F]+ OP_CHECKSIG$").unwrap());
static P2PKH_SCRIPT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^OP_DUP OP_HASH160 [0-9a-fA-F]{40} OP_EQUALVERIFY OP_CHECKSIG$").unwrap());
static DATA_SCRIPT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^OP_RETURN [0-9a-fA-F]+$").unwrap());
static P2MS_SCRIPT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"OP_CHECKMULTISIG$").unwrap());

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

    if P2MS_SCRIPT_REGEX.is_match(&script_repr) {
        return StandardType::P2ms;
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
pub fn p2pk_script(address: &[u8]) -> ScriptLang {
    let addr_str = bytes_to_string(address);
    let script_repr = format!("{addr_str} OP_CHECKSIG");

    ScriptLang::from_representation(&script_repr).unwrap()
}

pub fn p2pkh_script(h160: &[u8]) -> ScriptLang {
    let hash_str = bytes_to_string(h160);
    let script_repr = format!("OP_DUP OP_HASH160 {hash_str} OP_EQUALVERIFY OP_CHECKSIG");

    ScriptLang::from_representation(&script_repr).unwrap()
}

pub fn data_script(data: &[u8]) -> ScriptLang {
    let data_str = bytes_to_string(data);
    let script_repr = format!("OP_RETURN {data_str}");

    ScriptLang::from_representation(&script_repr).unwrap()
}

pub fn p2ms_script(m: usize, pub_keys: &[&[u8]]) -> ScriptLang {
    let mut keys = String::new();

    for pub_key in pub_keys {
        let pub_key_str = bytes_to_string(pub_key);
        keys.push_str(&pub_key_str);

        keys.push(' ');
    }
    let keys_str = keys.trim_end();

    let n = pub_keys.len();

    let script_repr = format!("OP_{m} {keys_str} OP_{n} OP_CHECKMULTISIG");

    ScriptLang::from_representation(&script_repr).unwrap()
}

#[cfg(test)]
mod standard_test {
    use super::*;

    #[test]
    fn test_p2pk_standard_type() {
        let script = p2pk_script(&vec![0x02, 0x00, 0x00, 0x00]);
        assert_eq!(script.representation(), "02000000 OP_CHECKSIG");

        let script_type = standard_type(&script);
        assert_eq!(script_type, StandardType::P2pk)
    }

    #[test]
    fn test_p2pkh_standard_type() {
        let script = p2pkh_script(&vec![
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xAA, 0xBB, 0xCC,
            0xDD, 0xEE,
        ]);
        assert_eq!(
            script.representation(),
            "OP_DUP OP_HASH160 AABBCCDDEEAABBCCDDEEAABBCCDDEEAABBCCDDEE OP_EQUALVERIFY OP_CHECKSIG"
        );

        let script_type = standard_type(&script);
        assert_eq!(script_type, StandardType::P2pkh)
    }

    #[test]
    fn test_data_standard_type() {
        let script = data_script(&vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
        assert_eq!(script.representation(), "OP_RETURN AABBCCDDEE");

        let script_type = standard_type(&script);
        assert_eq!(script_type, StandardType::Data)
    }

    #[test]
    fn test_p2ms_standard_type() {
        let script = p2ms_script(1, &vec![vec![0xAA, 0xBB].as_slice(), vec![0xCC, 0xDD].as_slice()]);
        assert_eq!(script.representation(), "OP_1 AABB CCDD OP_2 OP_CHECKMULTISIG");

        let script_type = standard_type(&script);
        assert_eq!(script_type, StandardType::P2ms)
    }
}
