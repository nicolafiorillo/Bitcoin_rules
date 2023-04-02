///! Point management in elliptic curve

#[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: Option<i64>,
    y: Option<i64>,
    a: i64,
    b: i64,
}

impl Point {
    const INFINITE: Point = Point {
        x: None,
        y: None,
        a: 0,
        b: 0,
    };

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
}
