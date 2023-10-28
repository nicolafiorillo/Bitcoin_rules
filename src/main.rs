/*!
Entry point
*/

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

use rug::Integer;

use crate::{flags::network::Network, std_lib::integer_extended::IntegerExtended, validate::tx};

mod bitcoin;
mod chain;
mod ecdsa;
mod flags;
mod hashing;
mod keys;
mod scripting;
mod std_lib;
mod transaction;
mod validate;
mod wallet;

fn main() {
    env_logger::init();
    println!("Bitcoin_rules! (ver. {})", version());
    println!("A Bitcoin node written in Rust for educational purposes.");
    println!();
    println!("This is a work in progress: please do not use it in production.");

    let tx_id: Integer = Integer::from_hex_str("c9a7d3bd4c39b43d410fc55e8a586ccd4d690086ffb070a69eea4b5612c44c4d");

    let tx = chain::tx::get_transaction(&tx_id, Network::Mainnet).unwrap();

    println!("{:}", tx);
    println!(" is valid: {:?}", tx::validate(tx));
}

fn version() -> &'static str {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("unknown")
}
