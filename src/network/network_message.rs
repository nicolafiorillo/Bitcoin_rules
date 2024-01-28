// https://en.bitcoin.it/wiki/Protocol_documentation

use crate::{
    flags::network_magic::NetworkMagic,
    hashing::hash256::hash256,
    std_lib::{
        std_result::StdResult,
        vector::{padding_right, trim_right},
    },
    transaction::tx_lib::le_bytes_to_u32,
};

type Command = String;

#[derive(Debug, Clone)]
pub struct NetworkMessage {
    pub magic: NetworkMagic,
    pub command: Command,
    pub payload: Vec<u8>,
}

static PAYLOAD_SIZE: usize = 32_000_000;

fn message_checksum(payload: &[u8]) -> [u8; 4] {
    let hash = hash256(payload);
    let mut checksum = [0; 4];
    checksum.copy_from_slice(&hash[..4]);
    checksum
}

impl NetworkMessage {
    pub fn new(cmd: &str, payload: Vec<u8>, magic: NetworkMagic) -> StdResult<NetworkMessage> {
        if payload.len() > PAYLOAD_SIZE {
            return Err("network_message_payload_too_big".into());
        }

        let command = cmd.to_string();

        let network_message = NetworkMessage {
            magic,
            command,
            payload,
        };

        Ok(network_message)
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(&self.magic.to_le_bytes());

        let padded_commend = padding_right(self.command.as_bytes(), 12, 0);
        buf.extend(padded_commend);

        let lenght = self.payload.len() as u32;
        buf.extend(lenght.to_le_bytes());

        let payload_checksum = message_checksum(&self.payload);

        buf.extend(payload_checksum);
        buf.extend(self.payload.as_slice());
        buf
    }

    pub fn deserialize(buf: &[u8], network: NetworkMagic) -> StdResult<NetworkMessage> {
        let magic_int = le_bytes_to_u32(buf, 0)?;
        let magic: NetworkMagic = magic_int.into();
        if magic != network {
            return Err("network_message_magic_mismatch".into());
        }

        let cmd_buf = trim_right(&buf[4..16], 0);

        let command = String::from_utf8_lossy(&cmd_buf).to_string();
        let payload_lenght = le_bytes_to_u32(buf, 16)?;

        let mut payload_checksum: [u8; 4] = [0; 4];
        payload_checksum.copy_from_slice(&buf[20..24]);

        let payload = buf[24..].to_vec();
        if payload.len() != payload_lenght as usize {
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

#[cfg(test)]
mod network_message_test {
    use crate::std_lib::vector::{bytes_to_string, hex_string_to_bytes};

    use super::*;

    #[test]
    fn network_message_new() {
        let command = "version";
        let payload = vec![0; 100];
        let magic = NetworkMagic::Mainnet;
        let network_message = NetworkMessage::new(command, payload, magic).unwrap();
        assert_eq!(network_message.magic, NetworkMagic::Mainnet);
        assert_eq!(network_message.command, "version");
        assert_eq!(network_message.payload, vec![0; 100]);
    }

    #[test]
    fn network_message_serialize() {
        let command = "verack";
        let payload = vec![0; 0];
        let magic = NetworkMagic::Mainnet;
        let network_message = NetworkMessage::new(command, payload, magic).unwrap();
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
        assert_eq!(network_message.command, "verack");
        assert_eq!(network_message.payload, vec![0; 0]);
    }
}
