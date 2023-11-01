use rug::{
    rand::{ThreadRandGen, ThreadRandState},
    Integer,
};

use crate::{
    bitcoin::ecdsa::P,
    flags::{compression::Compression, network::Network},
    keys::key::Key,
};

struct Seed(*const ());
impl ThreadRandGen for Seed {
    fn gen(&mut self) -> u32 {
        generate_seed()
    }
}

// Some other ideas at https://blog.orhun.dev/zero-deps-random-in-rust/
fn generate_seed() -> u32 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    RandomState::new().build_hasher().finish() as u32
}

// Usage:
//  let (secret, address) = new(Network::Testnet);
pub fn new(network: Network) -> (Integer, String) {
    let mut seed = Seed(&());
    let mut rand = ThreadRandState::new_custom(&mut seed);

    let max_value = (*P).clone();
    let secret = max_value.random_below(&mut rand);

    let private_key = Key::new(secret.clone());
    let addr = private_key.address(Compression::Compressed, network);

    (secret, addr)
}
