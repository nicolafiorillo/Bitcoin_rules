use rug::{integer::Order, Integer};

use crate::{
    hashing::hash256::hash256,
    std_lib::{std_result::StdResult, vector::padding_right},
    transaction::tx_lib::le_bytes_to_u32,
};

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

static HEADER_LENGTH: usize = 80;

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

    pub fn id(&self) -> String {
        format!("{:064X}", Self::hash(&self.serialize()))
    }

    fn hash(bin: &[u8]) -> Integer {
        let serialized = hash256(bin);
        Integer::from_digits(&serialized, Order::Lsf)
    }

    pub fn deserialize(bytes: &[u8]) -> StdResult<Self> {
        if bytes.len() != HEADER_LENGTH {
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

    pub fn serialize(&self) -> Vec<u8> {
        // TODO: check if this implementation is faster than the one in transaction.

        let mut bytes: Vec<u8> = Vec::new();
        bytes.reserve(HEADER_LENGTH);

        bytes.extend(&self.version.to_le_bytes());

        let previous_block: Vec<u8> = self.previous_block.to_digits(Order::Lsf);
        bytes.extend(padding_right(&previous_block, 32, 0));

        let merkle_root: Vec<u8> = self.merkle_root.to_digits(Order::Lsf);
        bytes.extend(padding_right(&merkle_root, 32, 0));

        bytes.extend(&self.timestamp.to_le_bytes());
        bytes.extend(&self.bits.to_le_bytes());
        bytes.extend(&self.nonce.to_le_bytes());

        bytes
    }
}

#[cfg(test)]
mod header_test {
    use rug::Integer;

    use crate::{
        block::header,
        chain::header::get_header,
        flags::network::Network,
        std_lib::{
            integer_extended::IntegerExtended,
            vector::{self},
        },
    };

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

    #[test]
    pub fn serialize_first_block() {
        let header = header::Header::new(
            1,
            Integer::from(0),
            Integer::from_hex_str("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"),
            1231006505,
            486604799,
            2083236893,
        );

        let serialized = header.serialize();
        assert_eq!(serialized.len(), 80);

        let res = vector::bytes_to_string(&serialized);
        assert_eq!(res, "0100000000000000000000000000000000000000000000000000000000000000000000003BA3EDFD7A7B12B27AC72C3E67768F617FC81BC3888A51323A9FB8AA4B1E5E4A29AB5F49FFFF001D1DAC2B7C");
    }

    #[test]
    pub fn serialize_second_block() {
        let header = header::Header::new(
            1,
            Integer::from_hex_str("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"),
            Integer::from_hex_str("0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098"),
            1231469665,
            486604799,
            2573394689,
        );

        let serialized = header.serialize();
        assert_eq!(serialized.len(), 80);

        let res = vector::bytes_to_string(&serialized);
        assert_eq!(res, "010000006FE28C0AB6F1B372C1A6A246AE63F74F931E8365E15A089C68D6190000000000982051FD1E4BA744BBBE680E1FEE14677BA1A3C3540BF7B1CDB606E857233E0E61BC6649FFFF001D01E36299");
    }

    #[test]
    pub fn id_first_block() {
        let header = header::Header::new(
            1,
            Integer::from(0),
            Integer::from_hex_str("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b"),
            1231006505,
            486604799,
            2083236893,
        );

        let _serialized = header.serialize();
        assert_eq!(
            header.id(),
            "000000000019D6689C085AE165831E934FF763AE46A2A6C172B3F1B60A8CE26F"
        );
    }
}
