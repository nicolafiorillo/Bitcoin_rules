// For now we will mock transaction fetching from node.

use once_cell::sync::Lazy;
use rug::Integer;
use std::collections::HashMap;

use crate::{
    block::header::Header,
    flags::network::Network,
    std_lib::{integer_extended::IntegerExtended, std_result::StdResult, vector::string_to_bytes},
};

fn get_id_to_header(id: &str, header: &str) -> (Integer, Header) {
    let s = string_to_bytes(header).unwrap();
    let h = Header::deserialize(&s).unwrap();
    let id = Integer::from_hex_str(id);

    (id, h)
}

pub static MAINNET: Lazy<HashMap<Integer, Header>> = Lazy::new(|| {
    let mut h: HashMap<Integer, Header> = HashMap::new();

    let (id, tx) = get_id_to_header("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f", "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c");
    h.insert(id, tx);

    let (id, tx) = get_id_to_header("00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048", "010000006fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000982051fd1e4ba744bbbe680e1fee14677ba1a3c3540bf7b1cdb606e857233e0e61bc6649ffff001d01e36299");
    h.insert(id, tx);

    h
});

pub static TESTNET: Lazy<HashMap<Integer, Header>> = Lazy::new(|| HashMap::new());

pub fn get_header(block_id: &Integer, network: Network) -> StdResult<&Header> {
    let h = match network {
        Network::Testnet => &*TESTNET,
        Network::Mainnet => &*MAINNET,
    };

    let header = h.get(block_id).ok_or("transaction_not_found")?;

    Ok(header)
}
