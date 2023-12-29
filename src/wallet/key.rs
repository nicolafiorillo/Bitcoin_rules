// To create a more deterministic tests, See also
// https://en.wikipedia.org/wiki/Linear_congruential_generator to generate
// a deterministic seed within a sequence of pseudo-randomized numbers

use std::fmt::{Display, Formatter};

use rug::{
    rand::{ThreadRandGen, ThreadRandState},
    Integer,
};

use crate::{
    bitcoin::ecdsa::P,
    flags::{compression::Compression, network::Network},
    keys::key::Key,
    std_lib::vector::bytes_to_string,
};

struct Seed(*const ());
impl ThreadRandGen for Seed {
    fn gen(&mut self) -> u32 {
        generate_seed()
    }
}

#[derive(Debug)]
pub struct UserKey {
    pub secret: Integer,
    pub pubkey: Vec<u8>,
    pub address: String,
    pub key: Key,
}

impl Display for UserKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "privkey: {:}\npubkey: {:} (len: {:})\naddress: {:}",
            self.secret,
            bytes_to_string(&self.pubkey),
            self.pubkey.len(),
            self.address,
        )
    }
}

// Some other ideas at https://blog.orhun.dev/zero-deps-random-in-rust/
fn generate_seed() -> u32 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    RandomState::new().build_hasher().finish() as u32
}

// Deep investigate using /dev/urandom
// see also https://github.com/trezor/trezor-firmware/blob/main/core/embed/trezorhal/unix/rng.c#L26
#[allow(dead_code)]
fn get_seed_from_system() -> u32 {
    use std::{fs::File, io::Read};

    let mut rnd = File::open("/dev/urandom").unwrap();

    let mut buffer = [0u8; 4];
    rnd.read_exact(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}

// Usage:
//  let user_key = new(Network::Testnet);
pub fn new(network: Network) -> UserKey {
    let mut seed = Seed(&());
    let mut rand = ThreadRandState::new_custom(&mut seed);

    let max_value = (*P).clone();
    let secret = max_value.random_below(&mut rand);

    let key = Key::new(secret.clone());
    let address = key.address(Compression::Compressed, network);
    let pubkey = key.public_key_sec();

    UserKey {
        secret,
        pubkey,
        address,
        key,
    }
}
