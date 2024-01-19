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

struct BlockFixture {
    pub height: u32,
    pub id: String,
    pub block: String,
}

pub static MAINNET: Lazy<HashMap<Integer, Header>> = Lazy::new(|| {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let block_fixture = format!("{}/test/fixtures/blocks.csv", manifest_dir);

    let blocks = read_blocks_from_fixture(&block_fixture);

    let mut h: HashMap<Integer, Header> = HashMap::new();

    for block in blocks {
        let (id, tx) = get_id_to_header(&block.id, &block.block);
        h.insert(id, tx);
    }

    h
});

fn read_blocks_from_fixture(block_fixture: &str) -> Vec<BlockFixture> {
    let content = std::fs::read_to_string(block_fixture).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let mut blocks = Vec::<BlockFixture>::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let h: Vec<&str> = line.split(',').collect();
        let height = h[0].parse::<u32>().unwrap();
        let id = h[1].trim().to_string();
        let block = h[2].trim().to_string();

        blocks.push(BlockFixture { height, id, block });
    }

    blocks
}

pub static TESTNET: Lazy<HashMap<Integer, Header>> = Lazy::new(|| HashMap::new());

pub fn get_header(block_id: &Integer, network: Network) -> StdResult<&Header> {
    let h = match network {
        Network::Testnet => &*TESTNET,
        Network::Mainnet => &*MAINNET,
    };

    let header = h.get(block_id).ok_or("transaction_not_found")?;

    Ok(header)
}
