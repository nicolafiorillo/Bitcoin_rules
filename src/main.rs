//! Entry point

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

mod bitcoin;
mod chain;
mod ecdsa;
mod encoding;
mod hashing;
mod keys;
mod lib;
mod transaction;

fn main() {
    env_logger::init();
    println!("BTCR, a Bitcoin node written in Rust");
}
