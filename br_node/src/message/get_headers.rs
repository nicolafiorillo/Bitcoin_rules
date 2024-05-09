use brl::{
    flags::network_magic::NetworkMagic,
    hashing::hash256::Hash256,
    network::{command::GET_HEADERS_COMMAND, get_header::GetHeader, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};

// By now the message requests from 'start' to next 2000 headers
pub fn new(start: Hash256, network: NetworkMagic) -> StdResult<NetworkMessage> {
    let get_header_message = GetHeader::new(start, Hash256::zero());
    let payload = get_header_message.serialize();

    NetworkMessage::new(GET_HEADERS_COMMAND, payload, network)
}
