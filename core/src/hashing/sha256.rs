use ripemd::Digest;
use sha2::Sha256;

pub fn sha256(s: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(s);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod hashing_test {
    use rug::{integer::Order, Integer};

    use super::sha256;
    use crate::std_lib::integer_extended::IntegerExtended;

    #[test]
    fn verify_a_sha256() {
        let hashed = sha256(&"A SECRET".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("013B82C9E4FE8F048A7C5BF07F4B0E6DB48D52C9C2169D855FC2153581B0F265");

        assert_eq!(hashed_integer, expected);
    }

    #[test]
    fn verify_empty_string_sha256() {
        let hashed = sha256(&"".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855");

        assert_eq!(hashed_integer, expected);
    }
}
