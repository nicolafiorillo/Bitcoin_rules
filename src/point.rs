///! Point management in elliptic curve
use std::ops::Add;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: Option<i64>,
    y: Option<i64>,
    a: i64,
    b: i64,
}

impl Point {
    pub fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Point {
        let point = Point {
            x,
            y,
            a,
            b,
            ..Default::default()
        };

        if let (Some(x_value), Some(y_value)) = (x, y) {
            if y_value.pow(2) != x_value.pow(3) + a * x_value + b {
                panic!("point is not in the curve");
            }
        }

        return point;
    }
}

impl Add for Point {
    type Output = Self;

    // Add operator
    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("points are not in the same curve");
        }

        if self.x == other.x && self.y != other.y {
            return Point::new(None, None, self.a, self.b);
        }

        if self.x.is_none() {
            return other.clone();
        }

        if other.x.is_none() {
            return self.clone();
        }

        if self.x != other.x {
            let y2 = other.y.unwrap();
            let y1 = self.y.unwrap();
            let x1 = self.x.unwrap();
            let x2 = other.x.unwrap();

            let s = (y2 - y1) / (x2 - x1);
            let x = s.pow(2) - x1 - x2;
            let y = s * (x1 - x) - y1;

            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        return Point::new(Some(0), Some(0), 0, 0);
    }
}

#[cfg(test)]
mod point_test {
    use crate::point::*;

    #[test]
    fn a_point_in_curve_1() {
        Point::new(Some(-1), Some(-1), 5, 7);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_1() {
        Point::new(Some(-1), Some(-2), 5, 7);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_2() {
        Point::new(Some(2), Some(4), 5, 7);
    }

    #[test]
    fn a_point_in_curve_2() {
        Point::new(Some(18), Some(77), 5, 7);
    }

    #[test]
    #[should_panic(expected = "point is not in the curve")]
    fn point_not_in_curve_3() {
        Point::new(Some(5), Some(7), 5, 7);
    }

    #[test]
    fn points_are_equal() {
        let p1 = Point::new(Some(18), Some(77), 5, 7);
        let p2 = Point::new(Some(18), Some(77), 5, 7);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_are_not_equal() {
        let p1 = Point::new(Some(18), Some(77), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);

        assert_ne!(p1, p2);
    }

    #[test]
    fn points_with_x_inf_are_equal() {
        let p1 = Point::new(None, Some(77), 5, 7);
        let p2 = Point::new(None, Some(77), 5, 7);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_with_y_inf_are_equal() {
        let p1 = Point::new(Some(18), None, 5, 7);
        let p2 = Point::new(Some(18), None, 5, 7);

        assert_eq!(p1, p2);
    }

    #[test]
    fn points_with_both_x_and_y_inf_are_equal() {
        let p1 = Point::new(None, None, 5, 7);
        let p2 = Point::new(None, None, 5, 7);

        assert_eq!(p1, p2);
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_a() {
        let p1 = Point::new(Some(-1), Some(0), 6, 7);
        let p2 = Point::new(Some(18), Some(77), 5, 7);

        let _r_ = p1 + p2;
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_b() {
        let p1 = Point::new(Some(18), Some(77), 5, 7);
        let p2 = Point::new(Some(0), Some(3), 5, 9);

        let _r_ = p1 + p2;
    }

    #[test]
    #[should_panic(expected = "points are not in the same curve")]
    fn adding_points_in_different_curve_both_a_and_b() {
        let p1 = Point::new(Some(18), Some(77), 5, 7);
        let p2 = Point::new(Some(0), Some(3), 6, 9);

        let _r_ = p1 + p2;
    }

    #[test]
    fn adding_infinite_x_point() {
        let p1 = Point::new(None, Some(77), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);

        let p3 = p1 + p2;

        assert_eq!(p3, p2);
    }

    #[test]
    fn adding_same_x_and_different_y_as_in_vertical_line() {
        let p1 = Point::new(Some(18), Some(77), 5, 7);
        let p2 = Point::new(Some(18), Some(-77), 5, 7);

        let p3 = Point::new(None, None, 5, 7);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn adding_with_different_x() {
        let p1 = Point::new(Some(2), Some(5), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);

        let p3 = Point::new(Some(3), Some(-7), 5, 7);

        assert_eq!(p1 + p2, p3);
    }
}
