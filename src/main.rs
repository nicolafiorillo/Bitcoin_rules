/*!
Entry point
*/

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

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
}

fn version() -> &'static str {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("unknown")
}
