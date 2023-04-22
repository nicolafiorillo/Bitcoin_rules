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
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

use rug::Integer;

use crate::field_element::FieldElement;

#[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Point {
        if let (Some(x_value), Some(y_value)) = (x.clone(), y.clone()) {
            if y_value.pow(2) != x_value.clone().pow(3) + a.clone() * x_value + b.clone() {
                panic!("point is not in the curve");
            }
        }

        let point = Point {
            x,
            y,
            a,
            b,
            ..Default::default()
        };

        return point;
    }

    pub fn is_infinite(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let x = if self.x.is_none() {
            "None".to_string()
        } else {
            self.x.as_ref().unwrap().to_string()
        };

        let y = if self.y.is_none() {
            "None".to_string()
        } else {
            self.y.as_ref().unwrap().to_string()
        };

        write!(f, "{}, {} (a: {}, b: {})", x, y, self.a, self.b)
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        return Point::new(
            self.x.clone(),
            self.y.clone(),
            self.a.clone(),
            self.b.clone(),
        );
    }
}

impl<'a, 'b> Add<&'b Point> for Point {
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
            return self.clone();
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
            let x = s.clone().pow(2) - x1 - x2;
            let y = &s * &(x1 - &x) - y1;

            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        if self == other.clone()
            && self.y.is_some()
            && self.y.clone().unwrap() == (0 * self.x.clone().unwrap())
        {
            return Point::new(None, None, self.a, self.b);
        }

        if self == other.clone() {
            let y1 = &self.y.unwrap();
            let x1 = &self.x.unwrap();

            let s = (3 * x1.clone().pow(2) + self.a.clone()) / (2 * y1);
            let x = s.clone().pow(2) - (2 * x1.clone());
            let y = s * (x1 - &x) - y1;

            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        return Point::new(None, None, self.a, self.b);
    }
}

impl<'a, 'b> Mul<u32> for Point {
    type Output = Point;

    fn mul(self, other: u32) -> Point {
        if other == 0 {
            panic!("TODO: multiplication by zero not implemented");
        }

        let p: Point = self.clone();
        let mut product = p.clone();

        for _x in 1..other {
            product = product.clone() + &self;
        }

        return product;
    }
}

#[cfg(test)]
mod point_test {
    use crate::point::*;

    #[test]
    fn a_point_in_curve_1() {
        let _p = a_point(-1, -1, 5, 7, 256);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_1() {
        let _p = a_point(-1, 2, 5, 7, 256);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_2() {
        let _p = a_point(2, 4, 5, 7, 256);
    }

    #[test]
    fn a_point_in_curve_2() {
        let _p = a_point(18, 77, 5, 7, 256);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_3() {
        let _p = a_point(5, 7, 5, 7, 256);
    }

    #[test]
    fn points_are_equal() {
        let prime = 256;

        let p1 = a_point(18, 77, 5, 7, prime);
        let p2 = a_point(18, 77, 5, 7, prime);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_are_not_equal() {
        let prime = 256;

        let p1 = a_point(18, 77, 5, 7, prime);
        let p2 = a_point(-1, -1, 5, 7, prime);

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

        let p1 = a_point(-1, 0, 6, 7, prime);
        let p2 = a_point(18, 77, 5, 7, prime);

        let _r_ = p1 + &p2;
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_b() {
        let prime = 256;

        let p1 = a_point(18, 77, 5, 7, prime);
        let p2 = a_point(0, 3, 5, 9, prime);

        let _r_ = p1 + &p2;
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_both_a_and_b() {
        let prime = 256;

        let p1 = a_point(18, 77, 5, 7, prime);
        let p2 = a_point(0, 3, 6, 9, prime);

        let _r_ = p1 + &p2;
    }

    #[test]
    fn adding_infinite_x_point() {
        let prime = 256;

        let p1 = a_point_x_none(77, 5, 7, prime);
        let p2 = a_point(-1, -1, 5, 7, prime);
        let p3 = p1 + &p2;

        assert_eq!(p3, p2);
    }

    #[test]
    fn adding_same_x_and_different_y_as_in_vertical_line() {
        let prime = 256;

        let p1 = a_point(18, 77, 5, 7, prime);
        let p2 = a_point(18, -77, 5, 7, prime);
        let p3 = a_infinite_point(5, 7, prime);

        assert_eq!(p1 + &p2, p3);
    }

    #[test]
    fn adding_two_points_1() {
        let prime = 223;

        let p1 = a_point(170, 142, 0, 7, prime);
        let p2 = a_point(60, 139, 0, 7, prime);
        let p3 = a_point(220, 181, 0, 7, prime);

        assert_eq!(p1 + &p2, p3);
    }

    #[test]
    fn adding_two_points_2() {
        let prime = 223;

        let p1 = a_point(47, 71, 0, 7, prime);
        let p2 = a_point(17, 56, 0, 7, prime);
        let p3 = a_point(215, 68, 0, 7, prime);

        assert_eq!(p1 + &p2, p3);
    }

    #[test]
    fn adding_two_points_3() {
        let prime = 223;

        let p1 = a_point(143, 98, 0, 7, prime);
        let p2 = a_point(76, 66, 0, 7, prime);
        let p3 = a_point(47, 71, 0, 7, prime);

        assert_eq!(p1 + &p2, p3);
    }

    #[test]
    fn adding_two_points_4() {
        let p1 = a_point(192, 105, 0, 7, 223);
        let p2 = p1.clone() * 2;
        let p3 = a_point(49, 71, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_5() {
        let p1 = a_point(143, 98, 0, 7, 223);
        let p2 = p1.clone() * 2;
        let p3 = a_point(64, 168, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_6() {
        let p1 = a_point(47, 71, 0, 7, 223);
        let p2 = p1.clone() * 2;
        let p3 = a_point(36, 111, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_7() {
        let p1 = a_point(47, 71, 0, 7, 223);
        let p2 = p1.clone() * 4;
        let p3 = a_point(194, 51, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_8() {
        let p1 = a_point(47, 71, 0, 7, 223);
        let p2 = p1.clone() * 8;

        let p3 = a_point(116, 55, 0, 7, 223);

        assert_eq!(p2, p3);
    }

    #[test]
    fn adding_two_points_infinite() {
        let p1 = a_point(47, 71, 0, 7, 223);
        let p2 = p1.clone() * 21;

        assert!(p2.is_infinite());
    }

    #[test]
    fn find_group_order() {
        let p = a_point(15, 86, 0, 7, 223);
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

        Point::new(None, None, afe, bfe)
    }

    fn a_point(x: i32, y: i32, a: i32, b: i32, prime: u32) -> Point {
        let xfe = FieldElement::new(Integer::from(x), prime);
        let yfe = FieldElement::new(Integer::from(y), prime);
        let afe = FieldElement::new(Integer::from(a), prime);
        let bfe = FieldElement::new(Integer::from(b), prime);

        Point::new(Some(xfe), Some(yfe), afe, bfe)
    }
}
