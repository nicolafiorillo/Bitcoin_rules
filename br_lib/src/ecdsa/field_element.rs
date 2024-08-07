//! Finite field element management

use rug::{ops::RemRounding, Integer};
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Sub},
};

use crate::std_lib::integer_extended::IntegerExtended;

#[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldElement {
    /// value
    num: Integer,
    // reference prime number
    prime: Integer,
    // value is included in bitcoin field (P)
    is_in_btc_field: bool, // TODO: I do not like it here
}

impl FieldElement {
    /// New `FieldElement` with given `num` and `prime`.
    pub fn new(num: Integer, prime: Integer) -> FieldElement {
        if prime < 2 {
            panic!("invalid base: it must be equal or greater than 2");
        }

        FieldElement {
            num,
            prime,
            is_in_btc_field: false,
        }
    }

    /// New `FieldElement` with given `num` and bitcoin P as prime.
    pub fn new_in_secp256k1(num: Integer) -> FieldElement {
        use crate::bitcoin::ecdsa::P;

        // TODO: verify that `num` is less than `P` (there is a verification in Point...)
        //  but it is not enough to apply a sort of "if num < *P { error... }"
        //  because it can generate flaky tests

        FieldElement {
            num,
            prime: (*P).clone(),
            is_in_btc_field: true,
        }
    }

    /// Power operation by i32.
    pub fn pow_by_i32(&self, exponent: i32) -> FieldElement {
        let big_exp = Integer::from(exponent);
        let n: Integer = self.prime.clone() - 1;

        let exp = big_exp.rem_euc(n);

        let res = self.num.clone().power_modulo(&exp, &self.prime);
        FieldElement::new(res, self.prime.clone())
    }

    /// Power operation by Integer.
    pub fn pow_by_integer(&self, exponent: Integer) -> FieldElement {
        let n: Integer = self.prime.clone() - 1;

        let exp = exponent.rem_euc(n);

        let res = self.num.clone().power_modulo(&exp, &self.prime);
        FieldElement::new(res, self.prime.clone())
    }

    /// Square root should be available only as per bitcoin protocol.
    /// It works for `p % 4 = 3` only.
    pub fn sqrt(&self) -> FieldElement {
        self.pow_by_integer((self.prime.clone() + 1) / Integer::from(4))
    }

    /// `FieldElement` is zero.
    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    /// `FieldElement` is in bitcoin field.
    pub fn is_in_bitcoin_field(&self) -> bool {
        self.is_in_btc_field
    }

    /// Get the value as Integer.
    pub fn num(&self) -> Integer {
        self.num.clone()
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:x} <{:x}>", self.num, self.prime)
    }
}

impl Clone for FieldElement {
    fn clone(&self) -> FieldElement {
        FieldElement::new(self.num.clone(), self.prime.clone())
    }
}

impl Add for FieldElement {
    type Output = Self;

    // Add operator: `FieldElement` + `FieldElement`.
    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot add two numbers in different fields");
        }

        let s = &self.num + &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());

        FieldElement::new(rem, self.prime.clone())
    }
}

impl Sub for FieldElement {
    type Output = Self;

    // Sub operator: `FieldElement` - `FieldElement`.
    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot sub two numbers in different fields");
        }

        let s = &self.num - &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());
        FieldElement::new(rem, self.prime.clone())
    }
}

impl Sub<Self> for &FieldElement {
    type Output = FieldElement;

    // Sub operator: `&FieldElement` - `&FieldElement`.
    fn sub(self, other: Self) -> FieldElement {
        if self.prime != other.prime {
            panic!("cannot sub two numbers in different fields");
        }

        let s = &self.num - &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());
        FieldElement::new(rem, self.prime.clone())
    }
}

impl Sub<&Self> for FieldElement {
    type Output = Self;

    // Sub operator: `FieldElement` - `&FieldElement`.
    fn sub(self, other: &Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot sub two numbers in different fields");
        }

        let s = &self.num - &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());
        FieldElement::new(rem, self.prime.clone())
    }
}

impl Mul for FieldElement {
    type Output = Self;

    // Mul operator: `FieldElement` * `FieldElement`.
    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot mul two numbers in different fields");
        }

        let s = &self.num * &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());
        FieldElement::new(rem, self.prime.clone())
    }
}

impl Mul<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    // Mul operator: `&FieldElement` * `&FieldElement`.
    fn mul(self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("cannot div two numbers in different fields");
        }

        let s = &self.num * &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());
        FieldElement::new(rem, self.prime.clone())
    }
}

impl Mul<&Self> for FieldElement {
    type Output = Self;

    // Mul operator: `FieldElement` * `&FieldElement`.
    fn mul(self, other: &Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot div two numbers in different fields");
        }

        let s = &self.num * &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());
        FieldElement::new(rem, self.prime.clone())
    }
}

impl Mul<&FieldElement> for i32 {
    type Output = FieldElement;

    // Mul operator: `i32` * `&FieldElement`.
    fn mul(self, other: &FieldElement) -> FieldElement {
        let s = self * &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(other.prime.clone());

        FieldElement::new(rem, other.prime.clone())
    }
}

impl Mul<FieldElement> for i32 {
    type Output = FieldElement;

    // Mul operator: `i32` * `FieldElement`.
    fn mul(self, other: FieldElement) -> FieldElement {
        let s = self * &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(other.prime.clone());

        FieldElement::new(rem, other.prime.clone())
    }
}

impl Div for FieldElement {
    type Output = Self;

    // Div operator: `FieldElement` / `FieldElement`.
    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot div two numbers in different fields");
        }

        let prime = self.prime.clone();

        let o = other.num.power_modulo(&(prime.clone() - 2), &prime);
        let s: Integer = &self.num * o;
        let (_q, rem) = s.div_rem_euc(prime);

        FieldElement::new(rem, self.prime)
    }
}

impl Div<Self> for &FieldElement {
    type Output = FieldElement;

    // Div operator: `&FieldElement` / `&FieldElement`.
    fn div(self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("cannot div two numbers in different fields");
        }

        let o = other
            .num
            .clone()
            .power_modulo(&(self.prime.clone() - 2), &self.prime.clone());
        let s: Integer = &self.num * o;
        let (_q, rem) = s.div_rem_euc(self.prime.clone());

        FieldElement::new(rem, self.prime.clone())
    }
}

#[cfg(test)]
mod field_element_test {
    use super::*;

    #[test]
    fn fields_are_equals() {
        let field1 = FieldElement::new(Integer::from(1), Integer::from(2));
        let field2 = FieldElement::new(Integer::from(1), Integer::from(2));

        assert_eq!(field1, field2);
    }

    #[test]
    fn fields_are_different_by_num() {
        let field1 = FieldElement::new(Integer::from(1), Integer::from(2));
        let field2 = FieldElement::new(Integer::from(2), Integer::from(2));

        assert_ne!(field1, field2);
    }

    #[test]
    fn fields_are_different_by_prime() {
        let field1 = FieldElement::new(Integer::from(1), Integer::from(2));
        let field2 = FieldElement::new(Integer::from(1), Integer::from(3));

        assert_ne!(field1, field2);
    }

    #[test]
    fn adding_fields() {
        let field1 = FieldElement::new(Integer::from(7), Integer::from(13));
        let field2 = FieldElement::new(Integer::from(12), Integer::from(13));
        let field3 = FieldElement::new(Integer::from(6), Integer::from(13));

        assert_eq!(field1 + field2, field3);
    }

    #[test]
    #[should_panic(expected = "cannot add two numbers in different fields")]
    fn adding_different_fields() {
        let field1 = FieldElement::new(Integer::from(7), Integer::from(10));
        let field2 = FieldElement::new(Integer::from(12), Integer::from(13));

        let _r_ = field1 + field2;
    }

    #[test]
    fn subtracting_fields() {
        let field1 = FieldElement::new(Integer::from(76), Integer::from(13));
        let field2 = FieldElement::new(Integer::from(12), Integer::from(13));
        let field3 = FieldElement::new(Integer::from(12), Integer::from(13));

        assert_eq!(field1 - field2, field3);
    }

    #[test]
    #[should_panic(expected = "cannot sub two numbers in different fields")]
    fn subtracting_different_fields() {
        let field1 = FieldElement::new(Integer::from(76), Integer::from(10));
        let field2 = FieldElement::new(Integer::from(12), Integer::from(13));

        let _r_ = field1 - field2;
    }

    #[test]
    fn multiplying_fields() {
        let field1 = FieldElement::new(Integer::from(3), Integer::from(13));
        let field2 = FieldElement::new(Integer::from(12), Integer::from(13));
        let field3 = FieldElement::new(Integer::from(10), Integer::from(13));

        assert_eq!(field1 * field2, field3);
    }

    #[test]
    #[should_panic(expected = "cannot mul two numbers in different fields")]
    fn multiplying_different_fields() {
        let field1 = FieldElement::new(Integer::from(76), Integer::from(10));
        let field2 = FieldElement::new(Integer::from(12), Integer::from(13));

        let _r_ = field1 * field2;
    }

    #[test]
    fn dividing_fields_1() {
        let field1 = FieldElement::new(Integer::from(3), Integer::from(31));
        let field2 = FieldElement::new(Integer::from(24), Integer::from(31));
        let field3 = FieldElement::new(Integer::from(4), Integer::from(31));

        assert_eq!(field1 / field2, field3);
    }

    #[test]
    fn dividing_fields_2() {
        let field1 = FieldElement::new(Integer::from(3), Integer::from(31));
        let field2 = FieldElement::new(Integer::from(24), Integer::from(31));
        let field3 = FieldElement::new(Integer::from(4), Integer::from(31));

        assert_eq!(field1 / field2, field3);
    }

    #[test]
    #[should_panic(expected = "cannot div two numbers in different fields")]
    fn dividing_different_fields() {
        let field1 = FieldElement::new(Integer::from(76), Integer::from(10));
        let field2 = FieldElement::new(Integer::from(12), Integer::from(13));

        let _r_ = field1 / field2;
    }

    #[test]
    fn exponentiationing_fields() {
        let field1 = FieldElement::new(Integer::from(3), Integer::from(13));
        let field2 = FieldElement::new(Integer::from(1), Integer::from(13));

        assert_eq!(field1.pow_by_i32(3), field2);
    }

    #[test]
    fn exercise8_1() {
        let field1 = FieldElement::new(Integer::from(3), Integer::from(31));
        let field2 = FieldElement::new(Integer::from(24), Integer::from(31));
        let field3 = FieldElement::new(Integer::from(4), Integer::from(31));

        assert_eq!(field1 * field2.pow_by_i32(-1), field3);
    }

    #[test]
    fn exercise8_2() {
        let field1 = FieldElement::new(Integer::from(17), Integer::from(31));
        let field2 = FieldElement::new(Integer::from(29), Integer::from(31));

        assert_eq!(field1.pow_by_i32(-3), field2);
    }

    #[test]
    fn exercise8_3() {
        let field1 = FieldElement::new(Integer::from(4), Integer::from(31));
        let field2 = FieldElement::new(Integer::from(11), Integer::from(31));
        let field3 = FieldElement::new(Integer::from(13), Integer::from(31));

        assert_eq!(field1.pow_by_i32(-4) * field2, field3);
    }

    #[test]
    fn exponentiationing_a_serie_7() {
        let v = a_serie(7);
        assert_eq!(v, a_vector_of_ones(7))
    }

    #[test]
    fn exponentiationing_a_serie_11() {
        let v = a_serie(11);
        assert_eq!(v, a_vector_of_ones(11))
    }

    #[test]
    fn exponentiationing_a_serie_17() {
        let v = a_serie(17);
        assert_eq!(v, a_vector_of_ones(17))
    }

    #[test]
    fn exponentiationing_a_serie_31() {
        let v = a_serie(31);
        assert_eq!(v, a_vector_of_ones(31))
    }

    fn a_vector_of_ones(p: u32) -> Vec<FieldElement> {
        let mut v = vec![];

        for _i in 1..p {
            v.push(FieldElement::new(Integer::from(1), Integer::from(p)));
        }

        return v;
    }

    fn a_serie(p: u32) -> Vec<FieldElement> {
        let mut v = vec![];

        for i in 1..p {
            v.push(FieldElement::new(Integer::from(i), Integer::from(p)).pow_by_i32(p as i32 - 1));
        }

        return v;
    }
}
