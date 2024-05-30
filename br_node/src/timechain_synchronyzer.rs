use brl::{hashing::hash256::Hash256, std_lib::std_result::StdResult};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::node_message::NodeMessage;

pub async fn start(
    start_block_hash: Hash256,
    sender: Sender<NodeMessage>,
    mut receiver: Receiver<NodeMessage>,
) -> StdResult<()> {
    loop {
        // get last block in persistence and use its hash as starting block
        // for now use genesis_block_hash as starting block
        let msg = receiver.recv().await?;
        match msg {
            NodeMessage::NodeReady(node_id) => {
                log::debug!("Node {} is ready", node_id);
                let _ = sender.send(NodeMessage::GetHeadersRequest(start_block_hash));
            }
            _ => {
                log::info!("Received message: {:?}", msg);
                log::debug!("timechain_synchronyzer exiting...");

                break;
            }
        }
    }

    Ok(())
}
