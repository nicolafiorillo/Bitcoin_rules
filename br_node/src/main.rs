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

mod handshake_state;
mod message;
mod node_listener;
mod remote_node;
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

    let handle = tokio::spawn(async move {
        let _ = remote_node::connect(&address, network).await;
    });

    let res = handle.await;
    if let Err(e) = res {
        log::error!("Connection error: {:}", e);
    }

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
