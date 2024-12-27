// https://en.bitcoin.it/wiki/Protocol_documentation

use std::fmt::{Display, Formatter};

use crate::{
    flags::network_magic::NetworkMagic, hashing::hash256::Hash256, network::headers::Headers,
    std_lib::std_result::StdResult,
};

use super::{command::*, fee_filter::FeeFilter, ping::Ping, send_compact::SendCompact, version::Version};

static PAYLOAD_SIZE: usize = 32_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkMessage {
    pub magic: NetworkMagic,
    pub command: Command,
    pub payload: Vec<u8>,
    pub len_serialized: usize,
}

fn message_checksum(payload: &[u8]) -> [u8; 4] {
    Hash256::calc(payload).into()
}

impl NetworkMessage {
    pub fn new(command: Command, payload: Vec<u8>, magic: NetworkMagic) -> StdResult<Self> {
        if payload.len() > PAYLOAD_SIZE {
            return Err("network_message_payload_too_big".into());
        }

        let len_serialized = 24 + payload.len();

        let network_message = NetworkMessage {
            magic,
            command,
            payload,
            len_serialized,
        };

        Ok(network_message)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.magic.to_le_bytes());

        buf.extend_from_slice(&self.command.bytes);

        let lenght = self.payload.len() as u32;
        buf.extend_from_slice(&lenght.to_le_bytes());

        let payload_checksum = message_checksum(&self.payload);

        buf.extend_from_slice(&payload_checksum);
        buf.extend_from_slice(&self.payload);
        buf
    }
}

impl From<NetworkMessage> for StdResult<Commands> {
    fn from(val: NetworkMessage) -> Self {
        match val.command {
            VERACK_COMMAND => Ok(Commands::VerAck),
            VERSION_COMMAND => {
                let payload = Version::deserialize(&val.payload)?;
                Ok(Commands::Version(payload))
            }
            SEND_COMPACT_COMMAND => {
                let payload = SendCompact::deserialize(&val.payload)?;
                Ok(Commands::SendCompact(payload))
            }
            PING_COMMAND => {
                let payload = Ping::deserialize(&val.payload)?;
                Ok(Commands::Ping(payload))
            }
            FEE_FILTER_COMMAND => {
                let payload = FeeFilter::deserialize(&val.payload)?;
                Ok(Commands::FeeFilter(payload))
            }
            WTXID_RELAY_COMMAND => Ok(Commands::WtxIdRelay),
            SENDADDRV2_COMMAND => Ok(Commands::SendAddrV2),
            HEADERS_COMMAND => {
                let payload = Headers::deserialize(&val.payload)?;
                Ok(Commands::Headers(payload))
            }
            _ => panic!("unknown_command: {:?}", val.command),
        }
    }
}

impl Display for NetworkMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "magic: {}, command: {}, payload: {:?}",
            self.magic, self.command, self.payload
        )
    }
}

#[cfg(test)]
mod network_message_test {
    use crate::{
        network::command::{VERACK_COMMAND, VERSION_COMMAND},
        std_lib::vector::bytes_to_hex_string,
    };

    use super::*;

    #[test]
    fn network_message_new() {
        let payload = vec![0; 100];
        let magic = NetworkMagic::Mainnet;
        let network_message = NetworkMessage::new(VERSION_COMMAND, payload, magic).unwrap();
        assert_eq!(network_message.magic, NetworkMagic::Mainnet);
        assert_eq!(network_message.command, VERSION_COMMAND);
        assert_eq!(network_message.payload, vec![0; 100]);
    }

    #[test]
    fn network_message_serialize() {
        let payload = vec![0; 0];
        let magic = NetworkMagic::Mainnet;
        let network_message = NetworkMessage::new(VERACK_COMMAND, payload, magic).unwrap();
        let bytes = network_message.serialize();
        let serialized = bytes_to_hex_string(&bytes);

        assert_eq!(serialized, "F9BEB4D976657261636B000000000000000000005DF6E0E2");
    }
}
