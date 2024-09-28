use core::{
    flags::network_magic::NetworkMagic,
    network::{
        command::VERSION_COMMAND, ip_address, network_address::NetworkAddress, network_message::NetworkMessage,
        version::Version,
    },
    std_lib::std_result::StdResult,
};

use crate::utils;

pub fn new(local_address: &str, network: NetworkMagic) -> StdResult<NetworkMessage> {
    let address = ip_address::parse_address(local_address).unwrap();

    let receiver = NetworkAddress::new(0, 0, address, 8333);
    let sender = NetworkAddress::new(0, 0, address, 8333);

    let version_message = Version::new(receiver, sender, 0, &utils::agent());
    let payload = version_message.serialize();

    NetworkMessage::new(VERSION_COMMAND, payload, network)
}
