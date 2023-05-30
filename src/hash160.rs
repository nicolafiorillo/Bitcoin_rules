//use ripemd::Digest;
use ripemd::Digest;
use ripemd::Ripemd160;
use rug::{integer::Order, Integer};

pub fn hash160(s: String) -> Integer {
    let mut hasher_sha256 = sha2::Sha256::new();
    hasher_sha256.update(s);
    let hashed_sha256 = hasher_sha256.finalize();

    let mut hasher_ripemd160 = Ripemd160::new();
    hasher_ripemd160.update(hashed_sha256);
    let hashed_ripemd160 = hasher_ripemd160.finalize();

    Integer::from_digits(hashed_ripemd160.as_slice(), Order::Msf)
}

#[cfg(test)]
mod hash160_test {
    use rug::Integer;

    use crate::{hash160::hash160, integer_ex::IntegerEx};

    #[test]
    fn verify_a_hash() {
        let hashed = hash160("A SECRET".to_string());
        let expected = Integer::new_from_hex_str("bfe26c5c796d44b7091cf33e7a2fecc55c7c0278");

        assert_eq!(hashed, expected);
    }

    #[test]
    fn verify_empty_string_hash() {
        let hashed = hash160("".to_string());
        let expected = Integer::new_from_hex_str("b472a266d0bd89c13706a4132ccfb16f7c3b9fcb");

        assert_eq!(hashed, expected);
    }
}
