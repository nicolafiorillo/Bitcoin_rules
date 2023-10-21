// ripemd160(sha256()))

use ripemd::Digest;
use ripemd::Ripemd160;

pub fn hash160(s: &[u8]) -> Vec<u8> {
    let mut hasher_sha256 = sha2::Sha256::new();
    hasher_sha256.update(s);
    let hashed_sha256 = hasher_sha256.finalize();

    let mut hasher_ripemd160 = Ripemd160::new();
    hasher_ripemd160.update(hashed_sha256);
    hasher_ripemd160.finalize().to_vec()
}

#[cfg(test)]
mod hashing_test {
    use rug::{integer::Order, Integer};

    use super::hash160;
    use crate::std_lib::integer_extended::IntegerExtended;

    #[test]
    fn verify_a_hash160() {
        let hashed = hash160(&"A SECRET".to_string().as_bytes());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("BFE26C5C796D44B7091CF33E7A2FECC55C7C0278");

        assert_eq!(hashed_integer, expected);
    }

    #[test]
    fn verify_empty_string_hash160() {
        let hashed = hash160(&"".to_string().as_bytes());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("B472A266D0BD89C13706A4132CCFB16F7C3B9FCB");

        assert_eq!(hashed_integer, expected);
    }
}
