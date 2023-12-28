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

    // genesis block
    let (id, tx) = get_id_to_header("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f", "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c");
    h.insert(id, tx);

    // second block
    let (id, tx) = get_id_to_header("00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048", "010000006fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000982051fd1e4ba744bbbe680e1fee14677ba1a3c3540bf7b1cdb606e857233e0e61bc6649ffff001d01e36299");
    h.insert(id, tx);

    // block with BIP34 (version 2)
    let (id, tx) = get_id_to_header("00000000000000d0dfd4c9d588d325dce4f32c1b31b7c0064cba7025a9b9adcc", "02000000b64ea7b7615283a01d9d6019f5bfbd694d2a534ca87a7d07aa0100000000000045ff55adc8d6bc183e9abfa7bdfef3e7b942d786dcd116e776ead8238451a238ac204f516e81021ad0f8cd01");
    h.insert(id, tx);

    // block with BIP66 (version 3)
    let (id, tx) = get_id_to_header("00000000000000001121383bdf780af5290a88dcba88ad38c6be5369f4b6023b", "030000000d6ef1e411382c00d8fd5b9d0f1acb1748e4d3de33e4320500000000000000009437e2c5a160c8e40050354ba7eb0ad0df925da3c277d0e2ae7887cab9cd4a8e137f74558b1a1718d1d2bcad");
    h.insert(id, tx);

    // block with BIP65 (version 4)
    let (id, tx) = get_id_to_header("0000000000000000098702b1f6f35cc002871e012dbdb383978d4d5ffc8b6617", "040000001cc480a37c0c176d109c45dbdb1289e0ebe83415c5218c0d000000000000000067feb9733954f8a3cb49e9fd0f290952ead9693cae85781e83298bf5dacb6bfd01d64f5689b21018078ee0f5");
    h.insert(id, tx);

    // block with BIP9 (version >= 4)
    let (id, tx) = get_id_to_header("000000000000000006e35d6675fb0fec767a5f3b346261a5160f6e2a8d258070", "00000030af7e7389ca428b05d8902fcdc148e70974524d39cb56bc0100000000000000007ce0cd0c9c648d1b585d29b9ab23ebc987619d43925b3c768d7cb4bc097cfb821441c05614a107187aef1ee1");
    h.insert(id, tx);

    // block with BIP91 and BIP141 (version >= 4)
    let (id, tx) = get_id_to_header("0000000000000000015411ca4b35f7b48ecab015b14de5627b647e262ba0ec40", "12000020734713ff00fef0cd9fedd9545fa9316cacc5fd922c55220000000000000000008de88ecac287705b0548490fa5469d1e8ddaad7f0078a0fa73106b32f9a3138a272a7459dc5d01187e183e6d");
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
