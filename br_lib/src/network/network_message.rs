// https://en.bitcoin.it/wiki/Protocol_documentation

use std::fmt::{Display, Formatter};

use crate::{
    flags::network_magic::NetworkMagic, hashing::hash256::hash256, std_lib::std_result::StdResult,
    transaction::tx_lib::le_bytes_to_u32,
};

use super::{
    command::{Command, Commands, SEND_COMPACT_COMMAND, VERACK_COMMAND, VERSION_COMMAND},
    version::Version,
};

static PAYLOAD_SIZE: usize = 32_000_000;

#[derive(Debug, Clone)]
pub struct NetworkMessage {
    pub magic: NetworkMagic,
    pub command: Command,
    pub payload: Vec<u8>,
}

fn message_checksum(payload: &[u8]) -> [u8; 4] {
    hash256(payload).into()
}

impl NetworkMessage {
    pub fn new(command: Command, payload: Vec<u8>, magic: NetworkMagic) -> StdResult<Self> {
        if payload.len() > PAYLOAD_SIZE {
            return Err("network_message_payload_too_big".into());
        }

        let network_message = NetworkMessage {
            magic,
            command,
            payload,
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

    pub fn deserialize(buf: &[u8], network: NetworkMagic) -> StdResult<Self> {
        let magic_int = le_bytes_to_u32(buf, 0)?;
        let magic: NetworkMagic = magic_int.into();
        if magic != network {
            return Err("network_message_magic_mismatch".into());
        }

        let c: [u8; 12] = buf[4..16].try_into().unwrap();
        let command = Command { bytes: c };

        let payload_lenght = le_bytes_to_u32(buf, 16)?;
        let payload_checksum: [u8; 4] = buf[20..24].try_into().unwrap();

        let payload = buf[24..].to_vec();
        if payload.len() != payload_lenght as usize {
            log::error!(
                "declared payload lenght: {}, real payload lenght: {}",
                payload_lenght,
                payload.len()
            );
            return Err("network_message_payload_lenght_mismatch".into());
        }

        let checksum = message_checksum(&payload);
        if checksum != payload_checksum {
            return Err("network_message_checksum_mismatch".into());
        }

        let network_message = NetworkMessage {
            magic,
            command,
            payload,
        };

        Ok(network_message)
    }
}

impl From<NetworkMessage> for Commands {
    fn from(val: NetworkMessage) -> Self {
        match val.command {
            VERACK_COMMAND => Commands::VerAck,
            VERSION_COMMAND => {
                let payload = Version::deserialize(&val.payload).unwrap();
                Commands::Version(payload)
            }
            SEND_COMPACT_COMMAND => Commands::SendCompact,
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
        std_lib::vector::{bytes_to_string, hex_string_to_bytes},
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
        let serialized = bytes_to_string(&bytes);

        assert_eq!(serialized, "F9BEB4D976657261636B000000000000000000005DF6E0E2");
    }

    #[test]
    fn network_message_deserialize() {
        let message = "F9BEB4D976657261636B000000000000000000005DF6E0E2";
        let bytes = hex_string_to_bytes(message).unwrap();

        let network_message = NetworkMessage::deserialize(&bytes, NetworkMagic::Mainnet).unwrap();
        assert_eq!(network_message.magic, NetworkMagic::Mainnet);
        assert_eq!(network_message.command, VERACK_COMMAND);
        assert_eq!(network_message.payload, vec![0; 0]);
    }

    #[test]
    fn receive_verack() {
        let received = [
            11, 17, 9, 7, 118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, 102, 0, 0, 0, 76, 149, 14, 113, 128, 17, 1,
            0, 9, 4, 0, 0, 0, 0, 0, 0, 233, 70, 12, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            114, 80, 174, 11, 149, 73, 11, 34, 16, 47, 83, 97, 116, 111, 115, 104, 105, 58, 50, 54, 46, 48, 46, 48, 47,
            47, 101, 39, 0, 1,
        ];
        let version_received = NetworkMessage::deserialize(&received, NetworkMagic::Testnet3).unwrap();
        let message: Commands = version_received.clone().into();

        assert!(matches!(message, Commands::Version { .. }));
    }
}
