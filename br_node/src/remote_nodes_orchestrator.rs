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
        let _ = remote_node::connect(
            node_id,
            address,
            network,
            node_to_rest_sender,
            &mut rest_to_node_receiver,
        )
        .await;
    });

    let _res = first_remote_node_handle.await;

    // loop {
    //     let msg = receiver.recv().await;
    //     if let Ok(msg) = msg {
    //         log::info!("Received message: {}", msg);

    //         break;
    //     }
    // }

    Ok(())
}
