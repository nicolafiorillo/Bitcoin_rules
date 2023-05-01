///! Integer expansion.
use rug::{integer::Order, Integer};

pub trait IntegerEx {
    fn new_from_256_digits(ll: u64, lr: u64, rl: u64, rr: u64) -> Self;
    fn power_modulo(&self, exp: &Integer, modulo: &Integer) -> Self;
    fn invert_by_modulo(&self, modulo: &Integer) -> Self;
}

impl IntegerEx for Integer {
    /// New Integer from 256 digits.
    fn new_from_256_digits(ll: u64, lr: u64, rl: u64, rr: u64) -> Integer {
        let digits: [u64; 4] = [ll, lr, rl, rr];
        Integer::from_digits(&digits, Order::Msf)
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
}
