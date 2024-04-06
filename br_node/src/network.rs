use brl::{flags::network_magic::NetworkMagic, std_lib::std_result::StdResult};

use crate::connection_context::ConnectionContext;

pub async fn connect_to_node(address: &str, network: NetworkMagic) -> StdResult<()> {
    log::info!("Connecting to remote node: {}", address);

    let mut context = ConnectionContext::new(address, network).await?;

    match context.try_handshake().await {
        Ok(_) => log::info!("Connection established."),
        Err(err) => log::error!("Error: {}", err),
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
