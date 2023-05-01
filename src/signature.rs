use std::fmt::{Display, Formatter, Result};

use rug::Integer;

pub struct Signature {
    pub r: Integer,
    pub s: Integer,
}

impl Signature {
    pub fn new(r: Integer, s: Integer) -> Signature {
        Signature { r, s }
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Signature({:x}, {:x})", self.r, self.s)
    }
}

#[cfg(test)]
mod signature_test {
    use rug::Integer;

    use crate::{btc_ecdsa::*, field_element::FieldElement, hash256::integer, point::Point};

    #[test]
    fn a_signature() {
        let z = integer(
            0xbc62d4b80d9e36da,
            0x29c16c5d4d9f1173,
            0x1f36052c72401a76,
            0xc23c0fb5a9b74423,
        );
        let r = integer(
            0x37206a0610995c58,
            0x074999cb9767b87a,
            0xf4c4978db68c06e8,
            0xe6e81d282047a7c6,
        );
        let s = integer(
            0x8ca63759c1157ebe,
            0xaec0d03cecca119f,
            0xc9a75bf8e6d0fa65,
            0xc841c8e2738cdaec,
        );
        let px = integer(
            0x04519fac3d910ca7,
            0xe7138f7013706f61,
            0x9fa8f033e6ec6e09,
            0x370ea38cee6a7574,
        );
        let py = integer(
            0x82b51eab8c27c66e,
            0x26c858a079bcdf4f,
            0x1ada34cec420cafc,
            0x7eac1a42216fb6c4,
        );

        let ppx = FieldElement::new(px, (*P).clone());
        let ppy = FieldElement::new(py, (*P).clone());
        let point = Point::new_secp256k1(Some(ppx), Some(ppy));

        let s_inv = match s.pow_mod(&((*N).clone() - 2), &(*N).clone()) {
            Ok(power) => power,
            Err(_) => unreachable!(),
        };

        let mu = &z * &s_inv;
        let (_q, u) = Integer::from(mu).div_rem_euc((*N).clone());

        let mv = &r * &s_inv;
        let (_q, v) = Integer::from(mv).div_rem_euc((*N).clone());

        let left = (&(*G).clone() * u) + &(&point * v);
        assert_eq!(left.x_as_num(), r);
    }
}
