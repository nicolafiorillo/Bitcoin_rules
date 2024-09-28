// For now we will mock transaction fetching from node.

use once_cell::sync::Lazy;
use rug::Integer;
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::{
    block::header::Header,
    flags::network::Network,
    std_lib::{
        fixture::load_fixture_file, integer_extended::IntegerExtended, std_result::StdResult,
        vector::hex_string_to_bytes,
    },
};

fn get_id_to_header(id: &str, header: &str) -> (Integer, Header) {
    let s = hex_string_to_bytes(header).unwrap();
    let h = Header::deserialize(&s).unwrap();
    let id = Integer::from_hex_str(id);

    (id, h)
}

struct BlockFixture {
    pub height: u32,
    pub id: String,
    pub block: String,
}

struct NetworkFixture {
    pub height_to_id: HeightToId,
    pub id_to_header: IdToHeader,
}

type HeightToId = HashMap<u32, Integer>;
type IdToHeader = HashMap<Integer, Header>;

static MAINNET: Lazy<NetworkFixture> = Lazy::new(|| {
    let block_fixture = load_fixture_file("blocks.csv");
    load_block_from_file(block_fixture)
});

fn load_block_from_file(block_fixture: String) -> NetworkFixture {
    let blocks = read_blocks_from_fixture(&block_fixture);

    let mut id_to_header: IdToHeader = HashMap::with_capacity(blocks.len());
    let mut height_to_id: HeightToId = HashMap::with_capacity(blocks.len());

    for block in blocks {
        let (id, tx) = get_id_to_header(&block.id, &block.block);
        id_to_header.insert(id.clone(), tx);
        height_to_id.insert(block.height, id);
    }

    NetworkFixture {
        height_to_id,
        id_to_header,
    }
}

fn read_blocks_from_fixture(fixture: &str) -> Vec<BlockFixture> {
    let file: File = std::fs::File::open(fixture).unwrap();
    let reader = BufReader::new(file);

    let mut blocks = Vec::<BlockFixture>::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();

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

static TESTNET: Lazy<NetworkFixture> = Lazy::new(|| {
    let block_fixture = load_fixture_file("test_blocks.csv");
    load_block_from_file(block_fixture)
});

pub fn get_header_by_id(block_id: &Integer, network: Network) -> StdResult<&Header> {
    let h = match network {
        Network::Testnet => &(TESTNET.id_to_header),
        Network::Mainnet => &(MAINNET.id_to_header),
    };

    let header = h.get(block_id).ok_or("block_not_found")?;

    Ok(header)
}

pub fn get_header_by_height(block_height: &u32, network: Network) -> StdResult<&Header> {
    let h = match network {
        Network::Testnet => &(*TESTNET),
        Network::Mainnet => &(*MAINNET),
    };

    let id = h
        .height_to_id
        .get(block_height)
        .ok_or(format!("block_not_found_{block_height}"))?;
    let header = h
        .id_to_header
        .get(id)
        .ok_or(format!("block_not_found_{block_height}"))?;

    Ok(header)
}
