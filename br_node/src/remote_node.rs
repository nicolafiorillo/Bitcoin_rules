use std::{net::SocketAddr, thread, time::Duration};

use crate::message::{pong, verack, version};
use brl::{
    flags::network_magic::NetworkMagic,
    network::{command::Commands, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};
use tokio::{
    io::AsyncWriteExt,
    net::{tcp::OwnedWriteHalf, TcpStream},
    sync::mpsc::Receiver,
};

use crate::node_listener::NodeListener;

/*
   TODO: refactor the entire module
*/

#[derive(Debug)]
struct RemoteNode {
    agent: String,
    version: u32,
    feerate: u64,
}

#[derive(Debug, PartialEq)]
enum HandshakeState {
    Connected,
    LocalVersionSent,
    RemoteVersionReceived,
    LocalVerackSent,
    RemoteVerackReceived,
    HandshakeCompleted,
}

impl RemoteNode {
    fn new() -> Self {
        RemoteNode {
            agent: "unknown".to_string(),
            version: 0,
            feerate: 0,
        }
    }
}

pub async fn connect(address: &str, network: NetworkMagic) -> StdResult<()> {
    log::info!("Connecting to remote node: {}", address);

    let stream = open_stream(address, network).await?;
    let local_address = stream.local_addr()?;

    let (reader, mut writer) = stream.into_split();

    let mut connection = NodeListener::new(reader, network);

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<NetworkMessage>(32);

    let listener_handle = tokio::spawn(async move {
        let _ = connection.listen(sender).await;
    });

    let mut remote_node = handshake(&mut writer, &mut receiver, local_address, network).await?;

    main_loop(&mut remote_node, &mut writer, &mut receiver, network).await?;

    let res = listener_handle.await;
    if let Err(e) = res {
        log::error!("Connection error: {:}", e);
    }

    Ok(())
}

async fn open_stream(address: &str, network: NetworkMagic) -> StdResult<TcpStream> {
    log::info!("Connecting to {} using {:?} network...", address, network);
    let stream = TcpStream::connect(address).await?;
    log::info!("Connected.");

    Ok(stream)
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

async fn receive_message(receiver: &mut Receiver<NetworkMessage>) -> Commands {
    let received = receiver.recv().await;

    loop {
        if let Some(message) = received {
            return message.into();
        }

        thread::sleep(DELAY);
    }
}

async fn main_loop(
    remote_node: &mut RemoteNode,
    writer: &mut OwnedWriteHalf,
    receiver: &mut Receiver<NetworkMessage>,
    network: NetworkMagic,
) -> StdResult<()> {
    log::info!("Main loop...");

    loop {
        let command = receive_message(receiver).await;

        match command {
            Commands::SendCompact(payload) => {
                log::info!("SendCompact command received.");
                log::info!("payload: {:?}", payload);
            }
            Commands::Ping(payload) => {
                log::debug!("Ping command received (nonce: {})", payload.nonce);

                let pong_message = pong::new(payload.nonce, network)?;
                send_message(writer, &pong_message).await?;
            }
            Commands::FeeFilter(payload) => {
                log::info!("FeeFilter command received.");
                log::info!("payload: {:?}", payload);

                remote_node.feerate = payload.feerate;

                log::info!("remote_node: {:?}", remote_node);
            }
            _ => continue,
        }
    }
}

async fn handshake(
    writer: &mut OwnedWriteHalf,
    receiver: &mut Receiver<NetworkMessage>,
    address: SocketAddr,
    network: NetworkMagic,
) -> StdResult<RemoteNode> {
    log::info!("Handshaking...");
    let mut status = HandshakeState::Connected;

    let mut remote_node = RemoteNode::new();

    loop {
        match status {
            HandshakeState::Connected => {
                let version_message = version_message(address, network)?;
                send_message(writer, &version_message).await?;
                status = HandshakeState::LocalVersionSent;
            }
            HandshakeState::LocalVersionSent => {
                let command = receive_message(receiver).await;

                if let Commands::Version(version) = command {
                    status = HandshakeState::RemoteVersionReceived;

                    log::info!(
                        "Remote version received (version: {}; height: {}; user_agent: {})",
                        version.version,
                        version.height,
                        version.user_agent
                    );

                    remote_node.agent = version.user_agent.into();
                    remote_node.version = version.version;
                }
            }
            HandshakeState::RemoteVersionReceived => {
                let verack_message = verack::new(network)?;
                send_message(writer, &verack_message).await?;
                status = HandshakeState::LocalVerackSent;
            }
            HandshakeState::LocalVerackSent => {
                let command = receive_message(receiver).await;

                if let Commands::VerAck = command {
                    status = HandshakeState::RemoteVerackReceived;
                    log::info!("Remote verack received.");
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

    Ok(remote_node)
}

// ADDING TESTS

// add also test on multiple incoming messages in the buffer

// #[cfg(test)]
// mod connection_tests {
//     use std::net::{IpAddr, Ipv4Addr};

//     use mockall::{mock, predicate};

//     use super::*;

//     mock! {
//         pub MyNodeStream {}

//         impl NodeStream for MyNodeStream {
//             fn local_addr(&self) -> io::Result<SocketAddr>;
//             async fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
//             async fn readable(&self) -> io::Result<()>;
//             async fn shutdown(&mut self) -> io::Result<()>;
//             fn try_read(&self, buf: &mut [u8]) -> io::Result<usize>;
//         }
//     }

//     const NETWORK: brl::flags::network_magic::NetworkMagic = NetworkMagic::Testnet3;

//     #[tokio::test]
//     async fn handshake_happy_path() {
//         let mut mocked_stream = MockMyNodeStream::new();

//         // Arrange - simulating a stream

//         // The stream will return the local address
//         let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
//         mocked_stream.expect_local_addr().returning(move || Ok(addr));

//         // Simulate the stream writing the version message and the verack message to remote node
//         let version_message = version_message(addr, NETWORK).unwrap();
//         let serialized_version_message = version_message.serialize();

//         let verack_message = verack_message(NETWORK).unwrap();
//         let serialized_verack_message = verack_message.serialize();

//         mocked_stream
//             .expect_write_all()
//             .with(predicate::eq(serialized_version_message))
//             .returning(|_| Ok(()));

//         mocked_stream
//             .expect_write_all()
//             .with(predicate::eq(serialized_verack_message))
//             .returning(|_| Ok(()));

//         // Simulated stream is always readable
//         mocked_stream.expect_readable().returning(|| Ok(()));

//         // Simulate the stream reading from remote node
//         // The remote node will respond with a version message the first time, and a verack message the second time
//         let mut write_all_sequence: u8 = 0;

//         mocked_stream
//             .expect_try_read()
//             .returning(move |buff| match write_all_sequence {
//                 0 => {
//                     // First time reading from stream, remote node responds with a version message
//                     let version_response = [
//                         11, 17, 9, 7, 118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, 102, 0, 0, 0, 18, 22, 47, 73,
//                         128, 17, 1, 0, 9, 4, 0, 0, 0, 0, 0, 0, 221, 248, 47, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 121, 26, 201, 31, 18, 68, 169, 16, 47, 83, 97, 116,
//                         111, 115, 104, 105, 58, 50, 54, 46, 48, 46, 48, 47, 47, 101, 39, 0, 1,
//                     ];
//                     let version_response_len = version_response.len();

//                     buff[0..version_response_len].copy_from_slice(&version_response);
//                     write_all_sequence += 1;
//                     Ok(version_response_len)
//                 }
//                 1 => {
//                     // Second time reading from stream, remote node responds with a verack message
//                     let verack_response = [
//                         11, 17, 9, 7, 118, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 246, 224, 226,
//                     ];
//                     let verack_response_len = verack_response.len();

//                     buff[0..verack_response_len].copy_from_slice(&verack_response);
//                     write_all_sequence += 1;
//                     Ok(verack_response_len)
//                 }
//                 _ => panic!("Unexpected call to try_read"),
//             });

//         // Act
//         let mut connection = Connection::new(mocked_stream, NETWORK).await.unwrap();
//         let result = connection.try_handshake().await;

//         // Assert - handshake completed successfully
//         assert!(result.is_ok());
//         assert_eq!(connection.status, HandshakeState::HandshakeCompleted);
//     }

//     #[tokio::test]
//     async fn handshake_stream_would_block() {
//         let mut mocked_stream = MockMyNodeStream::new();

//         // Arrange

//         // The stream will return the local address
//         let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
//         mocked_stream.expect_local_addr().returning(move || Ok(addr));

//         // Simulate the stream writing the version message and the verack message to remote node
//         let version_message = version_message(addr, NETWORK).unwrap();
//         let serialized_version_message = version_message.serialize();

//         let verack_message = verack_message(NETWORK).unwrap();
//         let serialized_verack_message = verack_message.serialize();

//         mocked_stream
//             .expect_write_all()
//             .with(predicate::eq(serialized_version_message))
//             .returning(|_| Ok(()));

//         mocked_stream
//             .expect_write_all()
//             .with(predicate::eq(serialized_verack_message))
//             .returning(|_| Ok(()));

//         // Simulated stream is always readable
//         mocked_stream.expect_readable().returning(|| Ok(()));

//         // Simulate the stream reading from remote node
//         // The remote node will respond with a version message the first time, and a verack message the second time
//         let mut write_all_sequence: u8 = 0;

//         mocked_stream
//             .expect_try_read()
//             .returning(move |buff| match write_all_sequence {
//                 0 => {
//                     // First time reading from stream, remote node responds with a version message
//                     let version_response = [
//                         11, 17, 9, 7, 118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, 102, 0, 0, 0, 18, 22, 47, 73,
//                         128, 17, 1, 0, 9, 4, 0, 0, 0, 0, 0, 0, 221, 248, 47, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//                         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 121, 26, 201, 31, 18, 68, 169, 16, 47, 83, 97, 116,
//                         111, 115, 104, 105, 58, 50, 54, 46, 48, 46, 48, 47, 47, 101, 39, 0, 1,
//                     ];
//                     let version_response_len = version_response.len();

//                     buff[0..version_response_len].copy_from_slice(&version_response);
//                     write_all_sequence += 1;
//                     Ok(version_response_len)
//                 }
//                 1 => {
//                     // Second time simulate a "would block" error
//                     write_all_sequence += 1;
//                     Err(std::io::ErrorKind::WouldBlock.into())
//                 }
//                 2 => {
//                     // Third time reading from stream, remote node responds with a verack message
//                     let verack_response = [
//                         11, 17, 9, 7, 118, 101, 114, 97, 99, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93, 246, 224, 226,
//                     ];
//                     let verack_response_len = verack_response.len();

//                     buff[0..verack_response_len].copy_from_slice(&verack_response);
//                     write_all_sequence += 1;
//                     Ok(verack_response_len)
//                 }
//                 _ => panic!("Unexpected call to try_read"),
//             });

//         // Act
//         let mut connection = Connection::new(mocked_stream, NETWORK).await.unwrap();
//         let result = connection.try_handshake().await;

//         // Assert - handshake completed successfully
//         assert!(result.is_ok());
//         assert_eq!(connection.status, HandshakeState::HandshakeCompleted);
//     }

//     #[tokio::test]
//     async fn handshake_connection_reset() {
//         let mut mocked_stream = MockMyNodeStream::new();

//         // Arrange

//         // The stream will return the local address
//         let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
//         mocked_stream.expect_local_addr().returning(move || Ok(addr));

//         // Simulate the stream writing the version message and the verack message to remote node
//         let version_message = version_message(addr, NETWORK).unwrap();
//         let serialized_version_message = version_message.serialize();

//         let verack_message = verack_message(NETWORK).unwrap();
//         let serialized_verack_message = verack_message.serialize();

//         mocked_stream
//             .expect_write_all()
//             .with(predicate::eq(serialized_version_message))
//             .returning(|_| Ok(()));

//         mocked_stream
//             .expect_write_all()
//             .with(predicate::eq(serialized_verack_message))
//             .returning(|_| Ok(()));

//         // Simulated stream is always readable
//         mocked_stream.expect_readable().returning(|| Ok(()));

//         // Simulate the stream error from remote node
//         mocked_stream
//             .expect_try_read()
//             .returning(move |_buff| Err(std::io::ErrorKind::ConnectionReset.into()));

//         // Act
//         let mut connection = Connection::new(mocked_stream, NETWORK).await.unwrap();
//         let result = connection.try_handshake().await;

//         // Assert - handshake not completed
//         assert!(result.is_err());
//         assert_eq!(result.unwrap_err().to_string(), "connection_reset_by_peer");
//     }

//     #[tokio::test]
//     async fn handshake_reset_by_peer() {
//         let mut mocked_stream = MockMyNodeStream::new();

//         // Arrange

//         // The stream will return the local address
//         let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
//         mocked_stream.expect_local_addr().returning(move || Ok(addr));

//         // Simulate the stream writing the version message and the verack message to remote node
//         let version_message = version_message(addr, NETWORK).unwrap();
//         let serialized_version_message = version_message.serialize();

//         let verack_message = verack_message(NETWORK).unwrap();
//         let serialized_verack_message = verack_message.serialize();

//         mocked_stream
//             .expect_write_all()
//             .with(predicate::eq(serialized_version_message))
//             .returning(|_| Ok(()));

//         mocked_stream
//             .expect_write_all()
//             .with(predicate::eq(serialized_verack_message))
//             .returning(|_| Ok(()));

//         // Simulated stream is always readable
//         mocked_stream.expect_readable().returning(|| Ok(()));

//         // Simulate the stream error from remote node
//         mocked_stream.expect_try_read().returning(move |_buff| Ok(0));

//         // Act
//         let mut connection = Connection::new(mocked_stream, NETWORK).await.unwrap();
//         let result = connection.try_handshake().await;

//         // Assert - handshake not completed
//         assert!(result.is_err());
//         assert_eq!(result.unwrap_err().to_string(), "connection_closed_by_peer");
//     }
// }
