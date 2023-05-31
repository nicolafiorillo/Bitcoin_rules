///! Private key management
use std::fmt::{Display, Formatter, Result};

use hmac::{Hmac, Mac};
use rug::{integer::Order, Integer};
use sha2::Sha256;

use crate::{
    btc_ecdsa::{G, N},
    helper::vector,
    integer_ex::IntegerEx,
    point::Point,
    signature::Signature,
};

/// Private key structure.
pub struct PrivateKey {
    /// secret number
    secret: Integer,
    /// public key
    pub point: Point,
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
        let k = Self::deterministic_k(&self.secret, &z);

        let r = (&(*G).clone() * k.clone()).x_as_num();

        let k_inv = k.invert_by_modulo(&N);

        let sl = (z + &r * &self.secret) * k_inv;
        let (_q, mut s) = sl.div_rem_euc((*N).clone());

        if s > ((*N).clone() / 2) {
            s = (*N).clone() - s;
        };

        Signature { r, s }
    }

    fn hmac_for_data(data: &[u8], mut k: [u8; 32]) -> [u8; 32] {
        let mut hmac_sha256 = Hmac::<Sha256>::new_from_slice(&k).expect("HMAC initialization failed");
        hmac_sha256.update(data);
        k.copy_from_slice(hmac_sha256.finalize().into_bytes().as_slice());

        k
    }

    /// https://www.rfc-editor.org/rfc/rfc6979.txt
    pub fn deterministic_k(secret: &Integer, hashed: &Integer) -> Integer {
        let mut z = hashed.clone();

        if z > *N {
            z -= (*N).clone();
        }

        let zero: [u8; 1] = [0u8];
        let one: [u8; 1] = [1u8];

        let z_vect: Vec<u8> = z.to_digits::<u8>(Order::Msf);
        let z_bytes: [u8; 32] = vector::vect_to_array_32(&z_vect);

        let secret_vect: Vec<u8> = secret.to_digits::<u8>(Order::Msf);
        let secret_bytes: [u8; 32] = vector::vect_to_array_32(&secret_vect);

        let mut k: [u8; 32] = [0u8; 32];
        let mut v: [u8; 32] = [1u8; 32];

        let mut data: Vec<u8> = [v.as_slice(), zero.as_slice(), &secret_bytes, &z_bytes].concat();
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
    use rug::{integer::Order, ops::Pow, Integer};

    use super::PrivateKey;
    use crate::{
        hashing::hash256,
        integer_ex::IntegerEx,
        point::{Compression, Point},
    };

    #[test]
    fn verify_a_signature() {
        let secret = "A SECRET".to_string();
        let message = "A MESSAGE".to_string();
        let e = hash256(&secret.as_bytes().to_vec());
        let z = hash256(&message.as_bytes().to_vec());

        let e_integer = Integer::from_digits(&e, Order::Msf);
        let z_integer = Integer::from_digits(&z, Order::Msf);

        let private_key = PrivateKey::new(e_integer);
        let sign = private_key.sign(z_integer.clone());

        assert!(private_key.point.verify(z_integer, sign));
    }

    pub fn to_hex_string(bytes: &[u8]) -> String {
        let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
        strs.join("")
    }

    #[test]
    fn serialize_a_public_key_1() {
        let private_key = PrivateKey::new(Integer::from(5000));
        let sec = private_key.point.serialize(Compression::Uncompressed);
        assert_eq!(to_hex_string(&sec), "04FFE558E388852F0120E46AF2D1B370F85854A8EB0841811ECE0E3E03D282D57C315DC72890A4F10A1481C031B03B351B0DC79901CA18A00CF009DBDB157A1D10");
    }

    #[test]
    fn serialize_a_public_key_2() {
        let private_key = PrivateKey::new(Integer::from(2018).pow(5));
        let sec = private_key.point.serialize(Compression::Uncompressed);
        assert_eq!(to_hex_string(&sec), "04027F3DA1918455E03C46F659266A1BB5204E959DB7364D2F473BDF8F0A13CC9DFF87647FD023C13B4A4994F17691895806E1B40B57F4FD22581A4F46851F3B06");
    }

    #[test]
    fn serialize_a_public_key_3() {
        let private_key = PrivateKey::new(Integer::new_from_hex_str("DEADBEEF12345"));
        let sec = private_key.point.serialize(Compression::Uncompressed);
        assert_eq!(to_hex_string(&sec), "04D90CD625EE87DD38656DD95CF79F65F60F7273B67D3096E68BD81E4F5342691F842EFA762FD59961D0E99803C61EDBA8B3E3F7DC3A341836F97733AEBF987121");
    }

    #[test]
    fn serialize_a_compressed_public_key_1() {
        let private_key = PrivateKey::new(Integer::from(5001));
        let sec = private_key.point.serialize(Compression::Compressed);
        assert_eq!(
            to_hex_string(&sec),
            "0357A4F368868A8A6D572991E484E664810FF14C05C0FA023275251151FE0E53D1"
        );
    }

    #[test]
    fn serialize_a_compressed_public_key_2() {
        let private_key = PrivateKey::new(Integer::from(2019).pow(5));
        let sec = private_key.point.serialize(Compression::Compressed);
        assert_eq!(
            to_hex_string(&sec),
            "02933EC2D2B111B92737EC12F1C5D20F3233A0AD21CD8B36D0BCA7A0CFA5CB8701"
        );
    }

    #[test]
    fn serialize_a_compressed_public_key_3() {
        let private_key = PrivateKey::new(Integer::new_from_hex_str("DEADBEEF54321"));
        let sec = private_key.point.serialize(Compression::Compressed);
        assert_eq!(
            to_hex_string(&sec),
            "0296BE5B1292F6C856B3C5654E886FC13511462059089CDF9C479623BFCBE77690"
        );
    }

    #[test]
    fn deserialize_a_public_key_1() {
        let point = Point::deserialize("04FFE558E388852F0120E46AF2D1B370F85854A8EB0841811ECE0E3E03D282D57C315DC72890A4F10A1481C031B03B351B0DC79901CA18A00CF009DBDB157A1D10");

        assert_eq!(
            point.x_as_num(),
            Integer::new_from_hex_str("FFE558E388852F0120E46AF2D1B370F85854A8EB0841811ECE0E3E03D282D57C")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::new_from_hex_str("315DC72890A4F10A1481C031B03B351B0DC79901CA18A00CF009DBDB157A1D10")
        );
    }

    #[test]
    fn deserialize_a_public_key_2() {
        let point = Point::deserialize("04027F3DA1918455E03C46F659266A1BB5204E959DB7364D2F473BDF8F0A13CC9DFF87647FD023C13B4A4994F17691895806E1B40B57F4FD22581A4F46851F3B06");

        assert_eq!(
            point.x_as_num(),
            Integer::new_from_hex_str("027F3DA1918455E03C46F659266A1BB5204E959DB7364D2F473BDF8F0A13CC9D")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::new_from_hex_str("FF87647FD023C13B4A4994F17691895806E1B40B57F4FD22581A4F46851F3B06")
        );
    }

    #[test]
    fn deserialize_a_public_key_3() {
        let point = Point::deserialize("04D90CD625EE87DD38656DD95CF79F65F60F7273B67D3096E68BD81E4F5342691F842EFA762FD59961D0E99803C61EDBA8B3E3F7DC3A341836F97733AEBF987121");

        assert_eq!(
            point.x_as_num(),
            Integer::new_from_hex_str("D90CD625EE87DD38656DD95CF79F65F60F7273B67D3096E68BD81E4F5342691F")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::new_from_hex_str("842EFA762FD59961D0E99803C61EDBA8B3E3F7DC3A341836F97733AEBF987121")
        );
    }

    #[test]
    fn deserialize_a_compressed_public_key_1() {
        let point = Point::deserialize("0357A4F368868A8A6D572991E484E664810FF14C05C0FA023275251151FE0E53D1");

        assert_eq!(
            point.x_as_num(),
            Integer::new_from_hex_str("57A4F368868A8A6D572991E484E664810FF14C05C0FA023275251151FE0E53D1")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::new_from_hex_str("D6CC87C5BC29B83368E17869E964F2F53D52EA3AA3E5A9EFA1FA578123A0C6D")
        );
    }

    #[test]
    fn deserialize_a_compressed_public_key_2() {
        let point = Point::deserialize("02933EC2D2B111B92737EC12F1C5D20F3233A0AD21CD8B36D0BCA7A0CFA5CB8701");

        assert_eq!(
            point.x_as_num(),
            Integer::new_from_hex_str("933EC2D2B111B92737EC12F1C5D20F3233A0AD21CD8B36D0BCA7A0CFA5CB8701")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::new_from_hex_str("96CBBFDD572F75ACE44D0AA59FBAB6326CB9F909385DCD066EA27AFFEF5A488C")
        );
    }

    #[test]
    fn deserialize_a_compressed_public_key_3() {
        let point = Point::deserialize("0296BE5B1292F6C856B3C5654E886FC13511462059089CDF9C479623BFCBE77690");

        assert_eq!(
            point.x_as_num(),
            Integer::new_from_hex_str("96BE5B1292F6C856B3C5654E886FC13511462059089CDF9C479623BFCBE77690")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::new_from_hex_str("32555D1B027C25C2828BA96A176D78419CD1236F71558F6187AEC09611325EB6")
        );
    }

    #[test]
    fn deterministic_k_1() {
        let k = PrivateKey::deterministic_k(&Integer::from(10), &Integer::from(1));
        assert_eq!(
            k,
            Integer::new_from_dec_str("23556289421633918234640030791776902309669950917001758018452865836473455104574")
        );
    }

    #[test]
    fn deterministic_k_2() {
        let k = PrivateKey::deterministic_k(&Integer::from(2345), &Integer::from(6789));
        assert_eq!(
            k,
            Integer::new_from_dec_str("34113680596947005563568962966999203522429670732921816689907697765389746251584")
        );
    }

    #[test]
    fn deterministic_k_3() {
        let k = PrivateKey::deterministic_k(&Integer::from(1000000), &Integer::from(1000000));
        assert_eq!(
            k,
            Integer::new_from_dec_str("35877450084421794080905523995859466786371393244910114637747627798158238933625")
        );
    }
}
