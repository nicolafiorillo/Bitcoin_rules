// To implement behaviour, see:
// https://en.bitcoin.it/wiki/Protocol_documentation#feefilter
// https://github.com/bitcoin/bips/blob/master/bip-0133.mediawiki

use crate::{std_lib::std_result::StdResult, transaction::tx_lib::le_bytes_to_u64};

#[derive(Debug, PartialEq)]
pub struct FeeFilter {
    pub feerate: u64, // LE - Represents a minimal fee and is expressed in satoshis per 1000 bytes
}

impl FeeFilter {
    pub fn new(feerate: u64) -> Self {
        Self { feerate }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.feerate.to_le_bytes().to_vec()
    }

    pub fn deserialize(buf: &[u8]) -> StdResult<Self> {
        let feerate = le_bytes_to_u64(buf, 0)?;
        Ok(Self { feerate })
    }
}

#[cfg(test)]
mod feefilter_test {

    use super::FeeFilter;

    #[test]
    fn serialize() {
        let feefilter = FeeFilter::new(123456789);
        let serialized_feefilter = feefilter.serialize();

        assert_eq!(serialized_feefilter, vec![21, 205, 91, 7, 0, 0, 0, 0]);
    }

    #[test]
    fn deserialize() {
        let serialized_feefilter = [21, 205, 91, 7, 0, 0, 0, 0];
        let feefilter = FeeFilter::deserialize(&serialized_feefilter).unwrap();

        assert_eq!(feefilter.feerate, 123456789);
    }
}
