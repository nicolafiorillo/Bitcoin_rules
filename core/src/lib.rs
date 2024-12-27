/*!
Entry point
*/

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

pub mod bitcoin;
pub mod block;
pub mod chain;
pub mod cli;
pub mod ecdsa;
pub mod flags;
pub mod hashing;
pub mod keys;
pub mod merkle;
pub mod network;
pub mod scripting;
pub mod std_lib;
pub mod transaction;
pub mod validate;
pub mod wallet;
