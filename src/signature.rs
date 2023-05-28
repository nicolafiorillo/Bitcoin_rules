use std::fmt::{Display, Formatter, Result};
use std::str;

use rug::{integer::Order, Integer};

use crate::helper::vector::{string_to_bytes, trim_left};

pub struct Signature {
    pub r: Integer,
    pub s: Integer,
}

impl Signature {
    pub fn new(r: Integer, s: Integer) -> Signature {
        Signature { r, s }
    }

    pub fn new_from_der(der: &str) -> Self {
        let der_vect = string_to_bytes(der);
        println!("der_vect: {:?}", der_vect);

        if der_vect.len() < 70 || der_vect.len() > 72 {
            panic!("invalid DER signature lenght: {:?}", der_vect.len()); // TODO: to log
        }

        if der_vect[0] != 0x30 {
            panic!("invalid DER signature initial marker"); // TODO: to log
        }

        if der_vect[1] != (der_vect.len() - 2) as u8 {
            panic!("DER signature length does not match"); // TODO: to log
        }

        if der_vect[2] != 0x02 {
            panic!("invalid DER signature r marker"); // TODO: to log
        }

        let (r, next) = Self::der_deserialize(&der_vect, 3);

        if der_vect[next] != 0x02 {
            panic!("invalid DER signature s marker"); // TODO: to log
        }

        let (s, _next) = Self::der_deserialize(&der_vect, next + 1);

        Signature { r, s }
    }

    pub fn der(&self) -> Vec<u8> {
        let r_value = Self::der_serialize(&self.r);
        let s_value = Self::der_serialize(&self.s);

        let mut result = r_value;
        result.extend(s_value);

        let mut res: Vec<u8> = vec![0x30, result.len() as u8];
        res.extend(result);

        res
    }

    fn der_serialize(value: &Integer) -> Vec<u8> {
        let mut v: Vec<u8> = value.to_digits::<u8>(Order::Msf);
        v = trim_left(&v, 0);

        if (v[0] & 0x80) > 0 {
            v.insert(0, 0);
        }

        let mut res: Vec<u8> = vec![0x02, v.len() as u8];
        res.extend(v);

        res
    }

    fn der_deserialize(der: &[u8], start: usize) -> (Integer, usize) {
        let lenght = der[start] as usize;

        let content_start = start + 1;
        let content_end = content_start + lenght;

        let bytes = der[content_start..content_end].to_vec();

        (Integer::from_digits(&bytes, Order::Msf), content_end)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Signature({:x}, {:x})", self.r, self.s)
    }
}

#[cfg(test)]
mod signature_test {
    use rug::{integer::Order, Integer};

    use crate::{btc_ecdsa::*, field_element::FieldElement, integer_ex::IntegerEx, point::Point, signature::Signature};

    #[test]
    fn a_signature() {
        let z = Integer::new_from_hex_str("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423");
        let r = Integer::new_from_hex_str("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let s = Integer::new_from_hex_str("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
        let px = Integer::new_from_hex_str("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574");
        let py = Integer::new_from_hex_str("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4");

        let ppx = FieldElement::new(px, (*P).clone());
        let ppy = FieldElement::new(py, (*P).clone());
        let point = Point::new_in_btc(Some(ppx), Some(ppy));

        let s_inv = s.invert_by_modulo(&N);

        let mu = &z * &s_inv;
        let (_q, u) = Integer::from(mu).div_rem_euc((*N).clone());

        let mv = &r * &s_inv;
        let (_q, v) = Integer::from(mv).div_rem_euc((*N).clone());

        let left = (&(*G).clone() * u) + &(&point * v);
        assert_eq!(left.x_as_num(), r);
    }

    #[test]
    fn serialize_a_der_signature() {
        let r = Integer::new_from_hex_str("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let s = Integer::new_from_hex_str("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");

        let sig = Signature::new(r, s);
        let der = sig.der();

        let res = Integer::from_digits(&der, Order::Msf);

        let expected =
            Integer::new_from_hex_str("3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
        assert_eq!(expected, res);
    }

    #[test]
    #[should_panic(expected = "invalid DER signature lenght")]
    fn deserialize_a_der_signature_invalid_lenght_less_than_70() {
        let der = "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738c";
        Signature::new_from_der(der);
    }

    #[test]
    #[should_panic(expected = "invalid DER signature lenght")]
    fn deserialize_a_der_signature_invalid_lenght_more_than_72() {
        let der = "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec0000";
        Signature::new_from_der(der);
    }

    #[test]
    #[should_panic(expected = "invalid DER signature initial marker")]
    fn deserialize_a_der_signature_invalid_initial_marker() {
        let der = "3145022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
        Signature::new_from_der(der);
    }

    #[test]
    #[should_panic(expected = "invalid DER signature r marker")]
    fn deserialize_a_der_signature_invalid_r_marker() {
        let der = "3045012037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
        Signature::new_from_der(der);
    }

    #[test]
    #[should_panic(expected = "invalid DER signature s marker")]
    fn deserialize_a_der_signature_invalid_s_marker() {
        let der = "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60121008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
        Signature::new_from_der(der);
    }

    #[test]
    fn deserialize_a_der_signature() {
        let der = "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec";
        let sig = Signature::new_from_der(der);

        let expected_r = Integer::new_from_hex_str("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let expected_s = Integer::new_from_hex_str("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");

        assert_eq!(expected_r, sig.r);
        assert_eq!(expected_s, sig.s);
    }
}
