//! Integer expansion.

use rug::{integer::Order, Complete, Integer};

use crate::low::vector::padding_right;

pub trait IntegerEx {
    fn from_256_digits(ll: u64, lr: u64, rl: u64, rr: u64) -> Self;
    fn from_hex_str(s: &str) -> Self;
    fn from_dec_str(s: &str) -> Self;
    fn power_modulo(&self, exp: &Integer, modulo: &Integer) -> Self;
    fn invert_by_modulo(&self, modulo: &Integer) -> Self;
    fn from_little_endian_bytes(bytes: &[u8]) -> Self;
    fn to_little_endian_bytes(&self, length: usize) -> Vec<u8>;
}

impl IntegerEx for Integer {
    /// New Integer from 256 digits.
    fn from_256_digits(ll: u64, lr: u64, rl: u64, rr: u64) -> Integer {
        let digits: [u64; 4] = [ll, lr, rl, rr];
        Integer::from_digits(&digits, Order::Msf)
    }

    fn from_hex_str(s: &str) -> Integer {
        Integer::parse_radix(s, 16).unwrap().complete()
    }

    fn from_dec_str(s: &str) -> Integer {
        Integer::parse(s).unwrap().complete()
    }

    /// Exponential applying modulo.
    fn power_modulo(&self, exp: &Integer, modulo: &Integer) -> Self {
        match self.clone().pow_mod(exp, modulo) {
            Ok(left) => left,
            Err(_) => unreachable!(),
        }
    }

    /// Invert Integer by modulo (1/self).
    fn invert_by_modulo(&self, modulo: &Integer) -> Self {
        self.power_modulo(&(modulo.clone() - 2), modulo)
    }

    // big endian: most significant value first.
    // little endian: least significant value first.
    fn from_little_endian_bytes(bytes: &[u8]) -> Integer {
        Integer::from_digits(bytes, Order::Lsf)
    }

    fn to_little_endian_bytes(&self, length: usize) -> Vec<u8> {
        let bytes = self.to_digits::<u8>(Order::Lsf);
        padding_right(&bytes, length, 0)
    }
}

#[cfg(test)]
mod integer_ex_test {
    use rug::Integer;

    use crate::low::integer_ex::IntegerEx;

    #[test]
    fn from_little_endian() {
        let bytes: Integer = IntegerEx::from_little_endian_bytes(&[0x39, 0x30]);
        assert_eq!(bytes, 12345);
    }

    #[test]
    fn to_little_endian() {
        let n = Integer::from(12345);
        let bytes = n.to_little_endian_bytes(2);
        assert_eq!(bytes, [0x39, 0x30]);
    }
}
