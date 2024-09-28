use sha1::{Digest, Sha1};

pub fn sha1(s: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(s);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod hashing_test {
    use rug::{integer::Order, Integer};

    use super::sha1;
    use crate::std_lib::integer_extended::IntegerExtended;

    #[test]
    fn verify_a_sha1() {
        let hashed = sha1(&"A SECRET".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("FEB35376D723CF3A498393E87E1DBF3E3C08D800");

        assert_eq!(hashed_integer, expected);
    }

    #[test]
    fn verify_empty_string_sha1() {
        let hashed = sha1(&"".to_string().as_bytes().to_vec());
        let hashed_integer = Integer::from_digits(&hashed, Order::Msf);

        let expected = Integer::from_hex_str("DA39A3EE5E6B4B0D3255BFEF95601890AFD80709");

        assert_eq!(hashed_integer, expected);
    }
}
