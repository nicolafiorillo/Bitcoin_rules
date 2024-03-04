/*!
Entry point
*/

// #![warn(
//     missing_docs,
//     clippy::missing_docs_in_private_items,
//     clippy::missing_errors_doc,
//     clippy::missing_panics_doc
// )]

use tracing::Level;

static LOG_FILE: &str = "brn.log";
static LOG_DIR: &str = "./log";

fn main() {
    init_log();

    log::info!("Bitcoin Rules! node started.");

    println!("Bitcoin_rules! node (ver. {}).", version());
    println!("This is a work in progress: please do not use in production.");

    log::info!("Bitcoin Rules! stopped.");
}

fn version() -> &'static str {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("unknown")
}

fn init_log() {
    let file_appender = tracing_appender::rolling::hourly(LOG_DIR, LOG_FILE);

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(Level::TRACE)
        .init();
}
