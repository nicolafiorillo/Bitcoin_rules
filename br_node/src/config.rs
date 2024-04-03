use serde_derive::{Deserialize, Serialize};

static CONFIG_FILE: &str = "brn";
static APP_NAME: &str = "bitcoin_rules";

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub network: String,
    pub remote_node_address: String,
    pub remote_node_port: u16,
}

pub fn load_config() -> Result<Configuration, Box<dyn std::error::Error>> {
    let cfg: Configuration = confy::load(APP_NAME, CONFIG_FILE)?;
    Ok(cfg)
}
