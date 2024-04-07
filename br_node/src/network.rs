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
