// ripemd160()

use ripemd::Digest;
use ripemd::Ripemd160;

pub fn ripemd160(s: &[u8]) -> Vec<u8> {
    let mut hasher_ripemd160 = Ripemd160::new();
    hasher_ripemd160.update(s);
    hasher_ripemd160.finalize().to_vec()
}

#[cfg(test)]
mod hashing_test {
    use rug::{integer::Order, Integer};

    use super::ripemd160;
    use crate::std_lib::integer_extended::IntegerExtended;

    #[test]
    fn verify_a_hash160() {
        let hashed = ripemd160(&"A SECRET".to_string().as_bytes());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("B9FF4EB2C0D13B5E8B3E3F75A0AD4FC8403A8D0D");

        assert_eq!(hashed_integer, expected);
    }

    #[test]
    fn verify_empty_string_hash160() {
        let hashed = ripemd160(&"".to_string().as_bytes());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("9C1185A5C5E9FC54612808977EE8F548B2258D31");

        assert_eq!(hashed_integer, expected);
    }
}
