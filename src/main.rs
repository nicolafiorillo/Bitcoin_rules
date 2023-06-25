//! Entry point

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

mod btc_ecdsa;
mod encoding;
mod field_element;
mod hashing;
mod helper;
mod integer_ex;
mod point;
mod private_key;
mod script_pub_key;
mod script_sig;
mod signature;
mod tx;
mod varint;

fn main() {
    env_logger::init();
    println!("Bitcoin in Rust");
}
