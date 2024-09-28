use core::{
    flags::network_magic::NetworkMagic,
    hashing::hash256::Hash256,
    network::{command::GET_HEADERS_COMMAND, get_header::GetHeader, network_message::NetworkMessage},
    std_lib::std_result::StdResult,
};

// By now the message requests from 'start' to next 2000 headers
pub fn new(start: Hash256) -> GetHeader {
    GetHeader::new(start, Hash256::zero())
}

pub fn as_network_message(get_header: &GetHeader, network: NetworkMagic) -> StdResult<NetworkMessage> {
    let payload = get_header.serialize();
    NetworkMessage::new(GET_HEADERS_COMMAND, payload, network)
}
