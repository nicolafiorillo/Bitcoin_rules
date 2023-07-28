//! Entry point

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

use rug::Integer;

use crate::{
    scripting::{opcode::*, operation::Operation, script::Script},
    std_lib::{integer_ex::IntegerEx, vector::string_to_bytes},
};

mod bitcoin;
mod chain;
mod ecdsa;
mod encoding;
mod hashing;
mod keys;
mod scripting;
mod std_lib;
mod transaction;

fn main() {
    env_logger::init();
    println!("BTCR, a Bitcoin node written in Rust");

    let z: Integer = IntegerEx::from_hex_str("7C076FF316692A3D7EB3C3BB0F8B1488CF72E1AFCD929E29307032997A838A3D");
    let pubkey = string_to_bytes("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34");
    let signature = string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601");

    let pubkey_script = Script::from_script_items(vec![Operation::Element(pubkey), Operation::Command(OP_CHECKSIG)]);

    let signature_script = Script::from_script_items(vec![Operation::Element(signature)]);
    let script = Script::combine(pubkey_script, signature_script);

    println!("Script: {:}", script);
    println!("Script evaluated: {:?}", script.evaluate(z));
}
