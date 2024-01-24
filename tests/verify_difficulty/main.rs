fn main() {
    env_logger::init();
    println!("Bitcoin_rules! (ver. {})", version());
    println!("A Bitcoin node written in Rust for educational purposes.");
    println!();
    println!("Verifying difficulties.");
}

fn version() -> &'static str {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("unknown")
}
