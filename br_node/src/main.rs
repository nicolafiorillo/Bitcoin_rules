/*!
Entry point
*/

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

mod config;
use config::{load_config, Configuration};

fn main() {
    env_logger::init();

    log::info!("Application started.");

    let cfg: Configuration = load_config().unwrap();
    log::info!("Configuration: {:?}", cfg);

    log::info!("Bitcoin_rules! node (ver. {}).", version());
    log::info!("This is a work in progress: please do not use in production.");

    log::info!("Application stopped.");
}

fn version() -> &'static str {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("unknown")
}
