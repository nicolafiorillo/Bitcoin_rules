use brl::{flags::network_magic::NetworkMagic, std_lib::std_result::StdResult};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::{node_message::NodeMessage, remote_node};

pub async fn start(
    address: String,
    network: NetworkMagic,
    node_to_rest_sender: Sender<NodeMessage>,
    mut rest_to_node_receiver: Receiver<NodeMessage>,
) -> StdResult<()> {
    let node_id: u8 = 0; // will be mut when connecting to multiple nodes

    let first_remote_node_handle = tokio::spawn(async move {
        let res = remote_node::connect(
            node_id,
            address,
            network,
            node_to_rest_sender,
            &mut rest_to_node_receiver,
        )
        .await;

        if let Err(e) = res {
            log::error!("Error managing to remote node: {:?}", e);
        }
    });

    let _res = first_remote_node_handle.await;

    // If first remote node fails, here we will try to connect to another one.
    // Exit for now, until peer discovery will be implemented.
    log::debug!("remote_nodes_orchestrator exiting...");

    Ok(())
}
