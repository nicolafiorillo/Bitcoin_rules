use tokio::{io::AsyncWriteExt, net::TcpStream};

use brl::{
    flags::network_magic::NetworkMagic,
    network::{command::Commands, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};

use crate::message::{verack, version};

enum HandshakeState {
    Connected,
    LocalVersionSent,
    RemoteVersionReceived,
    LocalVerackSent,
    RemoteVerackReceived,
    HandshakeCompleted,
}

pub struct ConnectionContext {
    stream: TcpStream,
    network: NetworkMagic,
    status: HandshakeState,
}

impl ConnectionContext {
    pub async fn new(stream: TcpStream, network: NetworkMagic) -> StdResult<Self> {
        let status = HandshakeState::Connected;

        Ok(Self {
            stream,
            network,
            status,
        })
    }

    fn version_message(&self) -> StdResult<NetworkMessage> {
        let local_address = self.stream.local_addr().unwrap().ip().to_string();
        log::debug!("Local address is {}", local_address);

        version::new(&local_address, self.network)
    }

    async fn send_message(&mut self, message: &NetworkMessage) -> StdResult<()> {
        match self.stream.write_all(&message.serialize()).await {
            Ok(_) => {
                log::debug!("Message {} sent", message.command);
                Ok(())
            }
            Err(err) => {
                log::error!("Error sending message: {}", err);
                Err(err.into())
            }
        }
    }

    async fn wait_for_message(&mut self) -> StdResult<NetworkMessage> {
        let mut buffer = vec![0; 1024];

        loop {
            self.stream.readable().await?;

            match self.stream.try_read(&mut buffer) {
                Ok(bytes_read) => {
                    if buffer.is_empty() {
                        log::warn!("connection_closed_by_peer");
                        return Err("connection_closed_by_peer".into());
                    }

                    buffer.truncate(bytes_read);
                    let received = NetworkMessage::deserialize(&buffer, self.network)?;
                    return Ok(received);
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

    pub async fn try_handshake(&mut self) -> StdResult<()> {
        loop {
            match self.status {
                HandshakeState::Connected => {
                    let version_message = Self::version_message(self)?;
                    Self::send_message(self, &version_message).await?;
                    self.status = HandshakeState::LocalVersionSent;
                }
                HandshakeState::LocalVersionSent => {
                    let received = self.wait_for_message().await?;
                    let command: Commands = received.clone().into();

                    if let Commands::Version(version) = command {
                        self.status = HandshakeState::RemoteVersionReceived;
                        log::debug!(
                            "Remote version received: {} with height {}",
                            version.version,
                            version.height
                        );
                        continue;
                    }

                    log::warn!("Unexpected command: {:?}", command);
                }
                HandshakeState::RemoteVersionReceived => {
                    let verack_message = verack::new(self.network)?;
                    Self::send_message(self, &verack_message).await?;

                    self.status = HandshakeState::LocalVerackSent;
                }
                HandshakeState::LocalVerackSent => {
                    let received = self.wait_for_message().await?;
                    let command: Commands = received.clone().into();

                    if let Commands::VerAck = command {
                        self.status = HandshakeState::RemoteVerackReceived;
                        log::debug!("Remote verAck received.");
                        continue;
                    }

                    log::warn!("Unexpected command: {:?}", command);
                }
                HandshakeState::RemoteVerackReceived => {
                    self.status = HandshakeState::HandshakeCompleted;
                }
                HandshakeState::HandshakeCompleted => {
                    log::info!("Handshake completed.");
                    break;
                }
            }
        }

        Ok(())
    }
}
