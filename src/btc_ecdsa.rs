use once_cell::sync::Lazy;
use rug::{integer::Order, ops::Pow, Integer};

use crate::{field_element::FieldElement, point::Point};

pub static GX: Lazy<Integer> = Lazy::new(|| {
    integer(
        0x79be667ef9dcbbac,
        0x55a06295ce870b07,
        0x029bfcdb2dce28d9,
        0x59f2815b16f81798,
    )
});

pub static GY: Lazy<Integer> = Lazy::new(|| {
    integer(
        0x483ada7726a3c465,
        0x5da4fbfc0e1108a8,
        0xfd17b448a6855419,
        0x9c47d08ffb10d4b8,
    )
});

pub static P: Lazy<Integer> = Lazy::new(|| Integer::from(2).pow(256) - Integer::from(2).pow(32) - 977);

pub static N: Lazy<Integer> = Lazy::new(|| {
    integer(
        0xffffffffffffffff,
        0xfffffffffffffffe,
        0xbaaedce6af48a03b,
        0xbfd25e8cd0364141,
    )
});

pub static G: Lazy<Point> = Lazy::new(|| {
    let x = FieldElement::new((*GX).clone(), (*P).clone());
    let y = FieldElement::new((*GY).clone(), (*P).clone());

    let zero = FieldElement::new(Integer::from(0), (*P).clone());
    let seven = FieldElement::new(Integer::from(7), (*P).clone());

    Point::new(Some(x), Some(y), zero, seven)
});

pub fn integer(ll: u64, lr: u64, rl: u64, rr: u64) -> Integer {
    let digits: [u64; 4] = [ll, lr, rl, rr];
    Integer::from_digits(&digits, Order::Msf)
}

#[cfg(test)]
mod s256_test {
    use crate::btc_ecdsa::*;
    use crate::field_element::*;
    use crate::point::Point;

    #[test]
    fn on_correct_secp256k1_numbers() {
        let left = match (*GY).clone().pow_mod(&Integer::from(2), &(*P).clone()) {
            Ok(left) => left,
            Err(_) => unreachable!(),
        };

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
