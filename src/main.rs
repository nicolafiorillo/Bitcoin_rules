//! Entry point

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

pub mod bitcoin;
pub mod chain;
pub mod ecdsa;
mod encoding;
mod hashing;
mod helper;
mod integer_ex;
mod point;
mod private_key;
mod signature;
pub mod transaction;

fn main() {
    env_logger::init();
    println!("BTCR, a Bitcoin node written in Rust");
}
