use std::fmt::{self, Display, Formatter};

use brl::{
    bitcoin::constants::network_to_environment, flags::network_magic::NetworkMagic, hashing::hash256::Hash256,
    std_lib::std_result::StdResult,
};
use serde_derive::{Deserialize, Serialize};

static CONFIG_FILE: &str = "brn";
static APP_NAME: &str = "bitcoin_rules";

#[derive(Debug, Clone)]
pub struct Environment {
    pub network: NetworkMagic,
    pub remote_node_address: String,
    pub remote_node_port: u16,
    pub genesis_block_hash: Hash256,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct Configuration {
    pub network: String,
    pub remote_node_address: String,
    pub remote_node_port: u16,
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{{ network: {:}, remote_node_address: {:}, remote_node_port: {:} }}",
            self.network, self.remote_node_address, self.remote_node_port
        )
    }
}

pub fn load_config() -> StdResult<Environment> {
    let cfg: Configuration = confy::load(APP_NAME, CONFIG_FILE)?;
    let network: NetworkMagic = cfg.network.into();

    let genesis_block_hash = network_to_environment(network);

    let env = Environment {
        network,
        remote_node_address: cfg.remote_node_address,
        remote_node_port: cfg.remote_node_port,
        genesis_block_hash,
    };

    Ok(env)
}
