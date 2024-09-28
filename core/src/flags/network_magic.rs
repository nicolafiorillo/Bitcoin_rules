use std::{
    convert::Into,
    fmt::{Display, Formatter, Result},
};

const MAGIC_MAINNET: u32 = 0xD9B4BEF9;
const MAGIC_TESTNET: u32 = 0xDAB5BFFA;
const MAGIC_TESTNET3: u32 = 0x0709110B;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMagic {
    Mainnet,
    Testnet,
    Testnet3,
    // Signet = 0x40CF030A,
}

impl Display for NetworkMagic {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let n = match self {
            NetworkMagic::Mainnet => "Mainnet",
            NetworkMagic::Testnet => "Testnet",
            NetworkMagic::Testnet3 => "Testnet3",
        };
        write!(f, "{:}", n)
    }
}

impl From<NetworkMagic> for u32 {
    fn from(val: NetworkMagic) -> Self {
        match val {
            NetworkMagic::Mainnet => MAGIC_MAINNET,
            NetworkMagic::Testnet => MAGIC_TESTNET,
            NetworkMagic::Testnet3 => MAGIC_TESTNET3,
        }
    }
}

impl From<u32> for NetworkMagic {
    fn from(n: u32) -> Self {
        match n {
            MAGIC_MAINNET => NetworkMagic::Mainnet,
            MAGIC_TESTNET => NetworkMagic::Testnet,
            MAGIC_TESTNET3 => NetworkMagic::Testnet3,
            _ => panic!("unknown_network_magic"), // TODO: no panic here
        }
    }
}

impl From<String> for NetworkMagic {
    fn from(v: String) -> Self {
        match v.trim().to_lowercase().as_str() {
            "mainnet" => NetworkMagic::Mainnet,
            "testnet" => NetworkMagic::Testnet,
            "testnet3" => NetworkMagic::Testnet3,
            _ => panic!("unknown_network_magic_string"), // TODO: no panic here
        }
    }
}

impl From<u8> for NetworkMagic {
    fn from(n: u8) -> Self {
        match n {
            0 => NetworkMagic::Mainnet,
            1 => NetworkMagic::Testnet,
            2 => NetworkMagic::Testnet3,
            _ => panic!("unknown_network_number"),
        }
    }
}

impl NetworkMagic {
    pub fn to_le_bytes(self) -> [u8; 4] {
        let n: u32 = self.into();
        n.to_le_bytes()
    }
}
