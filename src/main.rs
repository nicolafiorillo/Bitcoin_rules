//! Entry point

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

mod base_encoding;
mod btc_ecdsa;
mod field_element;
mod hashing;
mod helper;
mod integer_ex;
mod point;
mod private_key;
mod signature;

fn main() {
    println!("Bitcoin in Rust");
}
