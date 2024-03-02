use once_cell::sync::Lazy;
use rug::{integer::Order, Integer};

use super::std_result::StdResult;

static BASE58_ALPHABET: Lazy<Vec<char>> = Lazy::new(|| {
    "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        .chars()
        .collect()
});

const BASE58_ALPHABET_LENGTH: u8 = 58;

pub fn base58_encode(binary: &[u8]) -> String {
    // We will need it for pay-to-pubkey-hash (p2pkh)
    let zeroes: usize = count_first(binary, 0);
    let mut result = "".to_string();
    let mut num = Integer::from_digits(binary, Order::Msf);

    while num > 0 {
        let (new_num, remainder) = num.clone().div_rem(Integer::from(58));
        num = new_num;

        let prefix = (*BASE58_ALPHABET)[remainder.to_usize().unwrap()].to_string();
        result = prefix + &result;
    }

    format!("{}{}", "1".repeat(zeroes), result)
}

pub fn base58_decode(s: &str) -> Integer {
    // We will need it for pay-to-pubkey-hash (p2pkh)
    // manage leading 1 to zeros

    let mut result = Integer::from(0);
    let mut multi = Integer::from(1);

    for val in s.chars().rev() {
        result += (multi.clone()) * (*BASE58_ALPHABET).iter().position(|v| v == &val).unwrap();
        multi *= BASE58_ALPHABET_LENGTH;
    }

    result
}

// We will need it for pay-to-pubkey-hash (p2pkh)
fn count_first(binary: &[u8], val: u8) -> usize {
    let mut counter: usize = 0;
    let len = binary.len();

    while counter < len && binary[counter] == val {
        counter += 1;
    }

    counter
}

pub fn base58_encode_with_checksum(b: &[u8]) -> String {
    use crate::hashing::hash256::hash256;

    let checksum: Vec<u8> = hash256(b).drain(0..4).collect();
    let mut bin: Vec<u8> = b.to_vec();
    bin.extend(checksum);

    base58_encode(&bin)
}

pub fn base58_decode_with_checksum(s: &str) -> StdResult<Vec<u8>> {
    use crate::hashing::hash256::hash256;

    let d = base58_decode(s).to_digits(Order::Msf);

    if d.len() < 4 {
        Err("invalid_length")?;
    }

    let (data, checksum) = d.split_at(d.len() - 4);

    let mut hash = hash256(data);
    let drained = hash.drain(0..4);
    let data_checksum = drained.as_slice();

    if checksum != data_checksum {
        Err("invalid_checksum")?;
    }

    Ok(data.to_vec())
}

#[cfg(test)]
mod base58_test {
    use rug::{integer::Order, Integer};

    use crate::std_lib::integer_extended::IntegerExtended;

    use super::{base58_decode, base58_decode_with_checksum, base58_encode, base58_encode_with_checksum};

    #[test]
    fn encode_1() {
        let val = Integer::from_hex_str("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d");
        let v = val.to_digits::<u8>(Order::Msf);

        assert_eq!("9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6", base58_encode(&v))
    }

    #[test]
    fn encode_2() {
        let val = Integer::from_hex_str("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c");
        let v = val.to_digits::<u8>(Order::Msf);

        assert_eq!("4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd", base58_encode(&v))
    }

    #[test]
    fn encode_3() {
        let val = Integer::from_hex_str("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6");
        let v = val.to_digits::<u8>(Order::Msf);

        assert_eq!("EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7", base58_encode(&v))
    }

    #[test]
    fn decode_1() {
        let res = base58_decode("9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6");
        let expected = Integer::from_hex_str("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d");

        assert_eq!(expected, res)
    }

    #[test]
    fn decode_2() {
        let res = base58_decode("4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd");
        let expected = Integer::from_hex_str("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c");

        assert_eq!(expected, res)
    }
    #[test]
    fn decode_3() {
        let res = base58_decode("EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7");
        let expected = Integer::from_hex_str("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6");

        assert_eq!(expected, res)
    }

    #[test]
    fn encode_checksum_1() {
        let res = base58_encode_with_checksum(&"11".to_string().as_bytes());
        assert_eq!("RVnPfpC2", res)
    }

    #[test]
    fn encode_checksum_2() {
        let res = base58_encode_with_checksum(&"".to_string().as_bytes());
        assert_eq!("3QJmnh", res)
    }
    #[test]
    fn encode_checksum_3() {
        let res = base58_encode_with_checksum(
            &"4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd"
                .to_string()
                .as_bytes()
                .to_vec(),
        );
        assert_eq!("SFyVFVE84dMDxTAX88Rq8UJA2mWVNASRdWNorzbCAP22Qums1CuoZcPKU7xkjpBf", res)
    }

    #[test]
    fn decode_checksum_1() {
        let res = base58_decode_with_checksum("RVnPfpC2");
        assert_eq!("11".to_string().as_bytes().to_vec(), res.ok().unwrap())
    }

    #[test]
    fn decode_checksum_2() {
        let res = base58_decode_with_checksum("3QJmnh");
        assert_eq!("".to_string().as_bytes().to_vec(), res.ok().unwrap())
    }

    #[test]
    fn decode_checksum_3() {
        let res = base58_decode_with_checksum("SFyVFVE84dMDxTAX88Rq8UJA2mWVNASRdWNorzbCAP22Qums1CuoZcPKU7xkjpBf");
        assert_eq!(
            "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd"
                .to_string()
                .as_bytes()
                .to_vec(),
            res.ok().unwrap()
        )
    }

    #[test]
    fn decode_checksum_invalid_checksum() {
        let res = base58_decode_with_checksum("SFyVFVE84dMDxTAX88Rq8UJA2mWVNASRdWNorzbCAP22Qums1CuoZcPKU7xkjpBe");
        assert_eq!("invalid_checksum", res.err().unwrap().to_string())
    }

    #[test]
    fn decode_checksum_invalid_length() {
        let res = base58_decode_with_checksum("a");
        assert_eq!("invalid_length", res.err().unwrap().to_string())
    }
}
