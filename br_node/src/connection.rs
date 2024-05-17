use brl::{
    flags::network_magic::NetworkMagic, network::network_message::NetworkMessage, std_lib::std_result::StdResult,
};
use tokio::{net::tcp::OwnedReadHalf, sync::mpsc::Sender};

#[derive(Debug)]
pub struct Connection {
    reader: OwnedReadHalf,
    network: NetworkMagic,
}

impl Connection {
    pub fn new(reader: OwnedReadHalf, network: NetworkMagic) -> Connection {
        Connection { reader, network }
    }

    pub async fn listen(&mut self, sender: Sender<NetworkMessage>) -> StdResult<()> {
        log::info!("Starting to listen...");

        loop {
            let mut buffer = vec![0; 1024];
            self.reader.readable().await?;

            match self.reader.try_read(&mut buffer) {
                Ok(mut bytes_read) => {
                    if bytes_read == 0 {
                        log::warn!("connection_closed_by_peer");
                        return Err("connection_closed_by_peer".into());
                    }

                    // Align the buffer to the actual bytes read
                    buffer.truncate(bytes_read);

                    // Buffer can contain multiple messages: consume the buffer
                    // until it's empty, sending each message
                    while bytes_read > 0 {
                        let message_received = NetworkMessage::deserialize(&buffer, self.network)?;
                        let consumed = message_received.len_serialized;

                        sender.send(message_received).await?;

                        let _ = buffer.drain(..consumed);
                        bytes_read -= consumed;
                    }
                }

                Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => {
                    log::warn!("connection_reset_by_peer");
                    return Err("connection_reset_by_peer".into());
                }

                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    log::debug!("connection_would_block");
                }

                Err(err) => {
                    return Err(err.into());
                }
            }
        }
    }
}
