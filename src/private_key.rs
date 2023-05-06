///! Private key management
use std::fmt::{Display, Formatter, Result};

use hmac::{Hmac, Mac, NewMac};
use rug::{integer::Order, Integer};
use sha2::Sha256;

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
        let k = PrivateKey::deterministic_k(&self.secret, &z);

        let r = (&(*G).clone() * k.clone()).x_as_num();

        let k_inv = k.invert_by_modulo(&N);

        let sl = (z + &r * &self.secret) * k_inv;
        let (_q, mut s) = sl.div_rem_euc((*N).clone());

        if s > ((*N).clone() / 2) {
            s = (*N).clone() - s;
        };

        Signature { r, s }
    }

    fn vect_to_array_32(v: &Vec<u8>) -> [u8; 32] {
        let mut arr: [u8; 32] = [0u8; 32];
        for i in 0..v.len() {
            arr[31 - i] = v[i];
        }

        arr
    }

    fn hmac_for_data(data: &[u8], mut k: [u8; 32]) -> [u8; 32] {
        let mut hmac_sha256 = Hmac::<Sha256>::new_varkey(&k).expect("HMAC initialization failed");
        hmac_sha256.update(&data);
        k.copy_from_slice(hmac_sha256.finalize().into_bytes().as_slice());

        k
    }

    /// https://www.rfc-editor.org/rfc/rfc6979.txt
    pub fn deterministic_k(secret: &Integer, hashed: &Integer) -> Integer {
        let mut z = hashed.clone();

        if z > *N {
            z -= (*N).clone();
        }

        let zero = [0u8];
        let one = [1u8];

        let z_vect = z.to_digits::<u8>(Order::LsfBe);
        let z_bytes = PrivateKey::vect_to_array_32(&z_vect);

        let secret_vect = secret.to_digits::<u8>(Order::LsfBe);
        let secret_bytes = PrivateKey::vect_to_array_32(&secret_vect);

        let mut k: [u8; 32] = [0u8; 32];
        let mut v: [u8; 32] = [1u8; 32];

        let mut data = [v.as_slice(), zero.as_slice(), &secret_bytes, &z_bytes].concat();
        k = PrivateKey::hmac_for_data(&data, k);
        v = PrivateKey::hmac_for_data(&v, k);

        data = [v.as_slice(), one.as_slice(), &secret_bytes, &z_bytes].concat();
        k = PrivateKey::hmac_for_data(&data, k);
        v = PrivateKey::hmac_for_data(&v, k);

        loop {
            v = PrivateKey::hmac_for_data(&v, k);
            let candidate: Integer = Integer::from_digits(&v, Order::MsfBe);

            if candidate >= 1 && candidate < *N {
                return candidate;
            }

            data = [v.as_slice(), zero.as_slice()].concat();
            k = PrivateKey::hmac_for_data(&data, k);
            v = PrivateKey::hmac_for_data(&v, k);
        }
    }
}

impl Display for PrivateKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Private key({}", self.point)
    }
}

#[cfg(test)]
mod private_key_test {
    use rug::{Complete, Integer};

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

    #[test]
    fn deterministic_k_1() {
        let k = PrivateKey::deterministic_k(&Integer::from(10), &Integer::from(1));
        assert_eq!(
            k,
            Integer::parse("23556289421633918234640030791776902309669950917001758018452865836473455104574")
                .unwrap()
                .complete()
        );
    }

    #[test]
    fn deterministic_k_2() {
        let k = PrivateKey::deterministic_k(&Integer::from(2345), &Integer::from(6789));
        assert_eq!(
            k,
            Integer::parse("34113680596947005563568962966999203522429670732921816689907697765389746251584")
                .unwrap()
                .complete()
        );
    }

    #[test]
    fn deterministic_k_3() {
        let k = PrivateKey::deterministic_k(&Integer::from(1000000), &Integer::from(1000000));
        assert_eq!(
            k,
            Integer::parse("35877450084421794080905523995859466786371393244910114637747627798158238933625")
                .unwrap()
                .complete()
        );
    }
}
