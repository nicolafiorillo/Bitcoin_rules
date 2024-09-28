// https://en.bitcoin.it/wiki/Protocol_documentation#ping

use crate::{std_lib::std_result::StdResult, transaction::tx_lib::le_bytes_to_u64};

#[derive(Debug, PartialEq)]
pub struct Ping {
    pub nonce: u64, // LE?
}

impl Ping {
    pub fn new(nonce: u64) -> Self {
        Self { nonce }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.nonce.to_le_bytes().to_vec()
    }

    pub fn deserialize(buf: &[u8]) -> StdResult<Self> {
        let nonce = le_bytes_to_u64(buf, 0)?;
        Ok(Self { nonce })
    }
}

#[cfg(test)]
mod ping_test {
    use super::Ping;

    #[test]
    fn serialize() {
        let ping = Ping::new(123456789);
        let serialized_ping = ping.serialize();

        assert_eq!(serialized_ping, vec![21, 205, 91, 7, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize() {
        let serialized_ping = [21, 205, 91, 7, 0, 0, 0, 0];
        let ping = Ping::deserialize(&serialized_ping).unwrap();

        assert_eq!(ping.nonce, 123456789);
    }
}
