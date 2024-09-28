// https://en.bitcoin.it/wiki/Protocol_documentation#headers

use crate::{block::header::Header, std_lib::std_result::StdResult, transaction::tx_lib::varint_decode};

#[derive(Debug, Clone, PartialEq)]
pub struct Headers(pub Vec<Header>);

// Note from https://en.bitcoin.it/wiki/Protocol_documentation#headers:
// the block headers include a var_int transaction count so there can be more than 81 bytes per header,
// as opposed to the block headers that are hashed by miners.
// So, adding 1 for the tx_count varint (that shoul be always 0).
static EXPECTED_HEADER_LENGHT: usize = crate::block::header::HEADER_LENGTH + 1;

impl Headers {
    pub fn deserialize(buf: &[u8]) -> StdResult<Self> {
        let buf_lenght = buf.len();

        let headers_length = varint_decode(buf, 0)?;
        let rest = &buf[headers_length.length..];

        let mut headers: Vec<Header> = Vec::with_capacity(headers_length.value as usize);
        let mut cursor: usize = 0;

        for _i in 0..headers_length.value {
            if cursor > buf_lenght || cursor + EXPECTED_HEADER_LENGHT > buf_lenght {
                return Err("headers_length_mismatch".into());
            }

            // TODO: check if there is enough bytes to read
            let bytes = &rest[cursor..cursor + EXPECTED_HEADER_LENGHT];

            let header = Header::deserialize(bytes)?;
            headers.push(header);
            cursor += EXPECTED_HEADER_LENGHT;
        }

        if headers_length.value as usize != headers.len() {
            return Err("headers_length_mismatch".into());
        }

        Ok(Self(headers))
    }
}

#[cfg(test)]
mod headers_test {
    // use super::Ping;

    // #[test]
    // fn deserialize() {
    //     let serialized_ping = [21, 205, 91, 7, 0, 0, 0, 0];
    //     let ping = Ping::deserialize(&serialized_ping).unwrap();

    //     assert_eq!(ping.nonce, 123456789);
    // }
}
