// https://en.bitcoin.it/wiki/Protocol_documentation#pong

use crate::{std_lib::std_result::StdResult, transaction::tx_lib::le_bytes_to_u64};

#[derive(Debug, PartialEq)]
pub struct Pong {
    pub nonce: u64, // LE?
}

impl Pong {
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
mod pong_test {
    use super::Pong;

    #[test]
    fn serialize() {
        let pong = Pong::new(123456789);
        let serialized_pong = pong.serialize();

        assert_eq!(serialized_pong, vec![21, 205, 91, 7, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize() {
        let serialized_pong = [21, 205, 91, 7, 0, 0, 0, 0];
        let pong = Pong::deserialize(&serialized_pong).unwrap();

        assert_eq!(pong.nonce, 123456789);
    }
}
