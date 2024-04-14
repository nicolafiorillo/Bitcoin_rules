use std::fmt::{Display, Formatter};

use once_cell::sync::Lazy;
use rug::{integer::Order, ops::Pow, Float, Integer};

use crate::{
    hashing::hash256::hash256,
    std_lib::{
        integer_extended::IntegerExtended,
        std_result::StdResult,
        vector::{bytes_to_string_64, padding_right},
    },
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

static BIP9_POS: usize = 29;
static BIP91_POS: usize = 4;
static BIP141_POS: usize = 1;

static TWO_WEEKS_IN_SECONDS: u32 = 60 * 60 * 24 * 14;
static TWO_WEEKS_BY_FOUR_IN_SECONDS: u32 = TWO_WEEKS_IN_SECONDS * 4;
static TWO_WEEKS_DIV_FOUR_IN_SECONDS: u32 = TWO_WEEKS_IN_SECONDS / 4;

static MAX_BITS: u32 = 0x1D00FFFF;
static MAX_TARGET_STR: &str = "00000000FFFF0000000000000000000000000000000000000000000000000000";
static MAX_TARGET: Lazy<Integer> = Lazy::new(|| Integer::from_hex_str(MAX_TARGET_STR));

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let target = bits_to_target(self.bits);

        writeln!(
            f,
            "version: {:}\nprevious_block: {:}\nmerkle_root: {:}\ntimestamp: {:}\nbits: {:}\nnonce: {:}\n\ntarget: {:}",
            self.version,
            bytes_to_string_64(&self.previous_block.to_digits(Order::Msf)),
            bytes_to_string_64(&self.merkle_root.to_digits(Order::Msf)),
            self.timestamp,
            self.bits,
            self.nonce,
            bytes_to_string_64(&target.to_digits(Order::Msf)),
        )
    }
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

    pub fn id(&self) -> String {
        format!("{:064X}", Self::hash(&self.serialize()))
    }

    fn hash(bin: &[u8]) -> Integer {
        let serialized = hash256(bin);
        Integer::from_digits(&serialized, Order::Lsf)
    }

    pub fn deserialize(bytes: &[u8]) -> StdResult<Self> {
        if bytes.len() < HEADER_LENGTH {
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
        let mut bytes: Vec<u8> = Vec::with_capacity(HEADER_LENGTH);

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

// Resources: https://gist.github.com/Someguy123/1e4a1d1ead52c523a3ca4b1578ef1dad
// Other example: https://github.com/btcsuite/btcd/blob/91cdf0d7fc719022e65c08d0ee8ab791bcc0921d/blockchain/difficulty.go#L62
// TODO: explain
pub fn bits_to_target(bits: Bits) -> Integer {
    if bits > MAX_BITS {
        return (*MAX_TARGET).clone(); // TODO: better with error handling?
    }

    let index = bits >> 24;
    assert!(index >= 3);

    let coefficient = Integer::from(bits & 0x007fffff);

    Integer::from(256).pow(index - 3) * coefficient
}

pub fn target_to_bits(target: Integer) -> Bits {
    if target > *MAX_TARGET {
        return MAX_BITS; // TODO: better with error handling?
    }

    let bytes: Vec<u8> = target.to_digits(Order::Msf);

    let v: [u8; 4] = if bytes[0] > 0x7f {
        [(bytes.len() + 1) as u8, 0, bytes[0], bytes[1]]
    } else {
        [bytes.len() as u8, bytes[0], bytes[1], bytes[2]]
    };

    u32::from_be_bytes(v)
}

pub fn difficulty(target: Integer) -> f64 {
    // num: 0xFFFF * (256^(0x1d - 3))
    let num = Integer::from_dec_str("26959535291011309493156476344723991336010898738574164086137773096960");
    let f = Float::with_val(64, &num);

    (f / target).to_f64()
}

pub fn check_proof_of_work(header: &Header, target: &Integer) -> bool {
    let hash = Header::hash(&header.serialize());
    hash <= *target
}

pub fn adjust_target(first: &Header, last: &Header) -> Integer {
    let mut elapsed_time = last.timestamp - first.timestamp;
    let last_target = bits_to_target(last.bits);

    if elapsed_time < TWO_WEEKS_DIV_FOUR_IN_SECONDS {
        elapsed_time = TWO_WEEKS_DIV_FOUR_IN_SECONDS;
    } else if elapsed_time > TWO_WEEKS_BY_FOUR_IN_SECONDS {
        elapsed_time = TWO_WEEKS_BY_FOUR_IN_SECONDS;
    }

    last_target * elapsed_time / TWO_WEEKS_IN_SECONDS
}

macro_rules! bip_flag_is_on {
    ($f:ident, $p:ident) => {
        pub fn $f(version: u32) -> bool {
            (version >> $p & 1) == 1
        }
    };
}

// Current assignments: https://github.com/bitcoin/bips/blob/master/bip-0009/assignments.mediawiki
pub fn bip9(version: u32) -> bool {
    version >> BIP9_POS == 1 //0b001
}

bip_flag_is_on!(bip91, BIP91_POS);
bip_flag_is_on!(bip141, BIP141_POS);

#[cfg(test)]
mod header_test {
    use rug::Integer;

    use crate::{
        block::header::{self, *},
        chain::header::{get_header_by_height, get_header_by_id},
        flags::network::Network,
        std_lib::{
            integer_extended::IntegerExtended,
            vector::{self, bytes_to_string_64, hex_string_to_bytes},
        },
    };

    #[test]
    pub fn deserialize_genesis_block() {
        let block_id: Integer =
            Integer::from_hex_str("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

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
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

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
    pub fn serialize_genesis_block() {
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
    pub fn id_genesis_block() {
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

    #[test]
    pub fn version_2_for_a_block_with_bip34() {
        let block_id: Integer =
            Integer::from_hex_str("00000000000000d0dfd4c9d588d325dce4f32c1b31b7c0064cba7025a9b9adcc");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        assert_eq!(2, header.version);
    }

    #[test]
    pub fn version_3_for_a_block_with_bip66() {
        let block_id: Integer =
            Integer::from_hex_str("00000000000000001121383bdf780af5290a88dcba88ad38c6be5369f4b6023b");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        assert_eq!(3, header.version);
    }

    #[test]
    pub fn version_4_for_a_block_with_bip65() {
        let block_id: Integer =
            Integer::from_hex_str("0000000000000000098702b1f6f35cc002871e012dbdb383978d4d5ffc8b6617");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        assert_eq!(4, header.version);
    }

    #[test]
    pub fn version_with_bip9() {
        let block_id: Integer =
            Integer::from_hex_str("000000000000000006e35d6675fb0fec767a5f3b346261a5160f6e2a8d258070");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        assert!(bip9(header.version));
    }

    #[test]
    pub fn version_with_bip91() {
        let block_id: Integer =
            Integer::from_hex_str("0000000000000000015411ca4b35f7b48ecab015b14de5627b647e262ba0ec40");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        assert!(bip91(header.version));
    }

    #[test]
    pub fn version_with_bip141() {
        let block_id: Integer =
            Integer::from_hex_str("0000000000000000015411ca4b35f7b48ecab015b14de5627b647e262ba0ec40");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        assert!(bip141(header.version));
    }

    #[test]
    pub fn genesis_block_target() {
        let block_id: Integer =
            Integer::from_hex_str("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        let target = bits_to_target(header.bits);
        let target_hex = bytes_to_string_64(&target.to_digits(Order::Lsf));

        assert_eq!(
            "000000000000000000000000000000000000000000000000000000000000FFFF",
            target_hex
        );
    }

    #[test]
    pub fn genesis_block_difficulty() {
        let block_id: Integer =
            Integer::from_hex_str("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        let target = bits_to_target(header.bits);
        let difficulty = difficulty(target);

        assert_eq!(1.0, difficulty);
    }

    #[test]
    pub fn a_difficulty() {
        let bytes = hex_string_to_bytes("e93c0118").unwrap();
        let target = bits_to_target(le_bytes_to_u32(&bytes, 0).unwrap());

        let difficulty = difficulty(target);

        assert_eq!(888171856257.3206, difficulty);
    }

    #[test]
    pub fn verify_genesis_block_proof_of_work() {
        let block_id: Integer =
            Integer::from_hex_str("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        let target = bits_to_target(header.bits);

        assert!(check_proof_of_work(&header, &target));
    }

    #[test]
    pub fn verify_second_block_proof_of_work() {
        let block_id: Integer =
            Integer::from_hex_str("00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048");
        let header = get_header_by_id(&block_id, Network::Mainnet).unwrap();

        let target = bits_to_target(header.bits);

        assert!(check_proof_of_work(&header, &target));
    }

    #[test]
    pub fn verify_adjust_target_1() {
        let first_block_serialized = hex_string_to_bytes("00000020fdf740b0e49cf75bb3d5168fb3586f7613dcc5cd89675b0100000000000000002e37b144c0baced07eb7e7b64da916cd3121f2427005551aeb0ec6a6402ac7d7f0e4235954d801187f5da9f5").unwrap();
        let first_block_header = Header::deserialize(&first_block_serialized).unwrap();

        let last_block_serialized = hex_string_to_bytes("000000201ecd89664fd205a37566e694269ed76e425803003628ab010000000000000000bfcade29d080d9aae8fd461254b041805ae442749f2a40100440fc0e3d5868e55019345954d80118a1721b2e").unwrap();
        let last_block_header = Header::deserialize(&last_block_serialized).unwrap();

        let new_target = adjust_target(&first_block_header, &last_block_header);
        let new_target_hex = bytes_to_string_64(&new_target.to_digits(Order::Msf));

        assert_eq!(
            "0000000000000000019EAFC50672894AB6CD8EFB11D33F5617839A5BC7DEA00C",
            new_target_hex
        );
    }

    fn verify_adjust_target(first: &str, last: &str, bits: u32, target: &str, diff: f64) {
        let first_block_id: Integer = Integer::from_hex_str(first);
        let first_block_header = get_header_by_id(&first_block_id, Network::Mainnet).unwrap();

        let last_block_id: Integer = Integer::from_hex_str(last);
        let last_block_header = get_header_by_id(&last_block_id, Network::Mainnet).unwrap();

        let new_target = adjust_target(&first_block_header, &last_block_header);
        let new_bits = target_to_bits(new_target.clone());

        assert_eq!(new_bits, bits);

        let check_target = bits_to_target(new_bits);
        let check_target_hex = bytes_to_string_64(&check_target.to_digits(Order::Msf));

        assert_eq!(target, check_target_hex);

        // difficutly is calculate from the target calculated AFTER compacting
        // target -> bits (compacting) -> target
        assert_eq!(diff, difficulty(check_target));
    }

    fn verify_difficulty(height_first: u32, height_last: u32, diff: f64) {
        let first_block_header = get_header_by_height(&height_first, Network::Mainnet).unwrap();
        let last_block_header = get_header_by_height(&height_last, Network::Mainnet).unwrap();

        let new_target = adjust_target(&first_block_header, &last_block_header);
        let new_bits = target_to_bits(new_target.clone());

        let check_target = bits_to_target(new_bits);

        let difficulty = difficulty(check_target);
        assert_eq!(diff, difficulty);
    }

    #[test]
    pub fn verify_adjust_target_0_to_2015() {
        verify_adjust_target(
            "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
            "00000000693067b0e6b440bc51450b9f3850561b07f6d3c021c54fbd6abb9763",
            MAX_BITS,
            MAX_TARGET_STR,
            1.0,
        )
    }

    #[test]
    pub fn verify_adjust_target_30240_to_32255() {
        verify_adjust_target(
            "000000000fa8bfa0f0dd32f956b874b2c7f1772c5fbedcb1b35e03335c7fb0a8",
            "00000000984f962134a7291e3693075ae03e521f0ee33378ec30a334d860034b",
            0x1d00d86a,
            "00000000D86A0000000000000000000000000000000000000000000000000000",
            1.1828995343128408,
        )
    }

    #[test]
    pub fn verify_adjust_target_34272_to_36287() {
        verify_adjust_target(
            "000000002732d387256b57cabdcb17767e3d30e220ea73f844b1907c0b5919ea",
            "00000000128d789579ffbec00203a371cbb39cee27df35d951fd66e62ed59258",
            0x1d00be71,
            "00000000BE710000000000000000000000000000000000000000000000000000",
            1.3442249707710294,
        )
    }

    #[test]
    pub fn verify_difficulty_229824_to_231839() {
        verify_difficulty(229824, 231839, 8974296.01488785)
    }

    // Round expected difficulty to match Bitcoin Core value: probabily Bitcoin Core algorithm has less precision.
    #[test]
    pub fn verify_difficulty_431424_to_433439() {
        verify_difficulty(431424, 433439, 258522748404.5154 + 0.00004)
    }

    #[test]
    pub fn verify_difficulty_633024_to_635039() {
        verify_difficulty(633024, 635039, 15784744305477.41 - 0.002)
    }

    #[test]
    pub fn bits_to_target_over_max() {
        assert_eq!(*MAX_TARGET, bits_to_target(0xE93C0118));
    }

    #[test]
    pub fn target_to_bits_over_max() {
        let t: Integer = Integer::from_hex_str("00000001FFFF0000000000000000000000000000000000000000000000000000");
        assert_eq!(MAX_BITS, target_to_bits(t));
    }

    fn verify_target_and_bits(cases: &[(&str, u32)]) {
        for (target, bits) in cases.iter() {
            // target to bits
            let t = Integer::from_hex_str(target);
            let b = target_to_bits(t);

            assert_eq!(&b, bits);

            // bits to target
            let t = bits_to_target(*bits);
            let t_hex = bytes_to_string_64(&t.to_digits(Order::Msf));

            assert_eq!(t_hex, *target);
        }
    }

    #[test]
    pub fn target_and_bits_ffff() {
        let cases = [
            (
                "000000000000000000000000000000000000000000000000000000000000FFFF",
                0x0300FFFF,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000FFFF00",
                0x0400FFFF,
            ),
            (
                "00000000000000000000000000000000000000000000000000000000FFFF0000",
                0x0500FFFF,
            ),
            (
                "000000000000000000000000000000000000000000000000000000FFFF000000",
                0x0600FFFF,
            ),
            (
                "0000000000000000000000000000000000000000000000000000FFFF00000000",
                0x0700FFFF,
            ),
            (
                "00000000000000000000000000000000000000000000000000FFFF0000000000",
                0x0800FFFF,
            ),
            (
                "000000000000000000000000000000000000000000000000FFFF000000000000",
                0x0900FFFF,
            ),
            (
                "0000000000000000000000000000000000000000000000FFFF00000000000000",
                0x0A00FFFF,
            ),
            (
                "00000000000000000000000000000000000000000000FFFF0000000000000000",
                0x0B00FFFF,
            ),
            (
                "000000000000000000000000000000000000000000FFFF000000000000000000",
                0x0C00FFFF,
            ),
            (
                "0000000000000000000000000000000000000000FFFF00000000000000000000",
                0x0D00FFFF,
            ),
            (
                "00000000000000000000000000000000000000FFFF0000000000000000000000",
                0x0E00FFFF,
            ),
            (
                "000000000000000000000000000000000000FFFF000000000000000000000000",
                0x0F00FFFF,
            ),
            (
                "0000000000000000000000000000000000FFFF00000000000000000000000000",
                0x1000FFFF,
            ),
            (
                "00000000000000000000000000000000FFFF0000000000000000000000000000",
                0x1100FFFF,
            ),
            (
                "000000000000000000000000000000FFFF000000000000000000000000000000",
                0x1200FFFF,
            ),
            (
                "0000000000000000000000000000FFFF00000000000000000000000000000000",
                0x1300FFFF,
            ),
            (
                "00000000000000000000000000FFFF0000000000000000000000000000000000",
                0x1400FFFF,
            ),
            (
                "000000000000000000000000FFFF000000000000000000000000000000000000",
                0x1500FFFF,
            ),
            (
                "0000000000000000000000FFFF00000000000000000000000000000000000000",
                0x1600FFFF,
            ),
            (
                "00000000000000000000FFFF0000000000000000000000000000000000000000",
                0x1700FFFF,
            ),
            (
                "000000000000000000FFFF000000000000000000000000000000000000000000",
                0x1800FFFF,
            ),
            (
                "0000000000000000FFFF00000000000000000000000000000000000000000000",
                0x1900FFFF,
            ),
            (
                "00000000000000FFFF0000000000000000000000000000000000000000000000",
                0x1A00FFFF,
            ),
            (
                "000000000000FFFF000000000000000000000000000000000000000000000000",
                0x1B00FFFF,
            ),
            (
                "0000000000FFFF00000000000000000000000000000000000000000000000000",
                0x1C00FFFF,
            ),
            (
                "00000000FFFF0000000000000000000000000000000000000000000000000000",
                0x1D00FFFF,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_fefd() {
        let cases = [
            (
                "000000000000000000000000000000000000000000000000000000000000FEFD",
                0x0300FEFD,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000FEFD00",
                0x0400FEFD,
            ),
            (
                "00000000000000000000000000000000000000000000000000000000FEFD0000",
                0x0500FEFD,
            ),
            (
                "000000000000000000000000000000000000000000000000000000FEFD000000",
                0x0600FEFD,
            ),
            (
                "0000000000000000000000000000000000000000000000000000FEFD00000000",
                0x0700FEFD,
            ),
            (
                "00000000000000000000000000000000000000000000000000FEFD0000000000",
                0x0800FEFD,
            ),
            (
                "000000000000000000000000000000000000000000000000FEFD000000000000",
                0x0900FEFD,
            ),
            (
                "0000000000000000000000000000000000000000000000FEFD00000000000000",
                0x0A00FEFD,
            ),
            (
                "00000000000000000000000000000000000000000000FEFD0000000000000000",
                0x0B00FEFD,
            ),
            (
                "000000000000000000000000000000000000000000FEFD000000000000000000",
                0x0C00FEFD,
            ),
            (
                "0000000000000000000000000000000000000000FEFD00000000000000000000",
                0x0D00FEFD,
            ),
            (
                "00000000000000000000000000000000000000FEFD0000000000000000000000",
                0x0E00FEFD,
            ),
            (
                "000000000000000000000000000000000000FEFD000000000000000000000000",
                0x0F00FEFD,
            ),
            (
                "0000000000000000000000000000000000FEFD00000000000000000000000000",
                0x1000FEFD,
            ),
            (
                "00000000000000000000000000000000FEFD0000000000000000000000000000",
                0x1100FEFD,
            ),
            (
                "000000000000000000000000000000FEFD000000000000000000000000000000",
                0x1200FEFD,
            ),
            (
                "0000000000000000000000000000FEFD00000000000000000000000000000000",
                0x1300FEFD,
            ),
            (
                "00000000000000000000000000FEFD0000000000000000000000000000000000",
                0x1400FEFD,
            ),
            (
                "000000000000000000000000FEFD000000000000000000000000000000000000",
                0x1500FEFD,
            ),
            (
                "0000000000000000000000FEFD00000000000000000000000000000000000000",
                0x1600FEFD,
            ),
            (
                "00000000000000000000FEFD0000000000000000000000000000000000000000",
                0x1700FEFD,
            ),
            (
                "000000000000000000FEFD000000000000000000000000000000000000000000",
                0x1800FEFD,
            ),
            (
                "0000000000000000FEFD00000000000000000000000000000000000000000000",
                0x1900FEFD,
            ),
            (
                "00000000000000FEFD0000000000000000000000000000000000000000000000",
                0x1A00FEFD,
            ),
            (
                "000000000000FEFD000000000000000000000000000000000000000000000000",
                0x1B00FEFD,
            ),
            (
                "0000000000FEFD00000000000000000000000000000000000000000000000000",
                0x1C00FEFD,
            ),
            (
                "00000000FEFD0000000000000000000000000000000000000000000000000000",
                0x1D00FEFD,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_eeee() {
        let cases = [
            (
                "000000000000000000000000000000000000000000000000000000000000EEEE",
                0x0300EEEE,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000EEEE00",
                0x0400EEEE,
            ),
            (
                "00000000000000000000000000000000000000000000000000000000EEEE0000",
                0x0500EEEE,
            ),
            (
                "000000000000000000000000000000000000000000000000000000EEEE000000",
                0x0600EEEE,
            ),
            (
                "0000000000000000000000000000000000000000000000000000EEEE00000000",
                0x0700EEEE,
            ),
            (
                "00000000000000000000000000000000000000000000000000EEEE0000000000",
                0x0800EEEE,
            ),
            (
                "000000000000000000000000000000000000000000000000EEEE000000000000",
                0x0900EEEE,
            ),
            (
                "0000000000000000000000000000000000000000000000EEEE00000000000000",
                0x0A00EEEE,
            ),
            (
                "00000000000000000000000000000000000000000000EEEE0000000000000000",
                0x0B00EEEE,
            ),
            (
                "000000000000000000000000000000000000000000EEEE000000000000000000",
                0x0C00EEEE,
            ),
            (
                "0000000000000000000000000000000000000000EEEE00000000000000000000",
                0x0D00EEEE,
            ),
            (
                "00000000000000000000000000000000000000EEEE0000000000000000000000",
                0x0E00EEEE,
            ),
            (
                "000000000000000000000000000000000000EEEE000000000000000000000000",
                0x0F00EEEE,
            ),
            (
                "0000000000000000000000000000000000EEEE00000000000000000000000000",
                0x1000EEEE,
            ),
            (
                "00000000000000000000000000000000EEEE0000000000000000000000000000",
                0x1100EEEE,
            ),
            (
                "000000000000000000000000000000EEEE000000000000000000000000000000",
                0x1200EEEE,
            ),
            (
                "0000000000000000000000000000EEEE00000000000000000000000000000000",
                0x1300EEEE,
            ),
            (
                "00000000000000000000000000EEEE0000000000000000000000000000000000",
                0x1400EEEE,
            ),
            (
                "000000000000000000000000EEEE000000000000000000000000000000000000",
                0x1500EEEE,
            ),
            (
                "0000000000000000000000EEEE00000000000000000000000000000000000000",
                0x1600EEEE,
            ),
            (
                "00000000000000000000EEEE0000000000000000000000000000000000000000",
                0x1700EEEE,
            ),
            (
                "000000000000000000EEEE000000000000000000000000000000000000000000",
                0x1800EEEE,
            ),
            (
                "0000000000000000EEEE00000000000000000000000000000000000000000000",
                0x1900EEEE,
            ),
            (
                "00000000000000EEEE0000000000000000000000000000000000000000000000",
                0x1A00EEEE,
            ),
            (
                "000000000000EEEE000000000000000000000000000000000000000000000000",
                0x1B00EEEE,
            ),
            (
                "0000000000EEEE00000000000000000000000000000000000000000000000000",
                0x1C00EEEE,
            ),
            (
                "00000000EEEE0000000000000000000000000000000000000000000000000000",
                0x1D00EEEE,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_dddd() {
        let cases = [
            (
                "000000000000000000000000000000000000000000000000000000000000DDDD",
                0x0300DDDD,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000DDDD00",
                0x0400DDDD,
            ),
            (
                "00000000000000000000000000000000000000000000000000000000DDDD0000",
                0x0500DDDD,
            ),
            (
                "000000000000000000000000000000000000000000000000000000DDDD000000",
                0x0600DDDD,
            ),
            (
                "0000000000000000000000000000000000000000000000000000DDDD00000000",
                0x0700DDDD,
            ),
            (
                "00000000000000000000000000000000000000000000000000DDDD0000000000",
                0x0800DDDD,
            ),
            (
                "000000000000000000000000000000000000000000000000DDDD000000000000",
                0x0900DDDD,
            ),
            (
                "0000000000000000000000000000000000000000000000DDDD00000000000000",
                0x0A00DDDD,
            ),
            (
                "00000000000000000000000000000000000000000000DDDD0000000000000000",
                0x0B00DDDD,
            ),
            (
                "000000000000000000000000000000000000000000DDDD000000000000000000",
                0x0C00DDDD,
            ),
            (
                "0000000000000000000000000000000000000000DDDD00000000000000000000",
                0x0D00DDDD,
            ),
            (
                "00000000000000000000000000000000000000DDDD0000000000000000000000",
                0x0E00DDDD,
            ),
            (
                "000000000000000000000000000000000000DDDD000000000000000000000000",
                0x0F00DDDD,
            ),
            (
                "0000000000000000000000000000000000DDDD00000000000000000000000000",
                0x1000DDDD,
            ),
            (
                "00000000000000000000000000000000DDDD0000000000000000000000000000",
                0x1100DDDD,
            ),
            (
                "000000000000000000000000000000DDDD000000000000000000000000000000",
                0x1200DDDD,
            ),
            (
                "0000000000000000000000000000DDDD00000000000000000000000000000000",
                0x1300DDDD,
            ),
            (
                "00000000000000000000000000DDDD0000000000000000000000000000000000",
                0x1400DDDD,
            ),
            (
                "000000000000000000000000DDDD000000000000000000000000000000000000",
                0x1500DDDD,
            ),
            (
                "0000000000000000000000DDDD00000000000000000000000000000000000000",
                0x1600DDDD,
            ),
            (
                "00000000000000000000DDDD0000000000000000000000000000000000000000",
                0x1700DDDD,
            ),
            (
                "000000000000000000DDDD000000000000000000000000000000000000000000",
                0x1800DDDD,
            ),
            (
                "0000000000000000DDDD00000000000000000000000000000000000000000000",
                0x1900DDDD,
            ),
            (
                "00000000000000DDDD0000000000000000000000000000000000000000000000",
                0x1A00DDDD,
            ),
            (
                "000000000000DDDD000000000000000000000000000000000000000000000000",
                0x1B00DDDD,
            ),
            (
                "0000000000DDDD00000000000000000000000000000000000000000000000000",
                0x1C00DDDD,
            ),
            (
                "00000000DDDD0000000000000000000000000000000000000000000000000000",
                0x1D00DDDD,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_cccc() {
        let cases = [
            (
                "000000000000000000000000000000000000000000000000000000000000CCCC",
                0x0300CCCC,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000CCCC00",
                0x0400CCCC,
            ),
            (
                "00000000000000000000000000000000000000000000000000000000CCCC0000",
                0x0500CCCC,
            ),
            (
                "000000000000000000000000000000000000000000000000000000CCCC000000",
                0x0600CCCC,
            ),
            (
                "0000000000000000000000000000000000000000000000000000CCCC00000000",
                0x0700CCCC,
            ),
            (
                "00000000000000000000000000000000000000000000000000CCCC0000000000",
                0x0800CCCC,
            ),
            (
                "000000000000000000000000000000000000000000000000CCCC000000000000",
                0x0900CCCC,
            ),
            (
                "0000000000000000000000000000000000000000000000CCCC00000000000000",
                0x0A00CCCC,
            ),
            (
                "00000000000000000000000000000000000000000000CCCC0000000000000000",
                0x0B00CCCC,
            ),
            (
                "000000000000000000000000000000000000000000CCCC000000000000000000",
                0x0C00CCCC,
            ),
            (
                "0000000000000000000000000000000000000000CCCC00000000000000000000",
                0x0D00CCCC,
            ),
            (
                "00000000000000000000000000000000000000CCCC0000000000000000000000",
                0x0E00CCCC,
            ),
            (
                "000000000000000000000000000000000000CCCC000000000000000000000000",
                0x0F00CCCC,
            ),
            (
                "0000000000000000000000000000000000CCCC00000000000000000000000000",
                0x1000CCCC,
            ),
            (
                "00000000000000000000000000000000CCCC0000000000000000000000000000",
                0x1100CCCC,
            ),
            (
                "000000000000000000000000000000CCCC000000000000000000000000000000",
                0x1200CCCC,
            ),
            (
                "0000000000000000000000000000CCCC00000000000000000000000000000000",
                0x1300CCCC,
            ),
            (
                "00000000000000000000000000CCCC0000000000000000000000000000000000",
                0x1400CCCC,
            ),
            (
                "000000000000000000000000CCCC000000000000000000000000000000000000",
                0x1500CCCC,
            ),
            (
                "0000000000000000000000CCCC00000000000000000000000000000000000000",
                0x1600CCCC,
            ),
            (
                "00000000000000000000CCCC0000000000000000000000000000000000000000",
                0x1700CCCC,
            ),
            (
                "000000000000000000CCCC000000000000000000000000000000000000000000",
                0x1800CCCC,
            ),
            (
                "0000000000000000CCCC00000000000000000000000000000000000000000000",
                0x1900CCCC,
            ),
            (
                "00000000000000CCCC0000000000000000000000000000000000000000000000",
                0x1A00CCCC,
            ),
            (
                "000000000000CCCC000000000000000000000000000000000000000000000000",
                0x1B00CCCC,
            ),
            (
                "0000000000CCCC00000000000000000000000000000000000000000000000000",
                0x1C00CCCC,
            ),
            (
                "00000000CCCC0000000000000000000000000000000000000000000000000000",
                0x1D00CCCC,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_bbbb() {
        let cases = [
            (
                "000000000000000000000000000000000000000000000000000000000000BBBB",
                0x0300BBBB,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000BBBB00",
                0x0400BBBB,
            ),
            (
                "00000000000000000000000000000000000000000000000000000000BBBB0000",
                0x0500BBBB,
            ),
            (
                "000000000000000000000000000000000000000000000000000000BBBB000000",
                0x0600BBBB,
            ),
            (
                "0000000000000000000000000000000000000000000000000000BBBB00000000",
                0x0700BBBB,
            ),
            (
                "00000000000000000000000000000000000000000000000000BBBB0000000000",
                0x0800BBBB,
            ),
            (
                "000000000000000000000000000000000000000000000000BBBB000000000000",
                0x0900BBBB,
            ),
            (
                "0000000000000000000000000000000000000000000000BBBB00000000000000",
                0x0A00BBBB,
            ),
            (
                "00000000000000000000000000000000000000000000BBBB0000000000000000",
                0x0B00BBBB,
            ),
            (
                "000000000000000000000000000000000000000000BBBB000000000000000000",
                0x0C00BBBB,
            ),
            (
                "0000000000000000000000000000000000000000BBBB00000000000000000000",
                0x0D00BBBB,
            ),
            (
                "00000000000000000000000000000000000000BBBB0000000000000000000000",
                0x0E00BBBB,
            ),
            (
                "000000000000000000000000000000000000BBBB000000000000000000000000",
                0x0F00BBBB,
            ),
            (
                "0000000000000000000000000000000000BBBB00000000000000000000000000",
                0x1000BBBB,
            ),
            (
                "00000000000000000000000000000000BBBB0000000000000000000000000000",
                0x1100BBBB,
            ),
            (
                "000000000000000000000000000000BBBB000000000000000000000000000000",
                0x1200BBBB,
            ),
            (
                "0000000000000000000000000000BBBB00000000000000000000000000000000",
                0x1300BBBB,
            ),
            (
                "00000000000000000000000000BBBB0000000000000000000000000000000000",
                0x1400BBBB,
            ),
            (
                "000000000000000000000000BBBB000000000000000000000000000000000000",
                0x1500BBBB,
            ),
            (
                "0000000000000000000000BBBB00000000000000000000000000000000000000",
                0x1600BBBB,
            ),
            (
                "00000000000000000000BBBB0000000000000000000000000000000000000000",
                0x1700BBBB,
            ),
            (
                "000000000000000000BBBB000000000000000000000000000000000000000000",
                0x1800BBBB,
            ),
            (
                "0000000000000000BBBB00000000000000000000000000000000000000000000",
                0x1900BBBB,
            ),
            (
                "00000000000000BBBB0000000000000000000000000000000000000000000000",
                0x1A00BBBB,
            ),
            (
                "000000000000BBBB000000000000000000000000000000000000000000000000",
                0x1B00BBBB,
            ),
            (
                "0000000000BBBB00000000000000000000000000000000000000000000000000",
                0x1C00BBBB,
            ),
            (
                "00000000BBBB0000000000000000000000000000000000000000000000000000",
                0x1D00BBBB,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_aaaa() {
        let cases = [
            (
                "000000000000000000000000000000000000000000000000000000000000AAAA",
                0x0300AAAA,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000AAAA00",
                0x0400AAAA,
            ),
            (
                "00000000000000000000000000000000000000000000000000000000AAAA0000",
                0x0500AAAA,
            ),
            (
                "000000000000000000000000000000000000000000000000000000AAAA000000",
                0x0600AAAA,
            ),
            (
                "0000000000000000000000000000000000000000000000000000AAAA00000000",
                0x0700AAAA,
            ),
            (
                "00000000000000000000000000000000000000000000000000AAAA0000000000",
                0x0800AAAA,
            ),
            (
                "000000000000000000000000000000000000000000000000AAAA000000000000",
                0x0900AAAA,
            ),
            (
                "0000000000000000000000000000000000000000000000AAAA00000000000000",
                0x0A00AAAA,
            ),
            (
                "00000000000000000000000000000000000000000000AAAA0000000000000000",
                0x0B00AAAA,
            ),
            (
                "000000000000000000000000000000000000000000AAAA000000000000000000",
                0x0C00AAAA,
            ),
            (
                "0000000000000000000000000000000000000000AAAA00000000000000000000",
                0x0D00AAAA,
            ),
            (
                "00000000000000000000000000000000000000AAAA0000000000000000000000",
                0x0E00AAAA,
            ),
            (
                "000000000000000000000000000000000000AAAA000000000000000000000000",
                0x0F00AAAA,
            ),
            (
                "0000000000000000000000000000000000AAAA00000000000000000000000000",
                0x1000AAAA,
            ),
            (
                "00000000000000000000000000000000AAAA0000000000000000000000000000",
                0x1100AAAA,
            ),
            (
                "000000000000000000000000000000AAAA000000000000000000000000000000",
                0x1200AAAA,
            ),
            (
                "0000000000000000000000000000AAAA00000000000000000000000000000000",
                0x1300AAAA,
            ),
            (
                "00000000000000000000000000AAAA0000000000000000000000000000000000",
                0x1400AAAA,
            ),
            (
                "000000000000000000000000AAAA000000000000000000000000000000000000",
                0x1500AAAA,
            ),
            (
                "0000000000000000000000AAAA00000000000000000000000000000000000000",
                0x1600AAAA,
            ),
            (
                "00000000000000000000AAAA0000000000000000000000000000000000000000",
                0x1700AAAA,
            ),
            (
                "000000000000000000AAAA000000000000000000000000000000000000000000",
                0x1800AAAA,
            ),
            (
                "0000000000000000AAAA00000000000000000000000000000000000000000000",
                0x1900AAAA,
            ),
            (
                "00000000000000AAAA0000000000000000000000000000000000000000000000",
                0x1A00AAAA,
            ),
            (
                "000000000000AAAA000000000000000000000000000000000000000000000000",
                0x1B00AAAA,
            ),
            (
                "0000000000AAAA00000000000000000000000000000000000000000000000000",
                0x1C00AAAA,
            ),
            (
                "00000000AAAA0000000000000000000000000000000000000000000000000000",
                0x1D00AAAA,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_9999() {
        let cases = [
            (
                "0000000000000000000000000000000000000000000000000000000000009999",
                0x03009999,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000999900",
                0x04009999,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000099990000",
                0x05009999,
            ),
            (
                "0000000000000000000000000000000000000000000000000000009999000000",
                0x06009999,
            ),
            (
                "0000000000000000000000000000000000000000000000000000999900000000",
                0x07009999,
            ),
            (
                "0000000000000000000000000000000000000000000000000099990000000000",
                0x08009999,
            ),
            (
                "0000000000000000000000000000000000000000000000009999000000000000",
                0x09009999,
            ),
            (
                "0000000000000000000000000000000000000000000000999900000000000000",
                0x0A009999,
            ),
            (
                "0000000000000000000000000000000000000000000099990000000000000000",
                0x0B009999,
            ),
            (
                "0000000000000000000000000000000000000000009999000000000000000000",
                0x0C009999,
            ),
            (
                "0000000000000000000000000000000000000000999900000000000000000000",
                0x0D009999,
            ),
            (
                "0000000000000000000000000000000000000099990000000000000000000000",
                0x0E009999,
            ),
            (
                "0000000000000000000000000000000000009999000000000000000000000000",
                0x0F009999,
            ),
            (
                "0000000000000000000000000000000000999900000000000000000000000000",
                0x10009999,
            ),
            (
                "0000000000000000000000000000000099990000000000000000000000000000",
                0x11009999,
            ),
            (
                "0000000000000000000000000000009999000000000000000000000000000000",
                0x12009999,
            ),
            (
                "0000000000000000000000000000999900000000000000000000000000000000",
                0x13009999,
            ),
            (
                "0000000000000000000000000099990000000000000000000000000000000000",
                0x14009999,
            ),
            (
                "0000000000000000000000009999000000000000000000000000000000000000",
                0x15009999,
            ),
            (
                "0000000000000000000000999900000000000000000000000000000000000000",
                0x16009999,
            ),
            (
                "0000000000000000000099990000000000000000000000000000000000000000",
                0x17009999,
            ),
            (
                "0000000000000000009999000000000000000000000000000000000000000000",
                0x18009999,
            ),
            (
                "0000000000000000999900000000000000000000000000000000000000000000",
                0x19009999,
            ),
            (
                "0000000000000099990000000000000000000000000000000000000000000000",
                0x1A009999,
            ),
            (
                "0000000000009999000000000000000000000000000000000000000000000000",
                0x1B009999,
            ),
            (
                "0000000000999900000000000000000000000000000000000000000000000000",
                0x1C009999,
            ),
            (
                "0000000099990000000000000000000000000000000000000000000000000000",
                0x1D009999,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_8888() {
        let cases = [
            (
                "0000000000000000000000000000000000000000000000000000000000008888",
                0x03008888,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000888800",
                0x04008888,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000088880000",
                0x05008888,
            ),
            (
                "0000000000000000000000000000000000000000000000000000008888000000",
                0x06008888,
            ),
            (
                "0000000000000000000000000000000000000000000000000000888800000000",
                0x07008888,
            ),
            (
                "0000000000000000000000000000000000000000000000000088880000000000",
                0x08008888,
            ),
            (
                "0000000000000000000000000000000000000000000000008888000000000000",
                0x09008888,
            ),
            (
                "0000000000000000000000000000000000000000000000888800000000000000",
                0x0A008888,
            ),
            (
                "0000000000000000000000000000000000000000000088880000000000000000",
                0x0B008888,
            ),
            (
                "0000000000000000000000000000000000000000008888000000000000000000",
                0x0C008888,
            ),
            (
                "0000000000000000000000000000000000000000888800000000000000000000",
                0x0D008888,
            ),
            (
                "0000000000000000000000000000000000000088880000000000000000000000",
                0x0E008888,
            ),
            (
                "0000000000000000000000000000000000008888000000000000000000000000",
                0x0F008888,
            ),
            (
                "0000000000000000000000000000000000888800000000000000000000000000",
                0x10008888,
            ),
            (
                "0000000000000000000000000000000088880000000000000000000000000000",
                0x11008888,
            ),
            (
                "0000000000000000000000000000008888000000000000000000000000000000",
                0x12008888,
            ),
            (
                "0000000000000000000000000000888800000000000000000000000000000000",
                0x13008888,
            ),
            (
                "0000000000000000000000000088880000000000000000000000000000000000",
                0x14008888,
            ),
            (
                "0000000000000000000000008888000000000000000000000000000000000000",
                0x15008888,
            ),
            (
                "0000000000000000000000888800000000000000000000000000000000000000",
                0x16008888,
            ),
            (
                "0000000000000000000088880000000000000000000000000000000000000000",
                0x17008888,
            ),
            (
                "0000000000000000008888000000000000000000000000000000000000000000",
                0x18008888,
            ),
            (
                "0000000000000000888800000000000000000000000000000000000000000000",
                0x19008888,
            ),
            (
                "0000000000000088880000000000000000000000000000000000000000000000",
                0x1A008888,
            ),
            (
                "0000000000008888000000000000000000000000000000000000000000000000",
                0x1B008888,
            ),
            (
                "0000000000888800000000000000000000000000000000000000000000000000",
                0x1C008888,
            ),
            (
                "0000000088880000000000000000000000000000000000000000000000000000",
                0x1D008888,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_8000() {
        let cases = [
            (
                "0000000000000000000000000000000000000000000000000000000000008000",
                0x03008000,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000000800000",
                0x04008000,
            ),
            (
                "0000000000000000000000000000000000000000000000000000000080000000",
                0x05008000,
            ),
            (
                "0000000000000000000000000000000000000000000000000000008000000000",
                0x06008000,
            ),
            (
                "0000000000000000000000000000000000000000000000000000800000000000",
                0x07008000,
            ),
            (
                "0000000000000000000000000000000000000000000000000080000000000000",
                0x08008000,
            ),
            (
                "0000000000000000000000000000000000000000000000008000000000000000",
                0x09008000,
            ),
            (
                "0000000000000000000000000000000000000000000000800000000000000000",
                0x0A008000,
            ),
            (
                "0000000000000000000000000000000000000000000080000000000000000000",
                0x0B008000,
            ),
            (
                "0000000000000000000000000000000000000000008000000000000000000000",
                0x0C008000,
            ),
            (
                "0000000000000000000000000000000000000000800000000000000000000000",
                0x0D008000,
            ),
            (
                "0000000000000000000000000000000000000080000000000000000000000000",
                0x0E008000,
            ),
            (
                "0000000000000000000000000000000000008000000000000000000000000000",
                0x0F008000,
            ),
            (
                "0000000000000000000000000000000000800000000000000000000000000000",
                0x10008000,
            ),
            (
                "0000000000000000000000000000000080000000000000000000000000000000",
                0x11008000,
            ),
            (
                "0000000000000000000000000000008000000000000000000000000000000000",
                0x12008000,
            ),
            (
                "0000000000000000000000000000800000000000000000000000000000000000",
                0x13008000,
            ),
            (
                "0000000000000000000000000080000000000000000000000000000000000000",
                0x14008000,
            ),
            (
                "0000000000000000000000008000000000000000000000000000000000000000",
                0x15008000,
            ),
            (
                "0000000000000000000000800000000000000000000000000000000000000000",
                0x16008000,
            ),
            (
                "0000000000000000000080000000000000000000000000000000000000000000",
                0x17008000,
            ),
            (
                "0000000000000000008000000000000000000000000000000000000000000000",
                0x18008000,
            ),
            (
                "0000000000000000800000000000000000000000000000000000000000000000",
                0x19008000,
            ),
            (
                "0000000000000080000000000000000000000000000000000000000000000000",
                0x1A008000,
            ),
            (
                "0000000000008000000000000000000000000000000000000000000000000000",
                0x1B008000,
            ),
            (
                "0000000000800000000000000000000000000000000000000000000000000000",
                0x1C008000,
            ),
            (
                "0000000080000000000000000000000000000000000000000000000000000000",
                0x1D008000,
            ),
        ];

        verify_target_and_bits(&cases);
    }

    #[test]
    pub fn target_and_bits_7fffff() {
        let cases = [
            (
                "00000000000000000000000000000000000000000000000000000000007FFFFF",
                0x037FFFFF,
            ),
            (
                "000000000000000000000000000000000000000000000000000000007FFFFF00",
                0x047FFFFF,
            ),
            (
                "0000000000000000000000000000000000000000000000000000007FFFFF0000",
                0x057FFFFF,
            ),
            (
                "00000000000000000000000000000000000000000000000000007FFFFF000000",
                0x067FFFFF,
            ),
            (
                "000000000000000000000000000000000000000000000000007FFFFF00000000",
                0x077FFFFF,
            ),
            (
                "0000000000000000000000000000000000000000000000007FFFFF0000000000",
                0x087FFFFF,
            ),
            (
                "00000000000000000000000000000000000000000000007FFFFF000000000000",
                0x097FFFFF,
            ),
            (
                "000000000000000000000000000000000000000000007FFFFF00000000000000",
                0x0A7FFFFF,
            ),
            (
                "0000000000000000000000000000000000000000007FFFFF0000000000000000",
                0x0B7FFFFF,
            ),
            (
                "00000000000000000000000000000000000000007FFFFF000000000000000000",
                0x0C7FFFFF,
            ),
            (
                "000000000000000000000000000000000000007FFFFF00000000000000000000",
                0x0D7FFFFF,
            ),
            (
                "0000000000000000000000000000000000007FFFFF0000000000000000000000",
                0x0E7FFFFF,
            ),
            (
                "00000000000000000000000000000000007FFFFF000000000000000000000000",
                0x0F7FFFFF,
            ),
            (
                "000000000000000000000000000000007FFFFF00000000000000000000000000",
                0x107FFFFF,
            ),
            (
                "0000000000000000000000000000007FFFFF0000000000000000000000000000",
                0x117FFFFF,
            ),
            (
                "00000000000000000000000000007FFFFF000000000000000000000000000000",
                0x127FFFFF,
            ),
            (
                "000000000000000000000000007FFFFF00000000000000000000000000000000",
                0x137FFFFF,
            ),
            (
                "0000000000000000000000007FFFFF0000000000000000000000000000000000",
                0x147FFFFF,
            ),
            (
                "00000000000000000000007FFFFF000000000000000000000000000000000000",
                0x157FFFFF,
            ),
            (
                "000000000000000000007FFFFF00000000000000000000000000000000000000",
                0x167FFFFF,
            ),
            (
                "0000000000000000007FFFFF0000000000000000000000000000000000000000",
                0x177FFFFF,
            ),
            (
                "00000000000000007FFFFF000000000000000000000000000000000000000000",
                0x187FFFFF,
            ),
            (
                "000000000000007FFFFF00000000000000000000000000000000000000000000",
                0x197FFFFF,
            ),
            (
                "0000000000007FFFFF0000000000000000000000000000000000000000000000",
                0x1A7FFFFF,
            ),
            (
                "00000000007FFFFF000000000000000000000000000000000000000000000000",
                0x1B7FFFFF,
            ),
            (
                "000000007FFFFF00000000000000000000000000000000000000000000000000",
                0x1C7FFFFF,
            ),
        ];

        verify_target_and_bits(&cases);
    }
}
