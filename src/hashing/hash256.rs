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

        let expected = Integer::from_hex_str("64c8cc00820487ef146bc190e5664bee0d39654a1942809316cefd54c5def520");

        assert_eq!(hashed_integer, expected);
    }

    #[test]
    fn verify_empty_string_hash256() {
        let hashed = hash256(&"".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456");

        assert_eq!(hashed_integer, expected);
    }
}
