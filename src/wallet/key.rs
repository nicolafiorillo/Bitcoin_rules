use std::fmt::{Display, Formatter};

use rug::{
    rand::{ThreadRandGen, ThreadRandState},
    Integer,
};

use crate::{
    bitcoin::ecdsa::P,
    flags::{compression::Compression, network::Network},
    keys::key::Key,
    std_lib::vector::vect_to_hex_string,
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
}

impl Display for UserKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "privkey: {:}\npubkey:  {:} (len: {:})\naddress: {:}",
            self.secret,
            vect_to_hex_string(&self.pubkey),
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
    }
}
