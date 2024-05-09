use crate::{
    std_lib::{
        std_result::StdResult,
        varstring::{self, VarString},
    },
    transaction::tx_lib::{le_bytes_to_u32, le_bytes_to_u64},
};

use super::{constants, network_address::NetworkAddress};

// https://en.bitcoin.it/wiki/Protocol_documentation#version
#[derive(Debug, PartialEq)]
pub struct Version {
    pub version: u32, // LE
    service: u64,     // LE
    timestamp: u64,   // LE
    receiver: NetworkAddress,

    // if version >= 106, then the following fields are present
    sender: NetworkAddress,
    nonce: u64,
    user_agent: VarString,
    pub height: u32,

    // if version >= 70001, then the following field is present (BIP-0037)
    relay: u8, // 0x00 or 0x01
}

impl Version {
    pub fn new(receiver: NetworkAddress, sender: NetworkAddress, nonce: u64, agent: &str) -> Self {
        let version = constants::LAST_VERSION;
        let user_agent = VarString::new(agent);
        let relay = 0x00;
        let service = 0;
        let timestamp = 0;
        let height = 0;
        Self {
            version,
            service,
            timestamp,
            receiver,
            sender,
            nonce,
            user_agent,
            height,
            relay,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut v = vec![];

        // TODO: reserve_exact, calculating also the size of the VarString

        v.extend_from_slice(&self.version.to_le_bytes());
        v.extend_from_slice(&self.service.to_le_bytes());
        v.extend_from_slice(&self.timestamp.to_le_bytes());
        v.extend_from_slice(&self.receiver.serialize(false));
        v.extend_from_slice(&self.sender.serialize(false));
        v.extend_from_slice(&self.nonce.to_le_bytes());
        v.extend_from_slice(&self.user_agent.encode());
        v.extend_from_slice(&self.height.to_le_bytes());
        v.push(self.relay);
        v
    }

    pub fn deserialize(buf: &[u8]) -> StdResult<Self> {
        let mut offset: usize = 0;

        let version = le_bytes_to_u32(buf, offset)?;
        offset += 4;

        let service = le_bytes_to_u64(buf, offset)?;
        offset += 8;

        let timestamp = le_bytes_to_u64(buf, offset)?;
        offset += 8;

        let (receiver, off) = NetworkAddress::deserialize(&buf[offset..], false)?;
        offset += off;

        let (sender, off) = NetworkAddress::deserialize(&buf[offset..], false)?;
        offset += off;

        let nonce = le_bytes_to_u64(buf, offset)?;
        offset += 8;

        let user_agent = varstring::decode(&buf[offset..], 0)?;
        offset += user_agent.length.length + user_agent.length.value as usize;

        let height = le_bytes_to_u32(buf, offset)?;
        offset += 4;

        let relay = buf[offset];

        let v = Version {
            version,
            service,
            timestamp,
            receiver,
            sender,
            nonce,
            user_agent,
            height,
            relay,
        };

        Ok(v)
    }
}

#[cfg(test)]
mod version_test {
    use crate::network::ip_address;
    use crate::network::network_address::NetworkAddress;
    use crate::network::version::Version;
    use crate::std_lib::vector::hex_string_to_bytes;

    #[test]
    fn serialize() {
        let address = ip_address::parse_address("0.0.0.0").unwrap();

        let receiver = NetworkAddress::new(0, 0, address, 8333);
        let sender = NetworkAddress::new(0, 0, address, 8333);

        let agent = "/Bitcoin_rules!:0.0/";

        let version_message = Version::new(receiver, sender, 0, agent);
        let serialized_version = version_message.serialize();

        let expected = hex_string_to_bytes("7F11010000000000000000000000000000000000000000000000000000000000000000000000FFFF00000000208D000000000000000000000000000000000000FFFF00000000208D0000000000000000142F426974636F696E5F72756C6573213A302E302F0000000000").unwrap();

        assert_eq!(serialized_version, expected);
    }

    #[test]
    fn deserialize() {
        let serialized_version = [
            128, 17, 1, 0, 9, 4, 0, 0, 0, 0, 0, 0, 167, 133, 8, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 195, 176, 217, 39, 151, 2, 56, 154, 16, 47, 83, 97, 116, 111, 115, 104, 105, 58, 50, 54, 46, 48,
            46, 48, 47, 47, 101, 39, 0, 1,
        ];

        let version = Version::deserialize(&serialized_version).unwrap();
        assert_eq!(version.version, 70016);
        assert_eq!(version.service, 1033);
        assert_eq!(version.timestamp, 1711834535);
        assert_eq!(version.receiver.time, 0);
        assert_eq!(version.sender.time, 0);
        assert_eq!(version.nonce, 11112634928768594115);
        assert_eq!(String::from_utf8(version.user_agent.value).unwrap(), "/Satoshi:26.0.0/");
        assert_eq!(version.height, 2581807);
        assert_eq!(version.relay, 1);
    }
}
