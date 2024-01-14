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
    let headers: Vec<(u32, &str, &str)> = vec![(
        0, // genesis block
        "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
        "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c"
    ),
    (
        1, // second block
        "00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048",
        "010000006fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000982051fd1e4ba744bbbe680e1fee14677ba1a3c3540bf7b1cdb606e857233e0e61bc6649ffff001d01e36299"
    ),
    (
        2015,
        "00000000693067b0e6b440bc51450b9f3850561b07f6d3c021c54fbd6abb9763",
        "01000000e25509cde707c3d02a693e4fe9e7cdd57c38a0d2c8d6341f20dae84b000000000c113df1185e162ee92d031fe21d1400ff7d705a3e9b9c860eea855313cd8ca26c087f49ffff001d30b73231"
    ),
    (
        30240, // difficulty 1
        "000000000fa8bfa0f0dd32f956b874b2c7f1772c5fbedcb1b35e03335c7fb0a8",
        "01000000e6bf7fd7f7790a63786faa878d0dc7fd8f2ff365732e45862c66075100000000700d342f65c7b6834dffb615358a1897016f0448913372190cbe3d27a4b53355b1512b4bffff001dbfb02519"
    ),
    (
        32255, // difficulty 1
        "00000000984f962134a7291e3693075ae03e521f0ee33378ec30a334d860034b",
        "0100000049c1daab3b6536ff1b2633c3a316a6e06ec287676cdeec4ca7baae6b00000000ac10b36b8f354b3353207de15940a5edbc05bb8364af75b4b5409e7823f2b48923ec3a4bffff001dbd5fa412"
    ),
    (
        32256, // difficulty 1.182899534312841
        "000000004f2886a170adb7204cb0c7a824217dd24d11a74423d564c4e0904967",
        "010000004b0360d834a330ec7833e30e1f523ee05a0793361e29a73421964f980000000027b64a020af294e903feed93768705336a20090612a043f47af462a2f5e5b564f8ee3a4b6ad8001dd3a43707"
    ),
    (
        34271, // difficulty 1.182899534312841
        "000000005e36047e39452a7beaaa6721048ac408a3e75bb60a8b0008713653ce",
        "01000000405b203f92ee8012fffad48424cd52827504c06ca717cb7cd7aa69bc00000000a02521744962eccce06cb7187d2451f0dee624a291da2bf9c12ce786d92fa673b2a94b4b6ad8001dc0a41815"
    ),
    (
        34272,
        "000000002732d387256b57cabdcb17767e3d30e220ea73f844b1907c0b5919ea",
        "01000000ce53367108008b0ab65be7a308c48a042167aaea7b2a45397e04365e0000000040fbd504d61cb1a110826bca68c81d41736908dcbdb3f65154bfaf9c92cf50d9c5aa4b4b28c4001dfe665c05"
    ),
    (
        36287, // difficulty 1.305062131591525
        "00000000128d789579ffbec00203a371cbb39cee27df35d951fd66e62ed59258",
        "01000000dbdcf04be80a4e02b4f45d33c8e912dd3b25637955c34371160da54a000000009f05dc6ae211ec0b84740b4b80ee9c8cd807dd1c081aff0ea79aa7625c0b073421965d4b28c4001d16492293"
    ),
    (
        227836, // block with BIP34 (version 2)
        "00000000000000d0dfd4c9d588d325dce4f32c1b31b7c0064cba7025a9b9adcc",
        "02000000b64ea7b7615283a01d9d6019f5bfbd694d2a534ca87a7d07aa0100000000000045ff55adc8d6bc183e9abfa7bdfef3e7b942d786dcd116e776ead8238451a238ac204f516e81021ad0f8cd01"
    ),
    (
        359875, // block with BIP66 (version 3)
        "00000000000000001121383bdf780af5290a88dcba88ad38c6be5369f4b6023b",
        "030000000d6ef1e411382c00d8fd5b9d0f1acb1748e4d3de33e4320500000000000000009437e2c5a160c8e40050354ba7eb0ad0df925da3c277d0e2ae7887cab9cd4a8e137f74558b1a1718d1d2bcad"
    ),
    (
        384548, // block with BIP65 (version 4)
        "0000000000000000098702b1f6f35cc002871e012dbdb383978d4d5ffc8b6617",
        "040000001cc480a37c0c176d109c45dbdb1289e0ebe83415c5218c0d000000000000000067feb9733954f8a3cb49e9fd0f290952ead9693cae85781e83298bf5dacb6bfd01d64f5689b21018078ee0f5"
    ),
    (
        398364, // block with BIP9 (version >= 4)
        "000000000000000006e35d6675fb0fec767a5f3b346261a5160f6e2a8d258070",
        "00000030af7e7389ca428b05d8902fcdc148e70974524d39cb56bc0100000000000000007ce0cd0c9c648d1b585d29b9ab23ebc987619d43925b3c768d7cb4bc097cfb821441c05614a107187aef1ee1"
    ),
    (
        398364, // block with BIP91 and BIP141 (version >= 4)
        "0000000000000000015411ca4b35f7b48ecab015b14de5627b647e262ba0ec40",
        "12000020734713ff00fef0cd9fedd9545fa9316cacc5fd922c55220000000000000000008de88ecac287705b0548490fa5469d1e8ddaad7f0078a0fa73106b32f9a3138a272a7459dc5d01187e183e6d"
    ),
    ];

    let mut h: HashMap<Integer, Header> = HashMap::new();

    for (_height, id, header) in headers {
        let (id, tx) = get_id_to_header(id, header);
        h.insert(id, tx);
    }

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
