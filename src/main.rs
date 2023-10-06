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

use crate::{chain::tx::get_transaction, flags::network::Network, std_lib::integer_extended::IntegerExtended};

mod bitcoin;
mod chain;
mod ecdsa;
mod flags;
mod hashing;
mod keys;
mod scripting;
mod std_lib;
mod transaction;

fn main() {
    env_logger::init();
    println!("Bitcoin_rules!");
    println!("A Bitcoin node written in Rust for educational purposes.");

    let satoshi_transaction_id: Integer =
        Integer::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
    // let satoshi_transaction_id: Integer =
    //     Integer::from_hex_str("0437cd7f8525ceed2324359c2d0ba26006d92d856a9c20fa0241106ee5a597c9");

    let satoshi_transaction = get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

    println!("");
    println!("Satoshi transaction");
    println!("{:}", satoshi_transaction);
}
