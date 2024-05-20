/*!
Entry point
*/

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

use brl::flags::network_magic::NetworkMagic;

mod config;
use config::{load_config, Configuration};

use crate::node_message::NodeMessage;

mod handshake_state;
mod message;
mod node_listener;
mod node_message;
mod remote_node;
mod remote_nodes_orchestrator;
mod timechain_synchronyzer;
mod utils;

// TODO: move network stuff in a dedicated lib (br_net)

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Application started.");

    let cfg: Configuration = load_config().unwrap();
    log::info!("Configuration: {:?}", cfg);

    let network: NetworkMagic = cfg.network.into();

    let address = format!("{}:{}", cfg.remote_node_address, cfg.remote_node_port);

    emit_logo();

    log::info!("Bitcoin_rules! node (ver. {}).", utils::version());
    log::info!("This is a work in progress: please do not use in production.");

    log::info!("");
    log::info!("Network: {}", network);

    let (node_to_rest_sender, _node_to_rest_receiver) = tokio::sync::broadcast::channel::<NodeMessage>(16);
    let (rest_to_node_sender, _rest_to_node_receiver) = tokio::sync::broadcast::channel::<NodeMessage>(16);

    let rest_to_node_sx = rest_to_node_sender.clone();
    let node_to_rest_rx = node_to_rest_sender.subscribe();

    let timechain_synchronyzer_handle = tokio::spawn(async move {
        let _ = timechain_synchronyzer::start(rest_to_node_sx, node_to_rest_rx).await;
    });

    let rest_to_node_rx = rest_to_node_sender.subscribe();

    let remote_nodes_orchestrator_handle = tokio::spawn(async move {
        let _ = remote_nodes_orchestrator::start(address, network, node_to_rest_sender, rest_to_node_rx).await;
    });

    let _ = timechain_synchronyzer_handle.await;
    log::debug!("timechain_synchronyzer thread exit.");

    let _ = remote_nodes_orchestrator_handle.await;
    log::debug!("remote_nodes_orchestrator thread exit.");

    log::info!("Application stopped.");
}

fn emit_logo() {
    log::info!("");
    log::info!("______ _ _            _                             _           _ ");
    log::info!("| ___ (_) |          (_)                           | |         | |");
    log::info!("| |_/ /_| |_ ___ ___  _ _ __             _ __ _   _| | ___  ___| |");
    log::info!("| ___ \\ | __/ __/ _ \\| | '_ \\           | '__| | | | |/ _ \\/ __| |");
    log::info!("| |_/ / | || (_| (_) | | | | |  ______  | |  | |_| | |  __/\\__ \\_|");
    log::info!("\\____/|_|\\__\\___\\___/|_|_| |_| |______| |_|   \\__,_|_|\\___||___(_)");
    log::info!("");
}
