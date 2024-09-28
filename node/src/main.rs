/*!
Entry point
*/

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

use core::flags::network_magic::NetworkMagic;

mod environment;
use environment::{load_config, Environment};

use crate::internal_message::InternalMessage;

mod custom_log;
mod database;
mod handshake_state;
mod internal_message;
mod message;
mod node_listener;
mod remote_node;
mod remote_nodes_orchestrator;
mod timechain_synchronyzer;
mod utils;

// TODO: exit gracefully from all threads in case of error

#[tokio::main]
async fn main() {
    custom_log::init();
    dotenvy::dotenv().ok();

    log::info!("Node started.");
    core::cli::logo::emit();

    log::info!("Node started.");

    let env: Environment = load_config().unwrap();
    log::info!("Environment: {:}", env);

    let network: NetworkMagic = env.network;

    let address = format!("{}:{}", env.remote_node_address, env.remote_node_port);

    log::info!("P2P version {}.", utils::version());
    log::info!("");

    // These channels create senders and receivers. They are not use diretcly but passed, after cloning, to
    // the threads that will use them, so that the main thread can keep them alive in its scope.
    let (node_to_rest_sender, _node_to_rest_receiver) = tokio::sync::broadcast::channel::<InternalMessage>(16);
    let (rest_to_node_sender, _rest_to_node_receiver) = tokio::sync::broadcast::channel::<InternalMessage>(16);

    // Run timechain synchronyzer
    let rest_to_node_sx = rest_to_node_sender.clone();
    let node_to_rest_rx = node_to_rest_sender.subscribe();

    let timechain_synchronyzer_handle = tokio::spawn(async move {
        let res = timechain_synchronyzer::start(env.genesis_block_hash, rest_to_node_sx, node_to_rest_rx).await;
        if let Err(e) = res {
            log::error!("Error managing timechain synchronyzer: {:?}", e);
        }
    });

    // Run remote nodes orchestrator
    let node_to_rest_sender = node_to_rest_sender.clone();
    let rest_to_node_receiver = rest_to_node_sender.subscribe();

    let remote_nodes_orchestrator_handle = tokio::spawn(async move {
        let _ = remote_nodes_orchestrator::start(address, network, node_to_rest_sender, rest_to_node_receiver).await;
    });

    let _ = timechain_synchronyzer_handle.await;
    log::debug!("timechain_synchronyzer thread exit.");

    let _ = remote_nodes_orchestrator_handle.await;
    log::debug!("remote_nodes_orchestrator thread exit.");

    log::info!("Application stopped.");
}
