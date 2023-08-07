use ripemd::Digest;
use sha2::Sha256;

pub fn hash256(s: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(s);

    let hashed = hasher.finalize();

    hasher = Sha256::new();
    hasher.update(hashed.as_slice());

    hasher.finalize().to_vec()
}

#[cfg(test)]
mod hashing_test {
    use rug::{integer::Order, Integer};

    use super::hash256;
    use crate::std_lib::integer_ex::IntegerEx;

    #[test]
    fn verify_a_hash256() {
        let hashed = hash256(&"A SECRET".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("64C8CC00820487EF146BC190E5664BEE0D39654A1942809316CEFD54C5DEF520");

        assert_eq!(hashed_integer, expected);
    }

    #[test]
    fn verify_empty_string_hash256() {
        let hashed = hash256(&"".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("5DF6E0E2761359D30A8275058E299FCC0381534545F55CF43E41983F5D4C9456");

        assert_eq!(hashed_integer, expected);
    }
}
