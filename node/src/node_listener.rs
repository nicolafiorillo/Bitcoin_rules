use core::{
    flags::network_magic::NetworkMagic,
    hashing::hash256::Hash256,
    network::{command::Command, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
    transaction::tx_lib::le_bytes_to_u32,
};
use tokio::{io::AsyncReadExt, net::tcp::OwnedReadHalf, sync::mpsc::Sender};

#[derive(Debug)]
pub struct NodeListener {
    reader: OwnedReadHalf,
    network: NetworkMagic,
}

impl NodeListener {
    pub fn new(reader: OwnedReadHalf, network: NetworkMagic) -> NodeListener {
        NodeListener { reader, network }
    }

    // Listen for incoming messages from peer
    // Emit a NetworkMessage to the sender channel
    pub async fn listen(&mut self, sender: Sender<NetworkMessage>) -> StdResult<()> {
        loop {
            let message = receive_message(&mut self.reader, self.network).await?;
            sender.send(message).await?;
        }
    }
}

//    #[cfg(inline)]
fn message_checksum(payload: &[u8]) -> [u8; 4] {
    Hash256::calc(payload).into()
}

async fn read_exact(reader: &mut OwnedReadHalf, buf: &mut [u8]) -> StdResult<()> {
    match reader.read_exact(buf).await {
        Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => {
            log::trace!("connection_reset_by_peer");
            return Err("connection_reset_by_peer".into());
        }

        Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
            /*
               ErrorKind::WouldBlock occurs when a non-blocking socket operation cannot complete immediately
               without blocking the container thread.
               This usually means that data isn't fully available or the buffer is full.
               The operation should be retried later.
            */
            log::trace!("read_exact: WouldBlock");
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        Err(err) => {
            return Err(err.into());
        }
        _ => {}
    }

    Ok(())
}

async fn receive_message(reader: &mut OwnedReadHalf, network: NetworkMagic) -> StdResult<NetworkMessage> {
    let mut four_bytes: [u8; 4] = [0; 4];
    read_exact(reader, &mut four_bytes).await?;
    let magic_int = le_bytes_to_u32(&four_bytes, 0)?;

    let magic: NetworkMagic = magic_int.into();
    if magic != network {
        return Err("network_message_magic_mismatch".into());
    }

    let mut twelve_bytes: [u8; 12] = [0; 12];
    read_exact(reader, &mut twelve_bytes).await?;
    let command = Command { bytes: twelve_bytes };

    four_bytes = [0; 4];
    read_exact(reader, &mut four_bytes).await?;
    let declared_payload_lenght = le_bytes_to_u32(&four_bytes, 0)?;

    four_bytes = [0; 4];
    read_exact(reader, &mut four_bytes).await?;
    let declared_payload_checksum = four_bytes;

    let mut payload = vec![0; declared_payload_lenght as usize];
    read_exact(reader, &mut payload).await?;

    let checksum = message_checksum(&payload);
    if checksum != declared_payload_checksum {
        return Err("network_message_checksum_mismatch".into());
    }

    let len_serialized = 24 + declared_payload_lenght as usize;
    Ok(NetworkMessage {
        magic,
        command,
        payload,
        len_serialized,
    })
}

// TODO: Add tests on receive_message
/*
enum MyAsyncRead {
    Tcp(tokio::net::tcp::OwnedReadHalf),
    Unix(tokio::net::unix::OwnedReadHalf),
}

#[cfg(test)]
mod node_listener_test {
    use core::{
        flags::network_magic::NetworkMagic, network::command::VERACK_COMMAND, std_lib::vector::hex_string_to_bytes,
    };
    use tokio::{io::AsyncWriteExt, net::UnixStream};

    use crate::node_listener::NodeListener;

    #[tokio::test]
    async fn network_message_deserialize() {
        let message = "F9BEB4D976657261636B000000000000000000005DF6E0E2";
        let bytes = hex_string_to_bytes(message).unwrap();

        let (mut tx, rx) = UnixStream::pair().unwrap();
        tx.write_all(&bytes).await;
        let (mut rx_half, _) = rx.into_split();

        let network_message = NodeListener::receive_message(&mut rx_half, NetworkMagic::Mainnet).unwrap();

        assert_eq!(network_message.len_serialized, bytes.len());
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

        assert_eq!(version_received.len_serialized, received.len());
        assert!(matches!(message, Commands::Version { .. }));
    }
}
*/
