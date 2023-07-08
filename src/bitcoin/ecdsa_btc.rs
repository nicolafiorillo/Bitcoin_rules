//! Bitcoin ECDSA constants from 'secp256k1': https://en.bitcoin.it/wiki/Secp256k1
//!
/// Generic elliptic curve is expressed as in
///     `y^2 = x^3 + ax + b`
///
/// Elliptic curve used in Bitcoin's public-key cryptography is 'secp256k1' (a = 0, b = 7).
///     `y^2 = x^3 + 7` in (Fp)
/// which means
///    `y^2 mod p = (x^3 + 7) mod p`
/// where Fp is a prime field and p = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1
///
/// G is the generator point (Gx, Gy) where
///   Gx = 0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798
///   Gy = 0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8
///
/// N is the order of G
///  N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
///
/// See
///     https://en.bitcoin.it/wiki/Secp256k1
///     sec2-v2.pdf
///
use once_cell::sync::Lazy;
use rug::{ops::Pow, Integer};

use crate::ecdsa::field_element::FieldElement;
use crate::ecdsa::point::Point;
use crate::low::integer_ex::IntegerEx;

// X coordinate of Generator Point as per bitcoin protocol.
pub static GX: Lazy<Integer> =
    Lazy::new(|| Integer::from_hex_str("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798"));

// Y coordinate of Generator Point as per bitcoin protocol.
pub static GY: Lazy<Integer> =
    Lazy::new(|| Integer::from_hex_str("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8"));

// Prime number P as per bitcoin protocol.
// p = 2^256 - 2^32 - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 - 1 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
pub static P: Lazy<Integer> = Lazy::new(|| {
    Integer::from(2).pow(256)
        - Integer::from(2).pow(32)
        - Integer::from(2).pow(9)
        - Integer::from(2).pow(8)
        - Integer::from(2).pow(7)
        - Integer::from(2).pow(6)
        - Integer::from(2).pow(4)
        - 1
});

pub static N: Lazy<Integer> =
    Lazy::new(|| Integer::from_hex_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141"));

pub static A: Lazy<Integer> = Lazy::new(|| Integer::from(0));
pub static B: Lazy<Integer> = Lazy::new(|| Integer::from(7));

// Generator Point as per bitcoin protocol.
pub static G: Lazy<Point> = Lazy::new(|| {
    let x = FieldElement::new((*GX).clone(), (*P).clone());
    let y = FieldElement::new((*GY).clone(), (*P).clone());

    let zero = FieldElement::new((*A).clone(), (*P).clone());
    let seven = FieldElement::new((*B).clone(), (*P).clone());

    Point::new(Some(x), Some(y), zero, seven)
});

#[cfg(test)]
mod s256_test {
    use super::*;
    use crate::ecdsa::field_element::*;
    use crate::low::integer_ex::IntegerEx;

    #[test]
    fn on_correct_secp256k1_numbers() {
        let left = (*GY).clone().power_modulo(&Integer::from(2), &(*P).clone());

        let r: Integer = (*GX).clone().pow(3) + 7;
        let (_q, right) = r.div_rem_euc((*P).clone());

        assert_eq!(left, right);
    }

    #[test]
    fn a_secp256k1() {
        let x = FieldElement::new((*GX).clone(), (*P).clone());
        let y = FieldElement::new((*GY).clone(), (*P).clone());

        let zero = FieldElement::new(Integer::from(0), (*P).clone());
        let seven = FieldElement::new(Integer::from(7), (*P).clone());

        let g = &Point::new(Some(x), Some(y), zero, seven) * (*N).clone();

        assert!(g.is_infinite());
    }
}
