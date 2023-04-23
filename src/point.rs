/// 'Point' is a point i an elliptic curve.
/// Curve is expressed as in
///     'y^2 = x^3 + ax + b'
///
/// Elliptic curve used in Bitcoin's public-key cryptography is 'Secp256k1'.
///
/// See
///     https://en.bitcoin.it/wiki/Secp256k1
///     sec2-v2.pdf
///
use crate::field_element::FieldElement;
use rug::Integer;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

#[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Point {
        if let (Some(x_value), Some(y_value)) = (x.clone(), y.clone()) {
            if y_value.pow(2) != x_value.clone().pow(3) + a.clone() * x_value + b.clone() {
                panic!("point is not in the curve");
            }
        }

        Point { x, y, a, b }
    }

    pub fn new_with_numbers(x: i32, y: i32, a: i32, b: i32, prime: u32) -> Point {
        let xfe = FieldElement::new(Integer::from(x), prime);
        let yfe = FieldElement::new(Integer::from(y), prime);
        let afe = FieldElement::new(Integer::from(a), prime);
        let bfe = FieldElement::new(Integer::from(b), prime);

        Point::new(Some(xfe), Some(yfe), afe, bfe)
    }

    pub fn new_infinite(a: &FieldElement, b: &FieldElement) -> Point {
        Point::new(None, None, a.clone(), b.clone())
    }

    pub fn is_infinite(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let x = if let Some(x_val) = &self.x {
            x_val.to_string()
        } else {
            "None".to_string()
        };

        let y = if let Some(y_val) = &self.y {
            y_val.to_string()
        } else {
            "None".to_string()
        };

        write!(f, "{}, {} (a: {}, b: {})", x, y, self.a, self.b)
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point::new(self.x.clone(), self.y.clone(), self.a.clone(), self.b.clone())
    }
}

impl Add<&Self> for Point {
    type Output = Self;

    // Add operator
    fn add(self, other: &Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("points are not in the same curve");
        }

        if self.x.is_none() {
            return other.clone();
        }

        if other.x.is_none() {
            return self;
        }

        if self.x == other.x && self.y != other.y {
            return Point::new(None, None, self.a, self.b);
        }

        if self.x != other.x {
            let y2 = &other.y.clone().unwrap();
            let y1 = &self.y.unwrap();
            let x1 = &self.x.unwrap();
            let x2 = &other.x.clone().unwrap();

            let s = (y2 - y1) / (x2 - x1);
            let x = s.pow(2) - x1 - x2;
            let y = &s * &(x1 - &x) - y1;

            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        if let Some(y_value) = self.y.clone() {
            if self == other.clone() && y_value.is_zero() {
                return Point::new(None, None, self.a, self.b);
            }
        }

        if self == other.clone() {
            let y1 = &self.y.unwrap();
            let x1 = &self.x.unwrap();

            let s = (3 * x1.clone().pow(2) + self.a.clone()) / (2 * y1);
            let x = s.pow(2) - (2 * x1.clone());
            let y = s * (x1 - &x) - y1;

            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        Point::new_infinite(&self.a, &self.b)
    }
}

impl Mul<u32> for &Point {
    type Output = Point;

    fn mul(self, coefficient: u32) -> Point {
        if coefficient == 0 {
            panic!("TODO: multiplication by zero not implemented");
        }

        let mut sel: Point = self.clone();

        let mut coef = coefficient;
        let mut result = Point::new_infinite(&self.a, &self.b);

        while coef > 0 {
            if coef & 1 > 0 {
                result = result + &sel;
            }
            sel = sel.clone() + &sel;

            coef >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod point_test {
    use crate::point::*;

    #[test]
    fn a_point_in_curve_1() {
        let _p = Point::new_with_numbers(-1, -1, 5, 7, 256);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_1() {
        let _p = Point::new_with_numbers(-1, 2, 5, 7, 256);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_2() {
        let _p = Point::new_with_numbers(2, 4, 5, 7, 256);
    }

    #[test]
    fn a_point_in_curve_2() {
        let _p = Point::new_with_numbers(18, 77, 5, 7, 256);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_3() {
        let _p = Point::new_with_numbers(5, 7, 5, 7, 256);
    }

    #[test]
    fn points_are_equal() {
        let prime = 256;

        let p1 = Point::new_with_numbers(18, 77, 5, 7, prime);
        let p2 = Point::new_with_numbers(18, 77, 5, 7, prime);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_are_not_equal() {
        let prime = 256;

        let p1 = Point::new_with_numbers(18, 77, 5, 7, prime);
        let p2 = Point::new_with_numbers(-1, -1, 5, 7, prime);

        assert_ne!(p1, p2);
    }

    #[test]
    fn points_with_x_inf_are_equal() {
        let prime = 256;

        let p1 = a_point_x_none(77, 5, 7, prime);
        let p2 = a_point_x_none(77, 5, 7, prime);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_with_y_inf_are_equal() {
        let prime = 256;

        let p1 = a_point_y_none(18, 5, 7, prime);
        let p2 = a_point_y_none(18, 5, 7, prime);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_with_both_x_and_y_inf_are_equal() {
        let prime = 256;

        let p1 = a_infinite_point(5, 7, prime);
        let p2 = a_infinite_point(5, 7, prime);

        assert_eq!(p1, p2);
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_a() {
        let prime = 256;

        let p1 = Point::new_with_numbers(-1, 0, 6, 7, prime);
        let p2 = Point::new_with_numbers(18, 77, 5, 7, prime);

        let _r_ = p1 + &p2;
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_b() {
        let prime = 256;

        let p1 = Point::new_with_numbers(18, 77, 5, 7, prime);
        let p2 = Point::new_with_numbers(0, 3, 5, 9, prime);

        let _r_ = p1 + &p2;
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_both_a_and_b() {
        let prime = 256;

        let p1 = Point::new_with_numbers(18, 77, 5, 7, prime);
        let p2 = Point::new_with_numbers(0, 3, 6, 9, prime);

        let _r_ = p1 + &p2;
    }

    #[test]
    fn adding_infinite_x_point() {
        let prime = 256;

        let p1 = a_point_x_none(77, 5, 7, prime);
        let p2 = Point::new_with_numbers(-1, -1, 5, 7, prime);
        let p3 = p1 + &p2;

        assert_eq!(p3, p2);
    }

    #[test]
    fn adding_same_x_and_different_y_as_in_vertical_line() {
        let prime = 256;

        let p1 = Point::new_with_numbers(18, 77, 5, 7, prime);
        let p2 = Point::new_with_numbers(18, -77, 5, 7, prime);
        let p3 = a_infinite_point(5, 7, prime);

        assert_eq!(p1 + &p2, p3);
    }

    #[test]
    fn adding_two_points_1() {
        let prime = 223;

        let p1 = Point::new_with_numbers(170, 142, 0, 7, prime);
        let p2 = Point::new_with_numbers(60, 139, 0, 7, prime);
        let p3 = Point::new_with_numbers(220, 181, 0, 7, prime);

        assert_eq!(p1 + &p2, p3);
    }

    #[test]
    fn adding_two_points_2() {
        let prime = 223;

        let p1 = Point::new_with_numbers(47, 71, 0, 7, prime);
        let p2 = Point::new_with_numbers(17, 56, 0, 7, prime);
        let p3 = Point::new_with_numbers(215, 68, 0, 7, prime);

        assert_eq!(p1 + &p2, p3);
    }

    #[test]
    fn adding_two_points_3() {
        let prime = 223;

        let p1 = Point::new_with_numbers(143, 98, 0, 7, prime);
        let p2 = Point::new_with_numbers(76, 66, 0, 7, prime);
        let p3 = Point::new_with_numbers(47, 71, 0, 7, prime);

        assert_eq!(p1 + &p2, p3);
    }

    #[test]
    fn adding_two_points_4() {
        let p1 = Point::new_with_numbers(192, 105, 0, 7, 223);
        let p2 = &p1 * 2;
        let p3 = Point::new_with_numbers(49, 71, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_5() {
        let p1 = Point::new_with_numbers(143, 98, 0, 7, 223);
        let p2 = &p1 * 2;
        let p3 = Point::new_with_numbers(64, 168, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_6() {
        let p1 = Point::new_with_numbers(47, 71, 0, 7, 223);
        let p2 = &p1 * 2;
        let p3 = Point::new_with_numbers(36, 111, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_7() {
        let p1 = Point::new_with_numbers(47, 71, 0, 7, 223);
        let p2 = &p1 * 4;
        let p3 = Point::new_with_numbers(194, 51, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_8() {
        let p1 = Point::new_with_numbers(47, 71, 0, 7, 223);
        let p2 = &p1 * 8;

        let p3 = Point::new_with_numbers(116, 55, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_infinite() {
        let p1 = Point::new_with_numbers(47, 71, 0, 7, 223);
        let p2 = &p1 * 21;

        assert!(p2.is_infinite());
    }

    #[test]
    fn find_group_order() {
        let p = Point::new_with_numbers(15, 86, 0, 7, 223);
        let mut product = p.clone();

        let mut n = 1;
        while !product.is_infinite() {
            product = product.clone() + &p;
            n += 1;
        }

        assert_eq!(n, 7);
    }

    fn a_point_x_none(y: i32, a: i32, b: i32, prime: u32) -> Point {
        let yfe = FieldElement::new(Integer::from(y), prime);
        let afe = FieldElement::new(Integer::from(a), prime);
        let bfe = FieldElement::new(Integer::from(b), prime);

        Point::new(None, Some(yfe), afe, bfe)
    }

    fn a_point_y_none(x: i32, a: i32, b: i32, prime: u32) -> Point {
        let xfe = FieldElement::new(Integer::from(x), prime);
        let afe = FieldElement::new(Integer::from(a), prime);
        let bfe = FieldElement::new(Integer::from(b), prime);

        Point::new(Some(xfe), None, afe, bfe)
    }

    fn a_infinite_point(a: i32, b: i32, prime: u32) -> Point {
        let afe = FieldElement::new(Integer::from(a), prime);
        let bfe = FieldElement::new(Integer::from(b), prime);

        Point::new_infinite(&afe, &bfe)
    }
}
