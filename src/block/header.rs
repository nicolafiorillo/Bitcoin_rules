use rug::Integer;

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

    // pub fn deserialize(serialized: &[u8], network: Network) -> StdResult<Self> {
    //     if serialized.len() != 80 {
    //         return Err(BlockError::InvalidHeaderLength);
    //     }

    //     let mut cursor: usize = 0;

    //     // Version
    //     let version = le_bytes_to_u32(serialized, cursor)?;
    //     cursor += 4;

    //     Ok(Header {
    //         version: (),
    //         previous_block: (),
    //         merkle_root: (),
    //         timestamp: (),
    //         bits: (),
    //         nonce: (),
    //     })
    // }
}
