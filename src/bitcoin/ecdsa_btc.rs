//! Bitcoin protocol magic numbers.

use once_cell::sync::Lazy;
use rug::{ops::Pow, Integer};

use crate::ecdsa::field_element::FieldElement;
use crate::ecdsa::point::Point;
use crate::integer_ex::IntegerEx;

// X coordinate of Generator Point as per bitcoin protocol.
pub static GX: Lazy<Integer> =
    Lazy::new(|| Integer::from_hex_str("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"));

// Y coordinate of Generator Point as per bitcoin protocol.
pub static GY: Lazy<Integer> =
    Lazy::new(|| Integer::from_hex_str("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8"));

// Prime number P as per bitcoin protocol.
pub static P: Lazy<Integer> = Lazy::new(|| Integer::from(2).pow(256) - Integer::from(2).pow(32) - 977);

pub static N: Lazy<Integer> =
    Lazy::new(|| Integer::from_hex_str("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141"));

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
    use crate::integer_ex::IntegerEx;

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
