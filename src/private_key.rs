///! Private key management
use std::fmt::{Display, Formatter, Result};

use rug::{rand::RandState, Integer};

use crate::{
    btc_ecdsa::{G, N},
    integer_ex::IntegerEx,
    point::Point,
    signature::Signature,
};

/// Private key structure.
pub struct PrivateKey {
    /// secret number
    secret: Integer,
    /// public key
    point: Point,
}

impl PrivateKey {
    /// New `PrivateKey` by secret.
    pub fn new(secret: Integer) -> PrivateKey {
        let point = &(*G).clone() * secret.clone();
        PrivateKey { secret, point }
    }

    /// Sign a message.
    /// `z` is the hash of the message.
    /// Return the `Signature` for the signed message.
    pub fn sign(&self, z: Integer) -> Signature {
        let mut rand = RandState::new();
        let k = (*N).clone().random_below(&mut rand);

        let r = (&(*G).clone() * k.clone()).x_as_num();

        let k_inv = k.invert_by_modulo(&N);

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
