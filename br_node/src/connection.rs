use std::{io, net::SocketAddr};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use brl::{
    flags::network_magic::NetworkMagic,
    network::{command::Commands, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};

use crate::message::{verack, version};

#[derive(Debug, PartialEq)]
enum HandshakeState {
    Connected,
    LocalVersionSent,
    RemoteVersionReceived,
    LocalVerackSent,
    RemoteVerackReceived,
    HandshakeCompleted,
}

pub trait NodeStream {
    fn local_addr(&self) -> io::Result<SocketAddr>;
    async fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
    async fn readable(&self) -> io::Result<()>;
    async fn shutdown(&mut self) -> io::Result<()>;
    fn try_read(&self, buf: &mut [u8]) -> io::Result<usize>;
}

impl NodeStream for TcpStream {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.local_addr()
    }

    async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        AsyncWriteExt::write_all(self, buf).await
    }

    async fn readable(&self) -> io::Result<()> {
        self.readable().await
    }

    async fn shutdown(&mut self) -> io::Result<()> {
        AsyncWriteExt::shutdown(self).await
    }

    fn try_read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.try_read(buf)
    }
}

#[derive(Debug)]
pub struct Connection<C: NodeStream> {
    stream: C,
    network: NetworkMagic,
    status: HandshakeState,
}

impl<C: NodeStream> Connection<C> {
    pub async fn new(stream: C, network: NetworkMagic) -> StdResult<Self> {
        let status = HandshakeState::Connected;

        Ok(Self {
            stream,
            network,
            status,
        })
    }

    async fn send_message(&mut self, message: &NetworkMessage) -> StdResult<()> {
        let serialized_message = message.serialize();

        match self.stream.write_all(&serialized_message).await {
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
                    if bytes_read == 0 {
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
                    let local_address = self.stream.local_addr()?;
                    let version_message = version_message(local_address, self.network)?;

                    Self::send_message(self, &version_message).await?;

                    self.status = HandshakeState::LocalVersionSent;
                }
                HandshakeState::LocalVersionSent => {
                    let received = self.wait_for_message().await?;
                    let command: Commands = received.into();

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
                    break;
                }
                HandshakeState::RemoteVersionReceived => {
                    let verack_message = verack_message(self.network)?;
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
                    break;
                }
                HandshakeState::RemoteVerackReceived => {
                    self.status = HandshakeState::HandshakeCompleted;
                }
                HandshakeState::HandshakeCompleted => {
                    log::info!("Handshake completed.");
                    break;
                }
            }

            log::debug!("Last handshake state: {:?}", self.status);
        }

        Ok(())
    }

    pub async fn shutdown(&mut self) -> StdResult<()> {
        self.stream.shutdown().await?;

        Ok(())
    }

    pub async fn listen(&mut self) -> StdResult<()> {
        loop {
            self.stream.readable().await?;

            let received = self.wait_for_message().await?;
            let command: Commands = received.into();

            log::debug!("Received command: {:?}", command);
        }
    }
}

pub fn version_message(addr: SocketAddr, network: NetworkMagic) -> StdResult<NetworkMessage> {
    let local_address = addr.ip().to_string();
    log::debug!("Local address is {}", local_address);

    version::new(&local_address, network)
}

pub fn verack_message(network: NetworkMagic) -> StdResult<NetworkMessage> {
    verack::new(network)
}

#[cfg(test)]
mod connection_tests {
    use std::net::{IpAddr, Ipv4Addr};

    use mockall::{mock, predicate};

    use super::*;

    mock! {
        pub MyNodeStream {}

        impl NodeStream for MyNodeStream {
            fn local_addr(&self) -> io::Result<SocketAddr>;
            async fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
            async fn readable(&self) -> io::Result<()>;
            async fn shutdown(&mut self) -> io::Result<()>;
            fn try_read(&self, buf: &mut [u8]) -> io::Result<usize>;
        }
    }

    const NETWORK: brl::flags::network_magic::NetworkMagic = NetworkMagic::Testnet3;

    #[tokio::test]
    async fn handshake_happy_path() {
        let mut mocked_stream = MockMyNodeStream::new();

        // Arrange - simulating a stream

        // The stream will return the local address
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        mocked_stream.expect_local_addr().returning(move || Ok(addr));

        // Simulate the stream writing the version message and the verack message to remote node
        let version_message = version_message(addr, NETWORK).unwrap();
        let serialized_version_message = version_message.serialize();

        let verack_message = verack_message(NETWORK).unwrap();
        let serialized_verack_message = verack_message.serialize();

        mocked_stream
            .expect_write_all()
            .with(predicate::eq(serialized_version_message))
            .returning(|_| Ok(()));

        mocked_stream
            .expect_write_all()
            .with(predicate::eq(serialized_verack_message))
            .returning(|_| Ok(()));

        // Simulated stream is always readable
        mocked_stream.expect_readable().returning(|| Ok(()));

        // Simulate the stream reading from remote node
        // The remote node will respond with a version message the first time, and a verack message the second time
        let mut write_all_sequence: u8 = 0;

        mocked_stream
            .expect_try_read()
            .returning(move |buff| match write_all_sequence {
                0 => {
                    // First time reading from stream, remote node responds with a version message
                    let version_response = [
                        11, 17, 9, 7, 118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, 102, 0, 0, 0, 18, 22, 47, 73,
                        128, 17, 1, 0, 9, 4, 0, 0, 0, 0, 0, 0, 221, 248, 47, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 121, 26, 201, 31, 18, 68, 169, 16, 47, 83, 97, 116,
                        111, 115, 104, 105, 58, 50, 54, 46, 48, 46, 48, 47, 47, 101, 39, 0, 1,
                    ];
                    let version_response_len = version_response.len();

                    buff[0..version_response_len].copy_from_slice(&version_response);
                    write_all_sequence += 1;
                    Ok(version_response_len)
                }
                1 => {
                    // Second time reading from stream, remote node responds with a verack message
                    let verack_response = [
                        11, 17, 9, 7, 118, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 246, 224, 226,
                    ];
                    let verack_response_len = verack_response.len();

                    buff[0..verack_response_len].copy_from_slice(&verack_response);
                    write_all_sequence += 1;
                    Ok(verack_response_len)
                }
                _ => panic!("Unexpected call to try_read"),
            });

        // Act
        let mut connection = Connection::new(mocked_stream, NETWORK).await.unwrap();
        let result = connection.try_handshake().await;

        // Assert - handshake completed successfully
        assert!(result.is_ok());
        assert_eq!(connection.status, HandshakeState::HandshakeCompleted);
    }

    #[tokio::test]
    async fn handshake_stream_would_block() {
        let mut mocked_stream = MockMyNodeStream::new();

        // Arrange

        // The stream will return the local address
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        mocked_stream.expect_local_addr().returning(move || Ok(addr));

        // Simulate the stream writing the version message and the verack message to remote node
        let version_message = version_message(addr, NETWORK).unwrap();
        let serialized_version_message = version_message.serialize();

        let verack_message = verack_message(NETWORK).unwrap();
        let serialized_verack_message = verack_message.serialize();

        mocked_stream
            .expect_write_all()
            .with(predicate::eq(serialized_version_message))
            .returning(|_| Ok(()));

        mocked_stream
            .expect_write_all()
            .with(predicate::eq(serialized_verack_message))
            .returning(|_| Ok(()));

        // Simulated stream is always readable
        mocked_stream.expect_readable().returning(|| Ok(()));

        // Simulate the stream reading from remote node
        // The remote node will respond with a version message the first time, and a verack message the second time
        let mut write_all_sequence: u8 = 0;

        mocked_stream
            .expect_try_read()
            .returning(move |buff| match write_all_sequence {
                0 => {
                    // First time reading from stream, remote node responds with a version message
                    let version_response = [
                        11, 17, 9, 7, 118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, 102, 0, 0, 0, 18, 22, 47, 73,
                        128, 17, 1, 0, 9, 4, 0, 0, 0, 0, 0, 0, 221, 248, 47, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 121, 26, 201, 31, 18, 68, 169, 16, 47, 83, 97, 116,
                        111, 115, 104, 105, 58, 50, 54, 46, 48, 46, 48, 47, 47, 101, 39, 0, 1,
                    ];
                    let version_response_len = version_response.len();

                    buff[0..version_response_len].copy_from_slice(&version_response);
                    write_all_sequence += 1;
                    Ok(version_response_len)
                }
                1 => {
                    // Second time simulate a "would block" error
                    write_all_sequence += 1;
                    Err(std::io::ErrorKind::WouldBlock.into())
                }
                2 => {
                    // Third time reading from stream, remote node responds with a verack message
                    let verack_response = [
                        11, 17, 9, 7, 118, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 246, 224, 226,
                    ];
                    let verack_response_len = verack_response.len();

                    buff[0..verack_response_len].copy_from_slice(&verack_response);
                    write_all_sequence += 1;
                    Ok(verack_response_len)
                }
                _ => panic!("Unexpected call to try_read"),
            });

        // Act
        let mut connection = Connection::new(mocked_stream, NETWORK).await.unwrap();
        let result = connection.try_handshake().await;

        // Assert - handshake completed successfully
        assert!(result.is_ok());
        assert_eq!(connection.status, HandshakeState::HandshakeCompleted);
    }

    #[tokio::test]
    async fn handshake_connection_reset() {
        let mut mocked_stream = MockMyNodeStream::new();

        // Arrange

        // The stream will return the local address
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        mocked_stream.expect_local_addr().returning(move || Ok(addr));

        // Simulate the stream writing the version message and the verack message to remote node
        let version_message = version_message(addr, NETWORK).unwrap();
        let serialized_version_message = version_message.serialize();

        let verack_message = verack_message(NETWORK).unwrap();
        let serialized_verack_message = verack_message.serialize();

        mocked_stream
            .expect_write_all()
            .with(predicate::eq(serialized_version_message))
            .returning(|_| Ok(()));

        mocked_stream
            .expect_write_all()
            .with(predicate::eq(serialized_verack_message))
            .returning(|_| Ok(()));

        // Simulated stream is always readable
        mocked_stream.expect_readable().returning(|| Ok(()));

        // Simulate the stream error from remote node
        mocked_stream
            .expect_try_read()
            .returning(move |_buff| Err(std::io::ErrorKind::ConnectionReset.into()));

        // Act
        let mut connection = Connection::new(mocked_stream, NETWORK).await.unwrap();
        let result = connection.try_handshake().await;

        // Assert - handshake not completed
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "connection_reset_by_peer");
    }

    #[tokio::test]
    async fn handshake_reset_by_peer() {
        let mut mocked_stream = MockMyNodeStream::new();

        // Arrange

        // The stream will return the local address
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        mocked_stream.expect_local_addr().returning(move || Ok(addr));

        // Simulate the stream writing the version message and the verack message to remote node
        let version_message = version_message(addr, NETWORK).unwrap();
        let serialized_version_message = version_message.serialize();

        let verack_message = verack_message(NETWORK).unwrap();
        let serialized_verack_message = verack_message.serialize();

        mocked_stream
            .expect_write_all()
            .with(predicate::eq(serialized_version_message))
            .returning(|_| Ok(()));

        mocked_stream
            .expect_write_all()
            .with(predicate::eq(serialized_verack_message))
            .returning(|_| Ok(()));

        // Simulated stream is always readable
        mocked_stream.expect_readable().returning(|| Ok(()));

        // Simulate the stream error from remote node
        mocked_stream.expect_try_read().returning(move |_buff| Ok(0));

        // Act
        let mut connection = Connection::new(mocked_stream, NETWORK).await.unwrap();
        let result = connection.try_handshake().await;

        // Assert - handshake not completed
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "connection_closed_by_peer");
    }
}
