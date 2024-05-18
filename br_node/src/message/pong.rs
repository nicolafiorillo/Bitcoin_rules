use brl::{
    flags::network_magic::NetworkMagic,
    network::{command::PONG_COMMAND, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};

pub fn new(nonce: u64, network: NetworkMagic) -> StdResult<NetworkMessage> {
    let payload = nonce.to_le_bytes().to_vec();

    NetworkMessage::new(PONG_COMMAND, payload, network)
}
