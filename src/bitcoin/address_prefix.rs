use std::fmt::{Display, Formatter, Result};

use crate::flags::network::Network;

pub enum AddressPrefix {
    PrivateKeyP2pkhMainnet = Network::Mainnet as isize,
    // PRIVATE_KEY_P2SH_MAINNET = 0x05,
    PrivateKeyMainnet = 0x80,
    // PUBLIC_KEY_BIP32_MAINNET = 0x0488B21E,
    // PRIVATE_KEY_BIP32_MAINNET = 0x0488ADE4,
    PublicKeyP2pkhTestnet = Network::Testnet as isize,
    // PUBLIC_KEY_SCRIPT_TESTNET = 0xC4,
    PrivateKeyTestnet = 0xEF,
    // PUBLIC_KEY_BIP32_TESTNET = 0x043587CF,
    // PRIVATE_KEY_BIP32_TESTNET = 0x04358394,
    // EXT_PUBLIC_KEY_SEGWIT = 0x04B24746,
    // EXT_SECRET_KEY_SEGWIT = 0x04B2430C,
}

impl Display for AddressPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let n = match self {
            AddressPrefix::PrivateKeyP2pkhMainnet => "PrivateKeyP2pkhMainnet",
            AddressPrefix::PrivateKeyMainnet => "PrivateKeyMainnet",
            AddressPrefix::PublicKeyP2pkhTestnet => "PublicKeyP2pkhTestnet",
            AddressPrefix::PrivateKeyTestnet => "PrivateKeyTestnet",
        };
        writeln!(f, "{:}", n)
    }
}
