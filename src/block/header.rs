use rug::{integer::Order, Integer};

use crate::{std_lib::std_result::StdResult, transaction::tx_lib::le_bytes_to_u32};

type Version = u32;
type PreviousBlock = Integer; // will be u256 or [u8; 32]
type MerkleRoot = Integer; // will be u256 or [u8; 32]
type Timestamp = u32;
type Bits = u32;
type Nonce = u32;

#[derive(Debug, Clone)]
pub struct Header {
    pub version: Version,
    pub previous_block: PreviousBlock,
    pub merkle_root: MerkleRoot,
    pub timestamp: Timestamp,
    pub bits: Bits,
    pub nonce: Nonce,
}

impl Header {
    pub fn new(
        version: Version,
        previous_block: PreviousBlock,
        merkle_root: MerkleRoot,
        timestamp: Timestamp,
        bits: Bits,
        nonce: Nonce,
    ) -> Self {
        Self {
            version,
            previous_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
        }
    }

    pub fn deserialize(bytes: &[u8]) -> StdResult<Self> {
        if bytes.len() != 80 {
            Err("invalid_header_length")?;
        }

        let mut cursor: usize = 0;

        // Version
        let version = le_bytes_to_u32(bytes, cursor)?;
        cursor += 4;

        // Previous block
        let pb = &bytes[cursor..cursor + 32];
        let previous_block = Integer::from_digits(pb, Order::Lsf);
        cursor += 32;

        // Merkle root
        let mr = &bytes[cursor..cursor + 32];
        let merkle_root = Integer::from_digits(mr, Order::Lsf);
        cursor += 32;

        // Timestamp
        let timestamp = le_bytes_to_u32(bytes, cursor)?;
        cursor += 4;

        // Bits
        let bits = le_bytes_to_u32(bytes, cursor)?;
        cursor += 4;

        // nonce
        let nonce = le_bytes_to_u32(bytes, cursor)?;

        Ok(Header {
            version,
            previous_block,
            merkle_root,
            timestamp,
            bits,
            nonce,
        })
    }
}

#[cfg(test)]
mod header_test {
    use rug::Integer;

    use crate::{chain::header::get_header, flags::network::Network, std_lib::integer_extended::IntegerExtended};

    #[test]
    pub fn deserialize_first_block() {
        let block_id: Integer =
            Integer::from_hex_str("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
        let header = get_header(&block_id, Network::Mainnet).unwrap();

        assert_eq!(1, header.version);
        assert_eq!(Integer::from(0), header.previous_block);
        assert_eq!(
            Integer::from_hex_str("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"),
            header.merkle_root
        );
        assert_eq!(1231006505, header.timestamp);
        assert_eq!(486604799, header.bits);
        assert_eq!(2083236893, header.nonce);
    }

    #[test]
    pub fn deserialize_second_block() {
        let block_id: Integer =
            Integer::from_hex_str("00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048");
        let header = get_header(&block_id, Network::Mainnet).unwrap();

        assert_eq!(1, header.version);
        assert_eq!(
            Integer::from_hex_str("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"),
            header.previous_block
        );
        assert_eq!(
            Integer::from_hex_str("0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098"),
            header.merkle_root
        );
        assert_eq!(1231469665, header.timestamp);
        assert_eq!(486604799, header.bits);
        assert_eq!(2573394689, header.nonce);
    }
}
