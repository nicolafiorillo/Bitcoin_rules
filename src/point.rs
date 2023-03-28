use rug::{ops::Pow, Integer};

#[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: Integer,
    y: Integer,
    a: Integer,
    b: Integer,
}

impl Point {
    pub fn new(x: Integer, y: Integer, a: Integer, b: Integer) -> Point {
        if y.clone().pow(2) != x.clone().pow(3) + a.clone() * x.clone() + b.clone() {
            panic!("point is not in the curve");
        }

        Point {
            x,
            y,
            a,
            b,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod point_test {
    use crate::point::*;

    #[test]
    fn a_point_in_curve_1() {
        Point::new(
            Integer::from(-1),
            Integer::from(-1),
            Integer::from(5),
            Integer::from(7),
        );
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_1() {
        Point::new(
            Integer::from(-1),
            Integer::from(-2),
            Integer::from(5),
            Integer::from(7),
        );
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_2() {
        Point::new(
            Integer::from(2),
            Integer::from(4),
            Integer::from(5),
            Integer::from(7),
        );
    }

    #[test]
    fn a_point_in_curve_2() {
        Point::new(
            Integer::from(18),
            Integer::from(77),
            Integer::from(5),
            Integer::from(7),
        );
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_3() {
        Point::new(
            Integer::from(5),
            Integer::from(7),
            Integer::from(5),
            Integer::from(7),
        );
    }

    #[test]
    fn points_are_equal() {
        let p1 = Point::new(
            Integer::from(18),
            Integer::from(77),
            Integer::from(5),
            Integer::from(7),
        );

        let p2 = Point::new(
            Integer::from(18),
            Integer::from(77),
            Integer::from(5),
            Integer::from(7),
        );

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_are_not_equal() {
        let p1 = Point::new(
            Integer::from(18),
            Integer::from(77),
            Integer::from(5),
            Integer::from(7),
        );

        let p2 = Point::new(
            Integer::from(-1),
            Integer::from(-1),
            Integer::from(5),
            Integer::from(7),
        );

        assert_ne!(p1, p2);
    }
}
