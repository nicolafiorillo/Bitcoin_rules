///! Finite field element management
use rug::{ops::RemRounding, Integer};
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Sub},
};

use crate::integer_ex::IntegerEx;

#[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldElement {
    num: Integer,
    prime: Integer,
    is_secp256k1: bool,
}

impl FieldElement {
    pub fn new(num: Integer, prime: Integer) -> FieldElement {
        if prime < 2 {
            panic!("invalid base: it must be equal or greater than 2");
        }

        FieldElement {
            num,
            prime,
            is_secp256k1: false,
        }
    }

    pub fn new_secp256k1(num: Integer) -> FieldElement {
        use crate::btc_ecdsa::P;

        FieldElement {
            num,
            prime: (*P).clone(),
            is_secp256k1: true,
        }
    }
    pub fn pow(&self, exponent: i32) -> FieldElement {
        let big_exp = Integer::from(exponent);
        let n: Integer = self.prime.clone() - 1;

        let exp = big_exp.rem_euc(n);

        let res = self.num.clone().power_modulo(&exp, &self.prime);
        FieldElement::new(res, self.prime.clone())
    }

    pub fn is_zero(&self) -> bool {
        self.num == 0
    }

    pub fn is_secp256k1(&self) -> bool {
        self.is_secp256k1
    }

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

    // Add operator
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

    // Sub operator
    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot sub two numbers in different fields");
        }

        let s = &self.num - &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());
        FieldElement::new(rem, self.prime.clone())
    }
}

impl Sub<&FieldElement> for &FieldElement {
    type Output = FieldElement;

    fn sub(self, other: &FieldElement) -> FieldElement {
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

    // Mul operator
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

    fn mul(self, other: &Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot div two numbers in different fields");
        }

        let s = &self.num * &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());
        FieldElement::new(rem, self.prime.clone())
    }
}

impl Div for FieldElement {
    type Output = Self;

    // Div operator
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

impl Mul<&FieldElement> for i32 {
    type Output = FieldElement;

    fn mul(self, other: &FieldElement) -> FieldElement {
        let s = self * &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(other.prime.clone());

        FieldElement::new(rem, other.prime.clone())
    }
}

impl Mul<FieldElement> for i32 {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        let s = self * &other.num;
        let (_q, rem) = Integer::from(s).div_rem_euc(other.prime.clone());

        FieldElement::new(rem, other.prime.clone())
    }
}

impl Mul<i32> for FieldElement {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        let s = &self.num * other;
        let (_q, rem) = Integer::from(s).div_rem_euc(self.prime.clone());

        FieldElement::new(rem, self.prime.clone())
    }
}

#[cfg(test)]
mod field_element_test {
    use crate::field_element::*;

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

        assert_eq!(field1.pow(3), field2);
    }

    #[test]
    fn exercise8_1() {
        let field1 = FieldElement::new(Integer::from(3), Integer::from(31));
        let field2 = FieldElement::new(Integer::from(24), Integer::from(31));
        let field3 = FieldElement::new(Integer::from(4), Integer::from(31));

        assert_eq!(field1 * field2.pow(-1), field3);
    }

    #[test]
    fn exercise8_2() {
        let field1 = FieldElement::new(Integer::from(17), Integer::from(31));
        let field2 = FieldElement::new(Integer::from(29), Integer::from(31));

        assert_eq!(field1.pow(-3), field2);
    }

    #[test]
    fn exercise8_3() {
        let field1 = FieldElement::new(Integer::from(4), Integer::from(31));
        let field2 = FieldElement::new(Integer::from(11), Integer::from(31));
        let field3 = FieldElement::new(Integer::from(13), Integer::from(31));

        assert_eq!(field1.pow(-4) * field2, field3);
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
            v.push(FieldElement::new(Integer::from(i), Integer::from(p)).pow(p as i32 - 1));
        }

        return v;
    }
}
