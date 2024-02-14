// Some other ideas at https://blog.orhun.dev/zero-deps-random-in-rust/
pub fn generate_rand_32() -> u32 {
    generate_rand_64() as u32
}

pub fn generate_rand_64() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    RandomState::new().build_hasher().finish()
}
