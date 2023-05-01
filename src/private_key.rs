use std::fmt::{Display, Formatter, Result};

use rug::{rand::RandState, Integer};

use crate::{
    btc_ecdsa::{G, N},
    point::Point,
    signature::Signature,
};

pub struct PrivateKey {
    secret: Integer,
    point: Point,
}

impl PrivateKey {
    pub fn new(secret: Integer) -> PrivateKey {
        let point = &(*G).clone() * secret.clone();
        PrivateKey { secret, point }
    }

    pub fn sign(&self, z: Integer) -> Signature {
        let mut rand = RandState::new();
        let k = (*N).clone().random_below(&mut rand);

        let r = (&(*G).clone() * k.clone()).x_as_num();

        let k_inv = match k.pow_mod(&((*N).clone() - 2), &(*N).clone()) {
            Ok(power) => power,
            Err(_) => unreachable!(),
        };

        let sl = (z + &r * &self.secret) * k_inv;
        let (_q, mut s) = sl.div_rem_euc((*N).clone());

        if s > ((*N).clone() / 2) {
            s = (*N).clone() - s;
        };

        Signature { r, s }
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Private key({}", self.point)
    }
}

#[cfg(test)]
mod private_key_test {
    use super::PrivateKey;
    use crate::hash256::hash256;

    #[test]
    fn verify_a_signature() {
        let secret = "A SECRET".to_string();
        let message = "A MESSAGE".to_string();
        let e = hash256(secret);
        let z = hash256(message);

        let private_key = PrivateKey::new(e);
        let sign = private_key.sign(z.clone());

        assert!(private_key.point.verify(z, sign));
    }
}
