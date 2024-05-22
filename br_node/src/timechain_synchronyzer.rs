use brl::{hashing::hash256::Hash256, std_lib::std_result::StdResult};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::node_message::NodeMessage;

pub async fn start(
    _genesis_block_hash: Hash256,
    sender: Sender<NodeMessage>,
    mut receiver: Receiver<NodeMessage>,
) -> StdResult<()> {
    loop {
        let msg = receiver.recv().await;
        if let Ok(msg) = msg {
            log::info!("Received message: {:?}", msg);
            let _ = sender.send(NodeMessage::GetHeadersRequest); //  using genesis_block_hash

            break;
        }
    }

    Ok(())
}
