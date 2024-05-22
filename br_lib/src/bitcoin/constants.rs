use once_cell::sync::Lazy;
use rug::Integer;

use crate::{
    flags::network_magic::NetworkMagic,
    hashing::hash256::Hash256,
    std_lib::{integer_extended::IntegerExtended, vector::hex_string_to_bytes},
};

pub static MAINNET_GENESIS_BLOCK_ID_STR: &str = "000000000019D6689C085AE165831E934FF763AE46A2A6C172B3F1B60A8CE26F";
pub static MAINNET_GENESIS_BLOCK_ID: Lazy<Integer> = Lazy::new(|| Integer::from_hex_str(MAINNET_GENESIS_BLOCK_ID_STR));
pub static MAINNET_GENESIS_BLOCK_HASH: Lazy<Hash256> = Lazy::new(|| str_to_hash256(MAINNET_GENESIS_BLOCK_ID_STR));

pub static TESTNET_GENESIS_BLOCK_ID_STR: &str = "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943";
pub static TESTNET_GENESIS_BLOCK_ID: Lazy<Integer> = Lazy::new(|| Integer::from_hex_str(TESTNET_GENESIS_BLOCK_ID_STR));
pub static TESTNET_GENESIS_BLOCK_HASH: Lazy<Hash256> = Lazy::new(|| str_to_hash256(TESTNET_GENESIS_BLOCK_ID_STR));

pub fn network_to_environment(network: NetworkMagic) -> Hash256 {
    match network {
        NetworkMagic::Mainnet => MAINNET_GENESIS_BLOCK_HASH.clone(),
        NetworkMagic::Testnet3 => TESTNET_GENESIS_BLOCK_HASH.clone(),
        _ => panic!("unknown_network"),
    }
}

fn str_to_hash256(s: &str) -> Hash256 {
    let mut vbytes = hex_string_to_bytes(s).unwrap();
    vbytes.reverse();

    let bytes: [u8; 32] = vbytes.as_slice().try_into().unwrap();
    Hash256(bytes)
}
