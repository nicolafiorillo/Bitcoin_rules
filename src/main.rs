//! Entry point

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

mod btc_ecdsa;
mod field_element;
mod hash256;
mod helper;
mod integer_ex;
mod point;
mod private_key;
mod signature;

fn main() {
    println!("Bitcoin in Rust");
}
