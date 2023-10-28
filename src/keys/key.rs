//! Private key management and verification

use rug::{integer::Order, Integer};
use std::fmt::{Display, Formatter};

use crate::{
    bitcoin::{
        address_prefix::AddressPrefix,
        ecdsa::{G, N},
    },
    ecdsa::point::Point,
    flags::{compression::Compression, network::Network},
    keys::signature::Signature,
    std_lib::base58,
    std_lib::integer_extended::IntegerExtended,
    std_lib::vector::{padding_left, vect_to_array_32},
};

use super::key_error::KeyError;

/// Key structure.
pub struct Key {
    /// real private key (k)
    private_key: Integer,
    /// public key
    public_key: Point,
}

impl Key {
    /// New `PrivateKey` by secret.
    pub fn new(private_key: Integer) -> Key {
        let public_key = &(*G).clone() * private_key.clone();
        Key {
            private_key,
            public_key,
        }
    }

    pub fn verify(&self, z: &Integer, sig: &Signature) -> bool {
        Self::verify_signature(&self.public_key, z, sig)
    }

    pub fn from_wif(_wif: &str) -> Key {
        unimplemented!()
    }

    // ANCHOR: fn_address
    /// Algorithm:
    ///    [network, ((point.x, point.y) |> serialize() |> hash160())] |> base58check()
    pub fn address(&self, compression: Compression, network: Network) -> String {
        let h160 = self.public_key.hash160(compression);
        let p = vec![network as u8];

        let data = [p.as_slice(), h160.as_slice()].concat();

        base58::base58_encode_with_checksum(&data)
    }
    // ANCHOR_END: fn_address

    pub fn address_to_hash160(address: &str, network: Network) -> Result<Vec<u8>, KeyError> {
        let decoded = base58::base58_decode_with_checksum(address).unwrap();

        let net = decoded[0];
        if net != network as u8 {
            return Err(KeyError::IncongruentNetwork);
        }

        Ok(decoded[1..].to_vec())
    }

    /// Sign a message.
    /// `z` is the hash of the message.
    /// Return the `Signature` for the signed message.
    pub fn sign(&self, z: Integer) -> Signature {
        let k = Self::deterministic_k(&self.private_key, &z);

        let r = (&(*G).clone() * k.clone()).x_as_num();

        let k_inv = k.invert_by_modulo(&N);

        let sl = (z + &r * &self.private_key) * k_inv;
        let (_q, mut s) = sl.div_rem_euc((*N).clone());

        if s > ((*N).clone() / 2) {
            s = (*N).clone() - s;
        };

        Signature { r, s }
    }

    fn hmac_for_data(data: &[u8], mut k: [u8; 32]) -> [u8; 32] {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

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
        let z_bytes: [u8; 32] = vect_to_array_32(&z_vect);

        let secret_vect: Vec<u8> = secret.to_digits::<u8>(Order::Msf);
        let secret_bytes: [u8; 32] = vect_to_array_32(&secret_vect);

        let mut k: [u8; 32] = [0u8; 32];
        let mut v: [u8; 32] = [1u8; 32];

        let mut data: Vec<u8> = [v.as_slice(), zero.as_slice(), &secret_bytes, &z_bytes].concat();
        k = Key::hmac_for_data(&data, k);
        v = Key::hmac_for_data(&v, k);

        data = [v.as_slice(), one.as_slice(), &secret_bytes, &z_bytes].concat();
        k = Key::hmac_for_data(&data, k);
        v = Key::hmac_for_data(&v, k);

        loop {
            v = Key::hmac_for_data(&v, k);
            let candidate: Integer = Integer::from_digits(&v, Order::Msf);

            if candidate >= 1 && candidate < *N {
                return candidate;
            }

            data = [v.as_slice(), zero.as_slice()].concat();
            k = Key::hmac_for_data(&data, k);
            v = Key::hmac_for_data(&v, k);
        }
    }

    pub fn public_key_sec(&self) -> Vec<u8> {
        self.public_key.serialize(Compression::Compressed)
    }

    pub fn to_wif(&self, compression: Compression, network: Network) -> String {
        let secret_bytes = self.private_key.to_digits::<u8>(Order::Msf);
        let secret_bytes_padded = padding_left(&secret_bytes, 32, 0);

        let prefix = Self::wif_network_prefix(network);
        let suffix = Self::wif_compression_prefix(compression);
        let data = [prefix.as_slice(), &secret_bytes_padded, suffix.as_slice()].concat();

        base58::base58_encode_with_checksum(&data)
    }

    fn wif_network_prefix(network: Network) -> Vec<u8> {
        match network {
            Network::Mainnet => vec![AddressPrefix::PrivateKeyMainnet as u8],
            Network::Testnet => vec![AddressPrefix::PrivateKeyTestnet as u8],
        }
    }

    fn wif_compression_prefix(compression: Compression) -> Vec<u8> {
        match compression {
            Compression::Uncompressed => vec![],
            Compression::Compressed => vec![0x01],
        }
    }

    /// Verify `z` versus a `Signature`.
    /// `z`: the hashed message
    /// `sig`: the public signature
    pub fn verify_signature(point: &Point, z: &Integer, signature: &Signature) -> bool {
        let s_inv = signature.s.invert_by_modulo(&N);

        let mu = z * &s_inv;
        let (_q, u) = Integer::from(mu).div_rem_euc((*N).clone());

        let mv = &signature.r * &s_inv;
        let (_q, v) = Integer::from(mv).div_rem_euc((*N).clone());

        let total = (&(*G).clone() * u) + &(point * v);

        total.x_as_num() == signature.r
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Private key({}", self.public_key)
    }
}

#[cfg(test)]
mod private_key_test {
    use rug::{integer::Order, ops::Pow, Integer};

    use crate::{
        ecdsa::point::Point,
        flags::{compression::Compression, network::Network},
        hashing::hash256::hash256,
        keys::key::Key,
        std_lib::{
            integer_extended::IntegerExtended,
            vector::{string_to_bytes, vect_to_hex_string},
        },
    };

    #[test]
    fn verify_a_signature() {
        let secret = "A SECRET".to_string();
        let message = "A MESSAGE".to_string();
        let e = hash256(&secret.as_bytes().to_vec());
        let z = hash256(&message.as_bytes().to_vec());

        let e_integer = Integer::from_digits(&e, Order::Msf);
        let z_integer = Integer::from_digits(&z, Order::Msf);

        let private_key = Key::new(e_integer);
        let sign = private_key.sign(z_integer.clone());

        assert!(private_key.verify(&z_integer, &sign));
    }

    #[test]
    fn serialize_a_public_key_1() {
        let private_key = Key::new(Integer::from(5000));
        let sec = private_key.public_key.serialize(Compression::Uncompressed);
        assert_eq!(vect_to_hex_string(&sec), "04FFE558E388852F0120E46AF2D1B370F85854A8EB0841811ECE0E3E03D282D57C315DC72890A4F10A1481C031B03B351B0DC79901CA18A00CF009DBDB157A1D10");
    }

    #[test]
    fn serialize_a_public_key_2() {
        let private_key = Key::new(Integer::from(2018).pow(5));
        let sec = private_key.public_key.serialize(Compression::Uncompressed);
        assert_eq!(vect_to_hex_string(&sec), "04027F3DA1918455E03C46F659266A1BB5204E959DB7364D2F473BDF8F0A13CC9DFF87647FD023C13B4A4994F17691895806E1B40B57F4FD22581A4F46851F3B06");
    }

    #[test]
    fn serialize_a_public_key_3() {
        let private_key = Key::new(Integer::from_hex_str("DEADBEEF12345"));
        let sec = private_key.public_key.serialize(Compression::Uncompressed);
        assert_eq!(vect_to_hex_string(&sec), "04D90CD625EE87DD38656DD95CF79F65F60F7273B67D3096E68BD81E4F5342691F842EFA762FD59961D0E99803C61EDBA8B3E3F7DC3A341836F97733AEBF987121");
    }

    #[test]
    fn serialize_a_compressed_public_key_1() {
        let private_key = Key::new(Integer::from(5001));
        let sec = private_key.public_key.serialize(Compression::Compressed);
        assert_eq!(
            vect_to_hex_string(&sec),
            "0357A4F368868A8A6D572991E484E664810FF14C05C0FA023275251151FE0E53D1"
        );
    }

    #[test]
    fn serialize_a_compressed_public_key_2() {
        let private_key = Key::new(Integer::from(2019).pow(5));
        let sec = private_key.public_key.serialize(Compression::Compressed);
        assert_eq!(
            vect_to_hex_string(&sec),
            "02933EC2D2B111B92737EC12F1C5D20F3233A0AD21CD8B36D0BCA7A0CFA5CB8701"
        );
    }

    #[test]
    fn serialize_a_compressed_public_key_3() {
        let private_key = Key::new(Integer::from_hex_str("DEADBEEF54321"));
        let sec = private_key.public_key.serialize(Compression::Compressed);
        assert_eq!(
            vect_to_hex_string(&sec),
            "0296BE5B1292F6C856B3C5654E886FC13511462059089CDF9C479623BFCBE77690"
        );
    }

    #[test]
    fn deserialize_a_public_key_1() {
        let point_serialized = string_to_bytes("04FFE558E388852F0120E46AF2D1B370F85854A8EB0841811ECE0E3E03D282D57C315DC72890A4F10A1481C031B03B351B0DC79901CA18A00CF009DBDB157A1D10").unwrap();
        let point = Point::deserialize(point_serialized);

        assert_eq!(
            point.x_as_num(),
            Integer::from_hex_str("FFE558E388852F0120E46AF2D1B370F85854A8EB0841811ECE0E3E03D282D57C")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::from_hex_str("315DC72890A4F10A1481C031B03B351B0DC79901CA18A00CF009DBDB157A1D10")
        );
    }

    #[test]
    fn deserialize_a_public_key_2() {
        let point_serialized = string_to_bytes("04027F3DA1918455E03C46F659266A1BB5204E959DB7364D2F473BDF8F0A13CC9DFF87647FD023C13B4A4994F17691895806E1B40B57F4FD22581A4F46851F3B06").unwrap();
        let point = Point::deserialize(point_serialized);

        assert_eq!(
            point.x_as_num(),
            Integer::from_hex_str("027F3DA1918455E03C46F659266A1BB5204E959DB7364D2F473BDF8F0A13CC9D")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::from_hex_str("FF87647FD023C13B4A4994F17691895806E1B40B57F4FD22581A4F46851F3B06")
        );
    }

    #[test]
    fn deserialize_a_public_key_3() {
        let point_serialized = string_to_bytes("04D90CD625EE87DD38656DD95CF79F65F60F7273B67D3096E68BD81E4F5342691F842EFA762FD59961D0E99803C61EDBA8B3E3F7DC3A341836F97733AEBF987121").unwrap();
        let point = Point::deserialize(point_serialized);

        assert_eq!(
            point.x_as_num(),
            Integer::from_hex_str("D90CD625EE87DD38656DD95CF79F65F60F7273B67D3096E68BD81E4F5342691F")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::from_hex_str("842EFA762FD59961D0E99803C61EDBA8B3E3F7DC3A341836F97733AEBF987121")
        );
    }

    #[test]
    fn deserialize_a_compressed_public_key_1() {
        let point_serialized =
            string_to_bytes("0357A4F368868A8A6D572991E484E664810FF14C05C0FA023275251151FE0E53D1").unwrap();
        let point = Point::deserialize(point_serialized);

        assert_eq!(
            point.x_as_num(),
            Integer::from_hex_str("57A4F368868A8A6D572991E484E664810FF14C05C0FA023275251151FE0E53D1")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::from_hex_str("D6CC87C5BC29B83368E17869E964F2F53D52EA3AA3E5A9EFA1FA578123A0C6D")
        );
    }

    #[test]
    fn deserialize_a_compressed_public_key_2() {
        let point_serialized =
            string_to_bytes("02933EC2D2B111B92737EC12F1C5D20F3233A0AD21CD8B36D0BCA7A0CFA5CB8701").unwrap();
        let point = Point::deserialize(point_serialized);

        assert_eq!(
            point.x_as_num(),
            Integer::from_hex_str("933EC2D2B111B92737EC12F1C5D20F3233A0AD21CD8B36D0BCA7A0CFA5CB8701")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::from_hex_str("96CBBFDD572F75ACE44D0AA59FBAB6326CB9F909385DCD066EA27AFFEF5A488C")
        );
    }

    #[test]
    fn deserialize_a_compressed_public_key_3() {
        let point_serialized =
            string_to_bytes("0296BE5B1292F6C856B3C5654E886FC13511462059089CDF9C479623BFCBE77690").unwrap();
        let point = Point::deserialize(point_serialized);

        assert_eq!(
            point.x_as_num(),
            Integer::from_hex_str("96BE5B1292F6C856B3C5654E886FC13511462059089CDF9C479623BFCBE77690")
        );
        assert_eq!(
            point.y_as_num(),
            Integer::from_hex_str("32555D1B027C25C2828BA96A176D78419CD1236F71558F6187AEC09611325EB6")
        );
    }

    #[test]
    fn deterministic_k_1() {
        let k = Key::deterministic_k(&Integer::from(10), &Integer::from(1));
        assert_eq!(
            k,
            Integer::from_dec_str("23556289421633918234640030791776902309669950917001758018452865836473455104574")
        );
    }

    #[test]
    fn deterministic_k_2() {
        let k = Key::deterministic_k(&Integer::from(2345), &Integer::from(6789));
        assert_eq!(
            k,
            Integer::from_dec_str("34113680596947005563568962966999203522429670732921816689907697765389746251584")
        );
    }

    #[test]
    fn deterministic_k_3() {
        let k = Key::deterministic_k(&Integer::from(1000000), &Integer::from(1000000));
        assert_eq!(
            k,
            Integer::from_dec_str("35877450084421794080905523995859466786371393244910114637747627798158238933625")
        );
    }

    #[test]
    fn address_1() {
        let private_key = Key::new(Integer::from(5002));
        let addr = private_key.address(Compression::Uncompressed, Network::Testnet);

        assert_eq!("mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA", addr);
    }

    #[test]
    fn address_2() {
        let private_key = Key::new(Integer::from(2020).pow(5));
        let addr = private_key.address(Compression::Compressed, Network::Testnet);

        assert_eq!("mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH", addr);
    }

    #[test]
    fn address_3() {
        let private_key = Key::new(Integer::from_hex_str("12345deadbeef"));
        let addr = private_key.address(Compression::Compressed, Network::Mainnet);

        assert_eq!("1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1", addr);
    }

    #[test]
    fn wif_1() {
        let private_key = Key::new(Integer::from(5003));
        let wif = private_key.to_wif(Compression::Compressed, Network::Testnet);

        assert_eq!("cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN8rFTv2sfUK", wif);
    }

    #[test]
    fn wif_2() {
        let private_key = Key::new(Integer::from(2021).pow(5));
        let addr = private_key.to_wif(Compression::Uncompressed, Network::Testnet);

        assert_eq!("91avARGdfge8E4tZfYLoxeJ5sGBdNJQH4kvjpWAxgzczjbCwxic", addr);
    }

    #[test]
    fn wif_3() {
        let private_key = Key::new(Integer::from_hex_str("54321deadbeef"));
        let addr = private_key.to_wif(Compression::Compressed, Network::Mainnet);

        assert_eq!("KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgiuQJv1h8Ytr2S53a", addr);
    }
}
