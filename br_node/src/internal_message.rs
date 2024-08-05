use brl::{hashing::hash256::Hash256, network::headers::Headers};

// TODO: the u8 in NodeReady should be the node id: we have to find a way to assign it to message transparently
#[derive(Debug, Clone, PartialEq)]
pub enum InternalMessage {
    NodeIsReady(u8),
    GetHeadersRequest(u8, Hash256),
    GetHeadersResponse(u8, Headers),
}
