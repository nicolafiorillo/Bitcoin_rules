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
    ops::Add,
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
            self.x.as_ref().unwrap().to_string()
        };

        write!(f, "{}, {} <a: {}, b: {}>", x, y, self.a, self.b)
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

impl Add for Point {
    type Output = Self;

    // Add operator
    fn add(self, other: Self) -> Self {
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
            let y2 = &other.y.unwrap();
            let y1 = &self.y.unwrap();
            let x1 = &self.x.unwrap();
            let x2 = &other.x.unwrap();

            let s = (y2 - y1) / (x2 - x1);
            let x = s.clone().pow(2) - x1 - x2;
            let y = &s * &(x1 - &x) - y1;

            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        if self == other
            && self.y.is_some()
            && self.y.clone().unwrap() == (0 * self.x.clone().unwrap())
        {
            return Point::new(None, None, self.a, self.b);
        }

        if self == other {
            let y1 = &self.y.unwrap();
            let x1 = &self.x.unwrap();

            let s = (3 * x1.clone().pow(2) + self.a.clone()) / (2 * y1);
            let x = s.clone().pow(2) - (2 * x1.clone());
            let y = s * (x1 - &x) - y1;

            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        return Point::new(
            None,
            None,
            FieldElement::new(Integer::from(0), 0),
            FieldElement::new(Integer::from(0), 0),
        );
        //        return Point::new(None, None, self.a, self.b);
    }
}

#[cfg(test)]
mod point_test {
    use crate::point::*;

    #[test]
    fn a_point_in_curve_1() {
        let prime = 256;

        let x = FieldElement::new(Integer::from(-1), prime);
        let y = FieldElement::new(Integer::from(-1), prime);
        let a = FieldElement::new(Integer::from(5), prime);
        let b = FieldElement::new(Integer::from(7), prime);

        Point::new(Some(x), Some(y), a, b);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_1() {
        let prime = 256;

        let x = FieldElement::new(Integer::from(-1), prime);
        let y = FieldElement::new(Integer::from(-2), prime);
        let a = FieldElement::new(Integer::from(5), prime);
        let b = FieldElement::new(Integer::from(7), prime);

        Point::new(Some(x), Some(y), a, b);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_2() {
        let prime = 256;

        let x = FieldElement::new(Integer::from(2), prime);
        let y = FieldElement::new(Integer::from(4), prime);
        let a = FieldElement::new(Integer::from(5), prime);
        let b = FieldElement::new(Integer::from(7), prime);

        Point::new(Some(x), Some(y), a, b);
    }

    #[test]
    fn a_point_in_curve_2() {
        let prime = 256;

        let x = FieldElement::new(Integer::from(18), prime);
        let y = FieldElement::new(Integer::from(77), prime);
        let a = FieldElement::new(Integer::from(5), prime);
        let b = FieldElement::new(Integer::from(7), prime);

        Point::new(Some(x), Some(y), a, b);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_3() {
        let prime = 256;

        let x = FieldElement::new(Integer::from(5), prime);
        let y = FieldElement::new(Integer::from(7), prime);
        let a = FieldElement::new(Integer::from(5), prime);
        let b = FieldElement::new(Integer::from(7), prime);

        Point::new(Some(x), Some(y), a, b);
    }

    #[test]
    fn points_are_equal() {
        let prime = 256;

        let x1 = FieldElement::new(Integer::from(18), prime);
        let y1 = FieldElement::new(Integer::from(77), prime);
        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(18), prime);
        let y2 = FieldElement::new(Integer::from(77), prime);
        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_are_not_equal() {
        let prime = 256;

        let x1 = FieldElement::new(Integer::from(18), prime);
        let y1 = FieldElement::new(Integer::from(77), prime);
        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(-1), prime);
        let y2 = FieldElement::new(Integer::from(-1), prime);
        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        assert_ne!(p1, p2);
    }

    #[test]
    fn points_with_x_inf_are_equal() {
        let prime = 256;

        let y1 = FieldElement::new(Integer::from(77), prime);
        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(None, Some(y1), a1, b1);

        let y2 = FieldElement::new(Integer::from(77), prime);
        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(None, Some(y2), a2, b2);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_with_y_inf_are_equal() {
        let prime = 256;

        let x1 = FieldElement::new(Integer::from(18), prime);
        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), None, a1, b1);

        let x2 = FieldElement::new(Integer::from(18), prime);
        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), None, a2, b2);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_with_both_x_and_y_inf_are_equal() {
        let prime = 256;

        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(None, None, a1, b1);

        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(None, None, a2, b2);

        assert_eq!(p1, p2);
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_a() {
        let prime = 256;

        let x1 = FieldElement::new(Integer::from(-1), prime);
        let y1 = FieldElement::new(Integer::from(0), prime);
        let a1 = FieldElement::new(Integer::from(6), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(18), prime);
        let y2 = FieldElement::new(Integer::from(77), prime);
        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        let _r_ = p1 + p2;
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_b() {
        let prime = 256;

        let x1 = FieldElement::new(Integer::from(18), prime);
        let y1 = FieldElement::new(Integer::from(77), prime);
        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(0), prime);
        let y2 = FieldElement::new(Integer::from(3), prime);
        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(9), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        let _r_ = p1 + p2;
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_both_a_and_b() {
        let prime = 256;

        let x1 = FieldElement::new(Integer::from(18), prime);
        let y1 = FieldElement::new(Integer::from(77), prime);
        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(0), prime);
        let y2 = FieldElement::new(Integer::from(3), prime);
        let a2 = FieldElement::new(Integer::from(6), prime);
        let b2 = FieldElement::new(Integer::from(9), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        let _r_ = p1 + p2;
    }

    #[test]
    fn adding_infinite_x_point() {
        let prime = 256;

        let y1 = FieldElement::new(Integer::from(77), prime);
        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(None, Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(-1), prime);
        let y2 = FieldElement::new(Integer::from(-1), prime);
        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        let p3 = p1 + p2.clone();

        assert_eq!(p3, p2);
    }

    #[test]
    fn adding_same_x_and_different_y_as_in_vertical_line() {
        let prime = 256;

        let x1 = FieldElement::new(Integer::from(18), prime);
        let y1 = FieldElement::new(Integer::from(77), prime);
        let a1 = FieldElement::new(Integer::from(5), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(18), prime);
        let y2 = FieldElement::new(Integer::from(-77), prime);
        let a2 = FieldElement::new(Integer::from(5), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        let a3 = FieldElement::new(Integer::from(5), prime);
        let b3 = FieldElement::new(Integer::from(7), prime);

        let p3 = Point::new(None, None, a3, b3);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn adding_two_points_1() {
        let prime = 223;

        let x1 = FieldElement::new(Integer::from(170), prime);
        let y1 = FieldElement::new(Integer::from(142), prime);
        let a1 = FieldElement::new(Integer::from(0), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(60), prime);
        let y2 = FieldElement::new(Integer::from(139), prime);
        let a2 = FieldElement::new(Integer::from(0), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        let x3 = FieldElement::new(Integer::from(220), prime);
        let y3 = FieldElement::new(Integer::from(181), prime);
        let a3 = FieldElement::new(Integer::from(0), prime);
        let b3 = FieldElement::new(Integer::from(7), prime);

        let p3 = Point::new(Some(x3), Some(y3), a3, b3);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn adding_two_points_2() {
        let prime = 223;

        let x1 = FieldElement::new(Integer::from(47), prime);
        let y1 = FieldElement::new(Integer::from(71), prime);
        let a1 = FieldElement::new(Integer::from(0), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(17), prime);
        let y2 = FieldElement::new(Integer::from(56), prime);
        let a2 = FieldElement::new(Integer::from(0), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        let x3 = FieldElement::new(Integer::from(215), prime);
        let y3 = FieldElement::new(Integer::from(68), prime);
        let a3 = FieldElement::new(Integer::from(0), prime);
        let b3 = FieldElement::new(Integer::from(7), prime);

        let p3 = Point::new(Some(x3), Some(y3), a3, b3);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn adding_two_points_3() {
        let prime = 223;

        let x1 = FieldElement::new(Integer::from(143), prime);
        let y1 = FieldElement::new(Integer::from(98), prime);
        let a1 = FieldElement::new(Integer::from(0), prime);
        let b1 = FieldElement::new(Integer::from(7), prime);

        let p1 = Point::new(Some(x1), Some(y1), a1, b1);

        let x2 = FieldElement::new(Integer::from(76), prime);
        let y2 = FieldElement::new(Integer::from(66), prime);
        let a2 = FieldElement::new(Integer::from(0), prime);
        let b2 = FieldElement::new(Integer::from(7), prime);

        let p2 = Point::new(Some(x2), Some(y2), a2, b2);

        let x3 = FieldElement::new(Integer::from(47), prime);
        let y3 = FieldElement::new(Integer::from(71), prime);
        let a3 = FieldElement::new(Integer::from(0), prime);
        let b3 = FieldElement::new(Integer::from(7), prime);

        let p3 = Point::new(Some(x3), Some(y3), a3, b3);

        assert_eq!(p1 + p2, p3);
    }
}
