use brl::{
    flags::network_magic::NetworkMagic,
    network::{command::VERACK_COMMAND, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};

pub fn new(network: NetworkMagic) -> StdResult<NetworkMessage> {
    let payload = Vec::<u8>::new();

    NetworkMessage::new(VERACK_COMMAND, payload, network)
}
