use std::{
    fmt::{Display, Formatter, Result},
    net::SocketAddr,
    thread,
    time::Duration,
};

use tokio::{
    io::AsyncWriteExt,
    net::{tcp::OwnedWriteHalf, TcpStream},
    sync::mpsc::Receiver,
};

use brl::{
    flags::network_magic::NetworkMagic,
    network::{command::Commands, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};

use crate::{
    handshake_state::HandshakeState,
    message::{pong, verack, version},
    node_listener::NodeListener,
    node_message::NodeMessage,
};

#[derive(Debug)]
struct RemoteNode<'a> {
    node_id: u8,
    writer: &'a mut OwnedWriteHalf,
    receiver: &'a mut Receiver<NetworkMessage>,
    network: NetworkMagic,
    agent: String,
    version: u32,
    feerate: u64,
}

impl Display for RemoteNode<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{{ node_id: {}, agent: {}, version: {}, feerate: {} }}",
            self.node_id, self.agent, self.version, self.feerate
        )
    }
}

impl<'a> RemoteNode<'a> {
    fn new(
        node_id: u8,
        writer: &'a mut OwnedWriteHalf,
        receiver: &'a mut Receiver<NetworkMessage>,
        network: NetworkMagic,
    ) -> Self {
        RemoteNode {
            node_id,
            writer,
            receiver,
            network,
            agent: "unknown".to_string(),
            version: 0,
            feerate: 0,
        }
    }

    pub async fn handshake(&mut self, address: SocketAddr) -> StdResult<()> {
        log::info!("Handshaking...");
        let mut status = HandshakeState::Connected;

        loop {
            match status {
                HandshakeState::Connected => {
                    let version_message = version_message(address, self.network)?;
                    send_message(self.writer, &version_message).await?;
                    status = HandshakeState::LocalVersionSent;
                }
                HandshakeState::LocalVersionSent => {
                    let command = receive_from_remote(self.receiver).await;

                    if let Commands::Version(version) = command {
                        status = HandshakeState::RemoteVersionReceived;

                        log::debug!(
                            "Remote version received (version: {}; height: {}; user_agent: {})",
                            version.version,
                            version.height,
                            version.user_agent
                        );

                        self.agent = version.user_agent.into();
                        self.version = version.version;
                    }
                }
                HandshakeState::RemoteVersionReceived => {
                    let verack_message = verack::new(self.network)?;
                    send_message(self.writer, &verack_message).await?;
                    status = HandshakeState::LocalVerackSent;
                }
                HandshakeState::LocalVerackSent => {
                    let command = receive_from_remote(self.receiver).await;

                    if let Commands::VerAck = command {
                        status = HandshakeState::RemoteVerackReceived;
                        log::debug!("Remote verack received.");
                    }
                }
                HandshakeState::RemoteVerackReceived => {
                    status = HandshakeState::HandshakeCompleted;
                }
                HandshakeState::HandshakeCompleted => {
                    log::info!("Handshake completed.");
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn main_loop(
        &mut self,
        node_to_rest_sender: tokio::sync::broadcast::Sender<NodeMessage>,
        rest_to_node_receiver: &mut tokio::sync::broadcast::Receiver<NodeMessage>,
    ) -> StdResult<()> {
        loop {
            let command = receive(self.receiver, rest_to_node_receiver).await;

            match command {
                Commands::SendCompact(payload) => {
                    log::debug!("SendCompact command received ({:?}).", payload);
                }
                Commands::Ping(payload) => {
                    log::debug!("Ping command received (nonce: {})", payload.nonce);

                    let pong_message = pong::new(payload.nonce, self.network)?;
                    send_message(self.writer, &pong_message).await?;
                }
                Commands::FeeFilter(payload) => {
                    log::debug!("FeeFilter command received ({:?}).", payload);

                    self.feerate = payload.feerate;

                    log::info!("Remote node is ready: {}", self);
                    node_to_rest_sender.send(NodeMessage::NodeReady)?;
                }
                Commands::GetHeaders => {
                    log::debug!("GetHeaders should send to remote node.");
                }
                _ => continue,
            }
        }
    }
}

pub async fn connect(
    node_id: u8,
    remote_address: String,
    network: NetworkMagic,
    node_to_rest_sender: tokio::sync::broadcast::Sender<NodeMessage>,
    rest_to_node_receiver: &mut tokio::sync::broadcast::Receiver<NodeMessage>,
) -> StdResult<()> {
    log::info!("Connecting to {} using {:?} network...", remote_address, network);
    let stream = TcpStream::connect(remote_address).await?;
    log::info!("Connected.");

    let local_address = stream.local_addr()?;

    let (reader, mut writer) = stream.into_split();
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<NetworkMessage>(32);

    let listener_handle = tokio::spawn(async move {
        let mut connection = NodeListener::new(reader, network);
        let _ = connection.listen(sender).await;
    });

    let mut remote_node = RemoteNode::new(node_id, &mut writer, &mut receiver, network);

    remote_node.handshake(local_address).await?;

    remote_node
        .main_loop(node_to_rest_sender, rest_to_node_receiver)
        .await?;

    let res = listener_handle.await;
    if let Err(e) = res {
        log::error!("Connection error: {:}", e);
    }

    Ok(())
}

pub fn version_message(addr: SocketAddr, network: NetworkMagic) -> StdResult<NetworkMessage> {
    let local_address = addr.ip().to_string();
    log::debug!("Local address is {}", local_address);

    version::new(&local_address, network)
}

async fn send_message(writer: &mut OwnedWriteHalf, message: &NetworkMessage) -> StdResult<()> {
    let serialized_message = message.serialize();

    match writer.write_all(&serialized_message).await {
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

static DELAY: Duration = Duration::from_millis(1000);

async fn receive_from_remote(receiver: &mut Receiver<NetworkMessage>) -> Commands {
    loop {
        let received = receiver.recv().await;

        if let Some(message) = received {
            return message.into();
        }

        thread::sleep(DELAY);
    }
}

async fn receive(
    receiver: &mut Receiver<NetworkMessage>,
    rest_to_node_receiver: &mut tokio::sync::broadcast::Receiver<NodeMessage>,
) -> Commands {
    loop {
        tokio::select! {
            received = receiver.recv() => {
                if let Some(message) = received {
                    return message.into();
                }
            }
            received = rest_to_node_receiver.recv() => {
                match received {
                    Ok(NodeMessage::GetHeadersRequest) => {
                        log::debug!("Received GetHeadersRequest from internal.");
                        return Commands::GetHeaders;
                    }
                    Ok(val) => {
                        log::debug!("Received unknown value from rest_to_node_receiver: {:?}", val);
                        continue;
                    }
                    Err(err) => {
                        log::error!("Error receiving value from rest_to_node_receiver: {:?}", err);
                        continue;
                    }
                }
            }
        };

        thread::sleep(DELAY);
    }
}

// TODO: ADDING TESTS
/*
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
*/
