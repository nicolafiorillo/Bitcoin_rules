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
use tokio::sync::mpsc::{self, Sender};

mod config;
use config::{load_config, Configuration};

mod message;
mod network;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Application started.");

    let cfg: Configuration = load_config().unwrap();
    log::info!("Configuration: {:?}", cfg);

    let network: NetworkMagic = cfg.network.into();

    let address = format!("{}:{}", cfg.remote_node_address, cfg.remote_node_port);

    log::info!("");
    log::info!("______ _ _            _                    _           _ ");
    log::info!("| ___ (_) |          (_)                  | |         | |");
    log::info!("| |_/ /_| |_ ___ ___  _ _ __    _ __ _   _| | ___  ___| |");
    log::info!("| ___ \\ | __/ __/ _ \\| | '_ \\  | '__| | | | |/ _ \\/ __| |");
    log::info!("| |_/ / | || (_| (_) | | | | | | |  | |_| | |  __/\\__ \\_|");
    log::info!("\\____/|_|\\__\\___\\___/|_|_| |_| |_|   \\__,_|_|\\___||___(_)");
    log::info!("                 ______                        ");
    log::info!("                |______|                       ");
    log::info!("");
    log::info!("Bitcoin_rules! node (ver. {}).", utils::version());
    log::info!("This is a work in progress: please do not use in production.");

    log::info!("Network: {}", network);
    log::info!("Remote node: {}", address);

    let (sender, mut receiver) = mpsc::channel::<u16>(10);

    let handle = tokio::spawn(async move {
        if let Err(e) = network::connect(&address, network, sender).await {
            log::error!("Error connecting to {}: {:}", address, e);
        }
    });

    let _res = handle.await;

    log::info!("Application stopped.");
}
