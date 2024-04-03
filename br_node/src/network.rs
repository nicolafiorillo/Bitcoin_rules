use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc,
};

use brl::{
    flags::network_magic::NetworkMagic,
    network::{command::Commands, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};

use crate::message::version_message;

pub async fn connect(address: &str, network: NetworkMagic, sender: mpsc::Sender<u16>) -> StdResult<()> {
    log::info!("Connecting to {} using {:?} network...", address, network);
    let mut stream = TcpStream::connect(address).await?;
    log::info!("Connected.");

    let local_address = stream.local_addr().unwrap().ip().to_string();
    log::info!("Local address is {}", local_address);

    let version_message = version_message::new(&local_address, network);

    log::debug!("Sending version message...");

    match stream.write_all(&version_message.serialize()).await {
        Ok(_) => {
            log::debug!("Message sent");
        }
        Err(err) => {
            log::error!("Error sending message: {}", err);
            return Err(err.into());
        }
    }

    stream.readable().await?;

    let mut buffer = vec![0; 1024];

    // read server answer for the whole data
    match stream.try_read(&mut buffer) {
        Ok(bytes_read) => {
            if buffer.is_empty() {
                log::warn!("connection_closed_by_peer");
                sender.send(1).await?;
                return Ok(());
            }

            buffer.truncate(bytes_read);
            let received = NetworkMessage::deserialize(&buffer, network)?;
            //            let message: Commands = received.clone().into();

            log::info!("Peer response: {:}", received.command);
        }

        Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => {
            log::warn!("connection_reset_by_peer");
            sender.send(2).await?;
            return Ok(());
        }

        Err(err) => return Err(err.into()),
    }

    Ok(())
}

#[cfg(test)]
mod network_test {
    use brl::{
        flags::network_magic::NetworkMagic,
        network::{command::Commands, network_message::NetworkMessage},
    };

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

        assert_eq!(message, Commands::VerAck);
    }
}
