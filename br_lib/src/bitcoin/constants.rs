use once_cell::sync::Lazy;
use rug::Integer;

use crate::{hashing::hash256::Hash256, std_lib::integer_extended::IntegerExtended};

pub static GENESIS_BLOCK_ID_STR: &str = "000000000019D6689C085AE165831E934FF763AE46A2A6C172B3F1B60A8CE26F";
pub static GENESIS_BLOCK_ID: Lazy<Integer> = Lazy::new(|| Integer::from_hex_str(GENESIS_BLOCK_ID_STR));
pub static GENESIS_BLOCK_ID_HASH256: Hash256 = Hash256([
    111, 226, 140, 10, 182, 241, 179, 114, 193, 166, 162, 70, 174, 99, 247, 79, 147, 30, 131, 101, 225, 90, 8, 156,
    104, 214, 25, 0, 0, 0, 0, 0,
]);
