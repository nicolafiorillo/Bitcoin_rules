use crate::std_lib::varstring::VarString;

use super::network_address::NetworkAddress;

// https://en.bitcoin.it/wiki/Protocol_documentation#version

pub struct Version {
    version: u32,   // LE
    service: u64,   // LE
    timestamp: u64, // LE
    receiver: NetworkAddress,

    // if version >= 106, then the following fields are present
    sender: NetworkAddress,
    nonce: u64,
    user_agent: VarString,
    height: u32,

    // if version >= 70001, then the following field is present (BIP-0037)
    relay: u8, // 0x00 or 0x01
}

static LAST_VERSION: u32 = 70015;
static AGENT: &str = "/Bitcoin_rules!:0.0/";

impl Version {
    pub fn new(receiver: NetworkAddress, sender: NetworkAddress) -> Self {
        let version = LAST_VERSION;
        let user_agent = VarString::new(AGENT);
        let nonce = 0; // generate_rand_64();
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

        let version_message = Version::new(receiver, sender);
        let serialized_version = version_message.serialize();

        let expected = hex_string_to_bytes("7F11010000000000000000000000000000000000000000000000000000000000000000000000FFFF00000000208D000000000000000000000000000000000000FFFF00000000208D0000000000000000142F426974636F696E5F72756C6573213A302E302F0000000000").unwrap();

        assert_eq!(serialized_version, expected);
    }
}
