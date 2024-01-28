use std::{
    convert::Into,
    fmt::{Display, Formatter, Result},
};

const MAGIC_MAINNET: u32 = 0xD9B4BEF9;
const MAGIC_TESTNET: u32 = 0xDAB5BFFA;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMagic {
    Mainnet,
    Testnet,
    // Testnet3 = 0x0709110B,
    // Signet = 0x40CF030A,
}

impl Display for NetworkMagic {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let n = match self {
            NetworkMagic::Mainnet => "Mainnet",
            NetworkMagic::Testnet => "Testnet",
        };
        writeln!(f, "{:}", n)
    }
}

impl From<NetworkMagic> for u32 {
    fn from(val: NetworkMagic) -> Self {
        match val {
            NetworkMagic::Mainnet => MAGIC_MAINNET,
            NetworkMagic::Testnet => MAGIC_TESTNET,
        }
    }
}

impl From<u32> for NetworkMagic {
    fn from(n: u32) -> Self {
        match n {
            MAGIC_MAINNET => NetworkMagic::Mainnet,
            MAGIC_TESTNET => NetworkMagic::Testnet,
            _ => panic!("unknown_network_magic"), // TODO: no panic here
        }
    }
}

impl NetworkMagic {
    pub fn to_le_bytes(self) -> [u8; 4] {
        let n: u32 = self.into();
        n.to_le_bytes()
    }
}
