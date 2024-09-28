use rug::{integer::Order, Integer};
use std::fmt::{Display, Formatter};

use crate::std_lib::{std_result::StdResult, vector::trim_left};

#[derive(Debug)]
pub struct Signature {
    pub r: Integer,
    pub s: Integer,
}

const DER_MARKER: u8 = 0x30;
const DER_INTEGER: u8 = 0x02;

/*
    https://github.com/bitcoin/bips/blob/master/bip-0066.mediawiki
    https://github.com/libbitcoin/libbitcoin-system/wiki/ECDSA-and-DER-Signatures
*/
impl Signature {
    pub fn new(r: Integer, s: Integer) -> Signature {
        Signature { r, s }
    }

    pub fn new_from_der(der: Vec<u8>) -> StdResult<Self> {
        let der_length = der.len();

        if !(9..=73).contains(&der_length) {
            Err("invalid_signature_length")?;
        }

        if der[0] != DER_MARKER {
            Err("invalid_initial_marker")?;
        }

        if der[1] != (der_length - 2) as u8 {
            Err("signature_lengths_do_not_match")?;
        }

        if der[2] != DER_INTEGER {
            Err("invalid_r_marker")?;
        }

        let (r, next) = Self::der_deserialize(&der, 3)?;

        if next >= der_length {
            Err("missing_s_marker")?;
        }

        if der[next] != DER_INTEGER {
            Err("invalid_s_marker")?;
        }

        let (s, _next) = Self::der_deserialize(&der, next + 1)?;

        Ok(Signature { r, s })
    }

    pub fn der(&self) -> Vec<u8> {
        let r_value = Self::der_serialize(&self.r);
        let s_value = Self::der_serialize(&self.s);

        let mut result = r_value;
        result.extend(s_value);

        let mut res: Vec<u8> = vec![DER_MARKER, result.len() as u8];
        res.extend(result);

        res
    }

    fn der_serialize(value: &Integer) -> Vec<u8> {
        let mut v: Vec<u8> = value.to_digits::<u8>(Order::Msf);
        v = trim_left(&v, 0);

        if (v[0] & 0x80) == 0x80 {
            v.insert(0, 0);
        }

        let mut res: Vec<u8> = vec![DER_INTEGER, v.len() as u8];
        res.extend(v);

        res
    }

    pub fn der_deserialize(der: &[u8], start: usize) -> StdResult<(Integer, usize)> {
        if start >= der.len() {
            Err("invalid_rs_start")?;
        }

        let length = der[start] as usize;

        if length == 0 {
            Err("invalid_rs_lenght")?;
        }

        let content_start = start + 1;
        let content_end = content_start + length;

        let bytes = der[content_start..content_end].to_vec();

        /*
            Removing checking on negative r/s for DER values.

            if (bytes[0] & 0x80) == 0x80 {
                return Err(DerError::NegativeRS);
            }
        */

        Ok((Integer::from_digits(&bytes, Order::Msf), content_end))
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signature({:x}, {:x})", self.r, self.s)
    }
}

#[cfg(test)]
mod signature_test {
    use rug::{integer::Order, Integer};

    use super::*;
    use crate::{
        bitcoin::ecdsa::{G, N, P},
        ecdsa::{field_element::FieldElement, point::Point},
        std_lib::{integer_extended::IntegerExtended, vector::hex_string_to_bytes},
    };

    #[test]
    fn a_signature() {
        let z = Integer::from_hex_str("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423");
        let r = Integer::from_hex_str("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let s = Integer::from_hex_str("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
        let px = Integer::from_hex_str("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574");
        let py = Integer::from_hex_str("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4");

        let ppx = FieldElement::new(px, (*P).clone());
        let ppy = FieldElement::new(py, (*P).clone());
        let point = Point::new_in_secp256k1(Some(ppx), Some(ppy));

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
        let r = Integer::from_hex_str("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let s = Integer::from_hex_str("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");

        let sig = Signature::new(r, s);
        let der = sig.der();

        let res = Integer::from_digits(&der, Order::Msf);

        let expected =
            Integer::from_hex_str("3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
        assert_eq!(expected, res);
    }

    #[test]
    fn deserialize_a_der_signature_invalid_length_less_than_9() {
        let der = hex_string_to_bytes("3045022037206a06").unwrap();

        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_signature_length"
        );
    }

    #[test]
    fn deserialize_a_der_signature_invalid_length_more_than_73() {
        let der = hex_string_to_bytes("3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec000000").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_signature_length"
        );
    }

    #[test]
    fn deserialize_a_der_signature_invalid_length_more_than_73_2() {
        let der = hex_string_to_bytes("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_signature_length"
        );
    }

    #[test]
    fn deserialize_a_der_signature_invalid_initial_marker() {
        let der = hex_string_to_bytes("3145022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_initial_marker"
        );
    }

    #[test]
    fn deserialize_a_der_signature_missing_s() {
        let der =
            hex_string_to_bytes("302202200000000000000000000000000000000000000000000000000000000000000000").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "missing_s_marker"
        );
    }

    #[test]
    fn deserialize_a_der_signature_invalid_r_marker() {
        let der = hex_string_to_bytes("3045012037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_r_marker"
        );
    }

    #[test]
    fn deserialize_a_der_signature_invalid_r_length() {
        let der = hex_string_to_bytes("30140200021077777777777777777777777777777777").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_rs_lenght"
        );
    }

    #[test]
    fn deserialize_a_der_signature_invalid_s_marker() {
        let der = hex_string_to_bytes("3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60121008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_s_marker"
        );
    }

    #[test]
    fn deserialize_a_der_signature_invalid_s_length() {
        let der = hex_string_to_bytes("30140210777777777777777777777777777777770200").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_rs_lenght"
        );
    }

    /*
       Removing test on negative r/s for DER values.

    #[test]
    fn deserialize_a_der_signature_invalid_negative_r() {
        let der =
            string_to_bytes("3024021087777777777777777777777777777777021077777777777777777777777777777777").unwrap();
        assert_eq!(Signature::new_from_der(der).err().unwrap(), DerError::NegativeRS);
    }

    #[test]
    fn deserialize_a_der_signature_invalid_negative_s() {
        let der =
            string_to_bytes("3024021077777777777777777777777777777777021087777777777777777777777777777777").unwrap();
        assert_eq!(Signature::new_from_der(der).err().unwrap(), DerError::NegativeRS);
    }
    */

    #[test]
    fn deserialize_a_der_signature_invalid_s_start() {
        let der = hex_string_to_bytes("301302107777777777777777777777777777777702").unwrap();
        assert_eq!(
            Signature::new_from_der(der).err().unwrap().to_string(),
            "invalid_rs_start"
        );
    }

    #[test]
    fn deserialize_a_der_signature_1() {
        let der = hex_string_to_bytes("3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec").unwrap();
        let sig = Signature::new_from_der(der).unwrap();

        let expected_r = Integer::from_hex_str("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let expected_s = Integer::from_hex_str("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");

        assert_eq!(expected_r, sig.r);
        assert_eq!(expected_s, sig.s);
    }

    #[test]
    fn deserialize_a_der_signature_2() {
        let der = hex_string_to_bytes("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap();
        let sig = Signature::new_from_der(der).unwrap();

        let expected_r = Integer::from_hex_str("00eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c");
        let expected_s = Integer::from_hex_str("00c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6");

        assert_eq!(expected_r, sig.r);
        assert_eq!(expected_s, sig.s);
    }

    #[test]
    fn deserialize_a_der_signature_3() {
        let der = hex_string_to_bytes("3024021077777777777777777777777777777777021077777777777777777777777777777777")
            .unwrap();
        assert!(Signature::new_from_der(der).is_ok());
    }

    #[test]
    fn deserialize_a_der_signature_4() {
        let der =
            hex_string_to_bytes("304402200060558477337b9022e70534f1fea71a318caf836812465a2509931c5e7c4987022078ec32bd50ac9e03a349ba953dfd9fe1c8d2dd8bdb1d38ddca844d3d5c78c118").unwrap();
        let sig = Signature::new_from_der(der).unwrap();

        let expected_r = Integer::from_hex_str("0060558477337b9022e70534f1fea71a318caf836812465a2509931c5e7c4987");
        let expected_s = Integer::from_hex_str("78ec32bd50ac9e03a349ba953dfd9fe1c8d2dd8bdb1d38ddca844d3d5c78c118");

        assert_eq!(expected_r, sig.r);
        assert_eq!(expected_s, sig.s);
    }

    #[test]
    fn deserialize_a_der_signature_5() {
        let der =
            hex_string_to_bytes("304502202de8c03fc525285c9c535631019a5f2af7c6454fa9eb392a3756a4917c420edd02210046130bf2baf7cfc065067c8b9e33a066d9c15edcea9feb0ca2d233e3597925b4").unwrap();
        let sig = Signature::new_from_der(der).unwrap();

        let expected_r = Integer::from_hex_str("2de8c03fc525285c9c535631019a5f2af7c6454fa9eb392a3756a4917c420edd");
        let expected_s = Integer::from_hex_str("0046130bf2baf7cfc065067c8b9e33a066d9c15edcea9feb0ca2d233e3597925b4");

        assert_eq!(expected_r, sig.r);
        assert_eq!(expected_s, sig.s);
    }
}
