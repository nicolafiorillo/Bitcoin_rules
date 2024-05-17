// To implement behaviour, see:
// https://en.bitcoin.it/wiki/Protocol_documentation#sendcmpct
// https://github.com/bitcoin/bips/blob/master/bip-0152.mediawiki

use crate::{std_lib::std_result::StdResult, transaction::tx_lib::le_bytes_to_u64};

#[derive(Debug, PartialEq)]
pub struct SendCompact {
    pub compact: bool,
    pub version_number: u64, // LE
}

impl SendCompact {
    pub fn new(compact: bool, version_number: u64) -> Self {
        Self {
            compact,
            version_number,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];
        let compact = if self.compact { 1 } else { 0 };

        v.push(compact);
        v.extend_from_slice(&self.version_number.to_le_bytes());

        v
    }

    pub fn deserialize(buf: &[u8]) -> StdResult<Self> {
        let compact = buf[0] == 1;
        let version_number = le_bytes_to_u64(&buf[1..], 0)?;

        let s = Self {
            compact,
            version_number,
        };

        Ok(s)
    }
}

#[cfg(test)]
mod send_compact_test {

    use super::SendCompact;

    #[test]
    fn serialize_1() {
        let send_compact = SendCompact::new(true, 123456789);
        let serialized_send_compact = send_compact.serialize();

        assert_eq!(serialized_send_compact, vec![1, 21, 205, 91, 7, 0, 0, 0, 0]);
    }

    #[test]
    fn serialize_2() {
        let send_compact = SendCompact::new(false, 987654321);
        let serialized_send_compact = send_compact.serialize();

        assert_eq!(serialized_send_compact, vec![0, 177, 104, 222, 58, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize_1() {
        let serialized_send_compact = [1, 21, 205, 91, 7, 0, 0, 0, 0];
        let send_compact = SendCompact::deserialize(&serialized_send_compact).unwrap();

        assert!(send_compact.compact);
        assert_eq!(send_compact.version_number, 123456789);
    }

    #[test]
    fn deserialize_2() {
        let serialized_send_compact = [0, 177, 104, 222, 58, 0, 0, 0, 0];
        let send_compact = SendCompact::deserialize(&serialized_send_compact).unwrap();

        assert!(!send_compact.compact);
        assert_eq!(send_compact.version_number, 987654321);
    }
}
