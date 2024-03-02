use rug::Integer;

use crate::{
    flags::sighash::SigHash,
    keys::key::Key,
    scripting::{script_lang::ScriptLang, token::Token},
    std_lib::std_result::StdResult,
};

use super::{script::Script, tx::Tx};

pub fn generate_input_signature(
    tx: &Tx,
    input_index: usize,
    private_key: &Integer,
    script: Script,
) -> StdResult<Script> {
    let z = tx.hash_signature(input_index, script);

    let key = Key::new(private_key.clone());
    let signature = key.sign(z);
    let der = signature.der();

    let sec = key.public_key_sec();

    let hash_type = [SigHash::All as u8].to_vec(); //TODO parametrize SIGHASH
    let sig = [der, hash_type].concat();

    let signature_script = ScriptLang::from_tokens(vec![Token::Element(sig), Token::Element(sec)]);

    Ok(Script::new_from_script_lang(&signature_script))
}
