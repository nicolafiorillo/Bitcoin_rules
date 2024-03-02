use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy)]
pub enum Network {
    Mainnet = 0x00,
    Testnet = 0x6F,
    // Signet = 0x7B,
    // Regtest = 0xC4,
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let n = match self {
            Network::Mainnet => "Mainnet",
            Network::Testnet => "Testnet",
        };
        writeln!(f, "{:}", n)
    }
}
