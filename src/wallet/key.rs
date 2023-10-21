use rug::{
    ops::Pow,
    rand::{ThreadRandGen, ThreadRandState},
    Integer,
};
use std::time::SystemTime;

use crate::{
    flags::{compression::Compression, network::Network},
    keys::key::Key,
};

struct Seed(*const ());
impl ThreadRandGen for Seed {
    fn gen(&mut self) -> u32 {
        get_sys_time()
    }
}

// Usage:
//  let (secret, address) = generate_key(Network::Testnet);
pub fn new(network: Network) -> (Integer, String) {
    let mut seed = Seed(&());
    let mut rand = ThreadRandState::new_custom(&mut seed);

    let i = Integer::from(2).pow(256);
    let secret = i.random_below(&mut rand);

    let private_key = Key::new(secret.clone());
    let addr = private_key.address(Compression::Compressed, network);

    (secret, addr)
}

fn get_sys_time() -> u32 {
    let s = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    s.subsec_nanos() & 0xFFFF
}
