pub fn version() -> &'static str {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    VERSION.unwrap_or("unknown")
}

pub fn agent() -> String {
    format!("/Bitcoin_rules!:{}/", version())
}
