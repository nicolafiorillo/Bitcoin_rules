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
    use crate::low::integer_ex::IntegerEx;

    #[test]
    fn verify_a_hash160() {
        let hashed = hash160(&"A SECRET".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("bfe26c5c796d44b7091cf33e7a2fecc55c7c0278");

        assert_eq!(hashed_integer, expected);
    }

    #[test]
    fn verify_empty_string_hash160() {
        let hashed = hash160(&"".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("b472a266d0bd89c13706a4132ccfb16f7c3b9fcb");

        assert_eq!(hashed_integer, expected);
    }
}
