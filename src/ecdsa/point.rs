/// 'Point' is a point on an elliptic curve.
use crate::{
    bitcoin::{
        compression::Compression,
        ecdsa_btc::{N, P, SEVEN, ZERO},
    },
    ecdsa::field_element::FieldElement,
    hashing::hash160::hash160,
    std_lib::vector::vect_to_array_32,
};
use rug::{integer::Order, Integer};

use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

/// `Point` is a point on an elliptic curve.
/// Both `x` and `y` as `None` indicates point at infinite.
#[derive(Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

#[repr(u8)]
enum PointCoordinateParity {
    Uncompressed = 0x04,
    Odd = 0x03,
    Even = 0x02,
}

impl Point {
    /// New `Point` from all elements.
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Point {
        if let (Some(x_value), Some(y_value)) = (x.clone(), y.clone()) {
            if y_value.pow_by_i32(2) != x_value.clone().pow_by_i32(3) + a.clone() * x_value + b.clone() {
                panic!("point is not in the curve");
            }
        }

        Point { x, y, a, b }
    }

    /// New `Point` by `x` and `y` in btc field.
    pub fn new_in_secp256k1(x: Option<FieldElement>, y: Option<FieldElement>) -> Point {
        let a = FieldElement::new_in_secp256k1((*ZERO).clone());
        let b = FieldElement::new_in_secp256k1((*SEVEN).clone());

        Point { x, y, a, b }
    }

    /// New `Point` from raw numbers.
    pub fn new_with_numbers(x: i32, y: i32, a: i32, b: i32, prime: u32) -> Point {
        let xfe = FieldElement::new(Integer::from(x), Integer::from(prime));
        let yfe = FieldElement::new(Integer::from(y), Integer::from(prime));
        let afe = FieldElement::new(Integer::from(a), Integer::from(prime));
        let bfe = FieldElement::new(Integer::from(b), Integer::from(prime));

        Point::new(Some(xfe), Some(yfe), afe, bfe)
    }

    /// New `Point` expressing "infinite".
    pub fn new_infinite(a: &FieldElement, b: &FieldElement) -> Point {
        Point::new(None, None, a.clone(), b.clone())
    }

    /// `true` if `Point` is infinite..
    pub fn is_infinite(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    /// Get `x` value.
    pub fn x_as_num(&self) -> Integer {
        self.x.clone().unwrap().num()
    }

    /// Get `y` value.
    pub fn y_as_num(&self) -> Integer {
        self.y.clone().unwrap().num()
    }

    /// https://www.secg.org/
    pub fn serialize(&self, compression: Compression) -> Vec<u8> {
        match compression {
            Compression::Compressed => self.serialize_compressed().to_vec(),
            Compression::Uncompressed => self.serialize_uncompressed().to_vec(),
        }
    }

    fn serialize_uncompressed(&self) -> [u8; 65] {
        let x_vec: Vec<u8> = self.x_as_num().to_digits::<u8>(Order::Msf);
        let y_vec: Vec<u8> = self.y_as_num().to_digits::<u8>(Order::Msf);

        let x: [u8; 32] = vect_to_array_32(&x_vec);
        let y: [u8; 32] = vect_to_array_32(&y_vec);

        let mut res: [u8; 65] = [0; 65];
        res[0] = 4;
        res[1..33].copy_from_slice(&x);
        res[33..65].copy_from_slice(&y);

        res
    }

    // Compressed SEC format.
    // Here we only need to serialize `x` value and the information if `y` is odd or even.
    // `y` can be calculated from `x` but it can be `y` or `p-y`. `p` is odd, so we can know if `y` is odd or even.
    // (in elliptic curves algebra, given x we can have 0, 1, or 2 y).
    fn serialize_compressed(&self) -> [u8; 33] {
        let prefix: u8 = if self.y_as_num().is_odd() {
            PointCoordinateParity::Odd
        } else {
            PointCoordinateParity::Even
        } as u8;

        let x_vec: Vec<u8> = self.x_as_num().to_digits::<u8>(Order::Msf);
        let x: [u8; 32] = vect_to_array_32(&x_vec);

        let mut res: [u8; 33] = [0; 33];
        res[0] = prefix;
        res[1..33].copy_from_slice(&x);

        res
    }

    fn deserialize_uncompressed(bytes: &[u8]) -> Point {
        let x_digits = &bytes[1..33];
        let x = Integer::from_digits(x_digits, Order::Msf);

        let y_digits = &bytes[33..65];
        let y = Integer::from_digits(y_digits, Order::Msf);

        let x_field = FieldElement::new_in_secp256k1(x);
        let y_field = FieldElement::new_in_secp256k1(y);

        Point::new_in_secp256k1(Some(x_field), Some(y_field))
    }

    fn deserialize_compressed(bytes: &[u8]) -> Point {
        let x_digits = &bytes[1..33];
        let x = FieldElement::new_in_secp256k1(Integer::from_digits(x_digits, Order::Msf));

        // Apply y^2 = x^3 + 7 (in Fp) to retrieve y
        let right_side = x.pow_by_i32(3) + FieldElement::new_in_secp256k1((*SEVEN).clone());

        let left_side = right_side.sqrt();

        let y_is_even = bytes[0] == PointCoordinateParity::Even as u8;
        let left_side_is_even = left_side.num().is_even();

        let y = if y_is_even == left_side_is_even {
            left_side
        } else {
            // left_side_inverted
            FieldElement::new_in_secp256k1((*P).clone() - left_side.num())
        };

        Point::new_in_secp256k1(Some(x), Some(y))
    }

    pub fn deserialize(serialized: Vec<u8>) -> Point {
        let bytes_length = serialized.len();
        if bytes_length != 65 && bytes_length != 33 {
            panic!("invalid binary length");
        }

        if serialized[0] == PointCoordinateParity::Uncompressed as u8 {
            return Self::deserialize_uncompressed(&serialized);
        }

        if serialized[0] == PointCoordinateParity::Even as u8 || serialized[0] == PointCoordinateParity::Odd as u8 {
            return Self::deserialize_compressed(&serialized);
        }

        panic!("unknown binary type in deserialization");
    }

    pub fn hash160(&self, compression: Compression) -> Vec<u8> {
        let serialized = self.serialize(compression);
        hash160(&serialized)
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

    // Add operator: `Point` + `&Point`.
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
            let x = s.pow_by_i32(2) - x1 - x2;
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

            let s = (3 * x1.clone().pow_by_i32(2) + self.a.clone()) / (2 * y1);
            let x = s.pow_by_i32(2) - (2 * x1.clone());
            let y = s * (x1 - &x) - y1;

            return Point::new(Some(x), Some(y), self.a, self.b);
        }

        unreachable!();
    }
}

impl Mul<u32> for &Point {
    type Output = Point;

    // Mul operator: `Point` * `u32`.
    fn mul(self, coefficient: u32) -> Point {
        if coefficient == 0 {
            panic!("TODO: multiplication by zero not implemented");
        }

        let mut sel: Point = self.clone();

        let mut coef = coefficient;
        let mut result = Point::new_infinite(&self.a, &self.b);

        while coef > 0 {
            if (coef & 1) == 1 {
                result = result + &sel;
            }
            sel = sel.clone() + &sel;

            coef >>= 1;
        }

        result
    }
}

impl Mul<Integer> for &Point {
    type Output = Point;

    // Mul operator: `&Point` * `Integer`.
    fn mul(self, coefficient: Integer) -> Point {
        if coefficient == 0 {
            panic!("TODO: multiplication by zero not implemented");
        }

        let mut sel: Point = self.clone();
        let (_q, mut coef) = coefficient.div_rem_euc((*N).clone());
        let mut result = Point::new_infinite(&self.a, &self.b);

        while coef > 0 {
            if coef.get_bit(0) {
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
    use super::*;
    use crate::{
        bitcoin::ecdsa_btc::G,
        keys::{signature::Signature, verification::verify},
        std_lib::integer_ex::IntegerEx,
    };

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
        let yfe = FieldElement::new(Integer::from(y), Integer::from(prime));
        let afe = FieldElement::new(Integer::from(a), Integer::from(prime));
        let bfe = FieldElement::new(Integer::from(b), Integer::from(prime));

        Point::new(None, Some(yfe), afe, bfe)
    }

    fn a_point_y_none(x: i32, a: i32, b: i32, prime: u32) -> Point {
        let xfe = FieldElement::new(Integer::from(x), Integer::from(prime));
        let afe = FieldElement::new(Integer::from(a), Integer::from(prime));
        let bfe = FieldElement::new(Integer::from(b), Integer::from(prime));

        Point::new(Some(xfe), None, afe, bfe)
    }

    fn a_infinite_point(a: i32, b: i32, prime: u32) -> Point {
        let afe = FieldElement::new(Integer::from(a), Integer::from(prime));
        let bfe = FieldElement::new(Integer::from(b), Integer::from(prime));

        Point::new_infinite(&afe, &bfe)
    }

    #[test]
    fn a_secp256k1_test() {
        let p = &(*G).clone() * (*N).clone();
        assert!(p.is_infinite());
    }

    #[test]
    fn a_signature_1_verification() {
        let z = Integer::from_256_digits(
            0xec208baa0fc1c19f,
            0x708a9ca96fdeff3a,
            0xc3f230bb4a7ba4ae,
            0xde4942ad003c0f60,
        );
        let r = Integer::from_256_digits(
            0xac8d1c87e51d0d44,
            0x1be8b3dd5b05c879,
            0x5b48875dffe00b7f,
            0xfcfac23010d3a395,
        );
        let s = Integer::from_256_digits(
            0x068342ceff8935ed,
            0xedd102dd876ffd6b,
            0xa72d6a427a3edb13,
            0xd26eb0781cb423c4,
        );
        let px = Integer::from_256_digits(
            0x887387e452b8eacc,
            0x4acfde10d9aaf7f6,
            0xd9a0f975aabb10d0,
            0x06e4da568744d06c,
        );
        let py = Integer::from_256_digits(
            0x61de6d95231cd890,
            0x26e286df3b6ae4a8,
            0x94a3378e393e93a0,
            0xf45b666329a0ae34,
        );

        let ppx = FieldElement::new(px, (*P).clone());
        let ppy = FieldElement::new(py, (*P).clone());
        let point = Point::new_in_secp256k1(Some(ppx), Some(ppy));
        let sig = Signature::new(r, s);

        assert!(verify(&point, &z, &sig));
    }

    #[test]
    fn a_signature_2_verification() {
        let z = Integer::from_256_digits(
            0x7c076ff316692a3d,
            0x7eb3c3bb0f8b1488,
            0xcf72e1afcd929e29,
            0x307032997a838a3d,
        );
        let r = Integer::from_256_digits(
            0x00eff69ef2b1bd93,
            0xa66ed5219add4fb5,
            0x1e11a840f4048763,
            0x25a1e8ffe0529a2c,
        );
        let s = Integer::from_256_digits(
            0xc7207fee197d27c6,
            0x18aea621406f6bf5,
            0xef6fca38681d82b2,
            0xf06fddbdce6feab6,
        );
        let px = Integer::from_256_digits(
            0x887387e452b8eacc,
            0x4acfde10d9aaf7f6,
            0xd9a0f975aabb10d0,
            0x06e4da568744d06c,
        );
        let py = Integer::from_256_digits(
            0x61de6d95231cd890,
            0x26e286df3b6ae4a8,
            0x94a3378e393e93a0,
            0xf45b666329a0ae34,
        );

        let ppx = FieldElement::new(px, (*P).clone());
        let ppy = FieldElement::new(py, (*P).clone());
        let point = Point::new_in_secp256k1(Some(ppx), Some(ppy));
        let sig = Signature::new(r, s);

        assert!(verify(&point, &z, &sig));
    }
}
