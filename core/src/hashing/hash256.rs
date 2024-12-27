use ripemd::Digest;
use sha2::Sha256;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hash256(pub [u8; 32]);

impl Hash256 {
    pub fn calc(s: &[u8]) -> Hash256 {
        let mut hasher = Sha256::new();
        hasher.update(s);

        let hashed = hasher.finalize();

        hasher = Sha256::new();
        hasher.update(hashed.as_slice());

        let r = hasher.finalize();
        Hash256(r.into())
    }

    pub fn new(s: [u8; 32]) -> Hash256 {
        Hash256(s)
    }
}

impl Hash256 {
    pub fn zero() -> Self {
        Hash256([0; 32])
    }
}

impl From<Hash256> for [u8; 4] {
    fn from(v: Hash256) -> Self {
        v.0[..4].try_into().unwrap()
    }
}

impl From<Hash256> for Vec<u8> {
    fn from(v: Hash256) -> Self {
        v.0.to_vec()
    }
}

#[cfg(test)]
mod hashing_test {
    use rug::{integer::Order, Integer};

    use crate::{hashing::hash256::Hash256, std_lib::integer_extended::IntegerExtended};

    #[test]
    fn verify_a_hash256() {
        let hashed = Hash256::calc(&"A SECRET".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed.0, Order::Msf);

        let expected = Integer::from_hex_str("64C8CC00820487EF146BC190E5664BEE0D39654A1942809316CEFD54C5DEF520");

        assert_eq!(hashed_integer, expected);
    }

    #[test]
    fn verify_empty_string_hash256() {
        let hashed = Hash256::calc(&"".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed.0, Order::Msf);

        let expected = Integer::from_hex_str("5DF6E0E2761359D30A8275058E299FCC0381534545F55CF43E41983F5D4C9456");

        assert_eq!(hashed_integer, expected);
    }
}
