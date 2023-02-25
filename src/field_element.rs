use std::ops::{Add, Sub};

#[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    // Create a new FieldElement
    pub fn new(num: u32, prime: u32) -> FieldElement {
        FieldElement {
            num,
            prime,
            ..Default::default()
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    // Add operator
    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot add two numbers in different fields");
        }

        let n = (self.num + other.num).rem_euclid(self.prime);
        return FieldElement::new(n, self.prime);
    }
}

impl Sub for FieldElement {
    type Output = Self;

    // Sub operator
    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("cannot sub two numbers in different fields");
        }

        let n = (self.num - other.num).rem_euclid(self.prime);
        return FieldElement::new(n, self.prime);
    }
}

#[cfg(test)]
mod field_element_test {
    use crate::field_element::*;

    #[test]
    fn fields_are_equals() {
        let field1 = FieldElement::new(1, 2);
        let field2 = FieldElement::new(1, 2);

        assert_eq!(field1, field2);
    }

    #[test]
    fn fields_are_different_by_num() {
        let field1 = FieldElement::new(1, 2);
        let field2 = FieldElement::new(2, 2);

        assert_ne!(field1, field2);
    }

    #[test]
    fn fields_are_different_by_prime() {
        let field1 = FieldElement::new(1, 2);
        let field2 = FieldElement::new(1, 3);

        assert_ne!(field1, field2);
    }

    #[test]
    fn adding_fields() {
        let field1 = FieldElement::new(7, 13);
        let field2 = FieldElement::new(12, 13);
        let field3 = FieldElement::new(6, 13);

        assert_eq!(field1 + field2, field3);
    }

    #[test]
    #[should_panic(expected = "cannot add two numbers in different fields")]
    fn adding_different_fields() {
        let field1 = FieldElement::new(7, 10);
        let field2 = FieldElement::new(12, 13);

        let _r_ = field1 + field2;
    }

    #[test]
    fn subtracting_fields() {
        let field1 = FieldElement::new(76, 13);
        let field2 = FieldElement::new(12, 13);
        let field3 = FieldElement::new(12, 13);

        assert_eq!(field1 - field2, field3);
    }

    #[test]
    #[should_panic(expected = "cannot sub two numbers in different fields")]
    fn subtracting_different_fields() {
        let field1 = FieldElement::new(76, 10);
        let field2 = FieldElement::new(12, 13);

        let _r_ = field1 - field2;
    }
}
